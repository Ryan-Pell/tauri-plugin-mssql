use serde_json::json;
use tauri::{command, plugin::{Builder, TauriPlugin}, Manager, Runtime, State};
use std::{collections::HashMap};
use tiberius::{Client, Config, AuthMethod, SqlBrowser, error::Error, ColumnData};
use async_std::{net::TcpStream, sync::Mutex};
mod errors;

#[derive(Default, Debug)]
struct BrowserInstance(Mutex<HashMap<String, Client<TcpStream>>>);


/**SQL CONFIGURATION INIT INPUT */
#[derive(Default, Debug)]
struct ConfInstance(SqlConfig);

#[derive(Clone, Debug, Default)]
pub struct SqlConfig {
  application_name: Option<String>,
  host: String,
  instance_name: String,
  database: Option<String>,
  auth: Auth
}

#[derive(Clone, Debug, Default)]
pub struct Auth {
  username: String,
  password: String
}

impl SqlConfig {
  pub fn new() -> Self { Self {
    application_name: None,
    host: "".to_owned(), 
    instance_name: "".to_owned(), 
    database: None, 
    auth: Auth { username: "".to_owned(), password: "".to_owned() }
  }}

  pub fn application_name(&mut self, app_name: impl ToString) { self.application_name = Some(app_name.to_string()); }
  pub fn host(&mut self, host: impl ToString) { self.host = host.to_string(); }
  pub fn instance_name(&mut self, instance_name: impl ToString) { self.instance_name = instance_name.to_string(); }
  pub fn database(&mut self, database: impl ToString) { self.database = Some(database.to_string()); }
  pub fn auth(&mut self, usr: impl ToString, pwd: impl ToString) { self.auth = Auth { username: usr.to_string(), password: pwd.to_string() } }

  pub fn get_tiberius_config(&self) -> Config {
    let settings = self.clone();
    let mut config = Config::new();

    config.host(settings.host);
    config.instance_name(settings.instance_name);
    config.authentication(AuthMethod::sql_server(settings.auth.username, settings.auth.password));
    config.trust_cert();

    if settings.application_name.is_some() { config.application_name( String::from(settings.application_name.unwrap())); }
    if settings.database.is_some() { config.database( String::from(settings.database.unwrap()) ); }

    config
  }
}
/**************/


/**INITIALISING THE PLUGIN */
pub fn init<R: Runtime>(init_config: SqlConfig) -> TauriPlugin<R> {
  Builder::new("mssql")
    .invoke_handler(tauri::generate_handler![connect, disconnect, query, default_config])
    .setup(|app| {
      let mut config = ConfInstance::default();
      config.0 = init_config;
      app.manage(config);

      let mut browser = BrowserInstance::default();
      app.manage(browser);

      Ok(())
    })
    .build()
}
/**************/


/**PLUGIN COMMANDS */
#[tauri::command]
async fn connect<R: Runtime>(_app: tauri::AppHandle<R>, conf_instance: State<'_, ConfInstance>, db: Option<String>, browser_instance: State<'_, BrowserInstance>) -> Result<(), String> {
  //Set Connection String
  let config: Config;
  if db.is_some() {
    let db: String = db.clone().unwrap();

    let ext_config: Result<Config, Error>;
    if db.starts_with("jdbc:sqlserver://") { ext_config = Config::from_jdbc_string(&db); }
    else { ext_config = Config::from_ado_string(&db); }

    //check for errors on external;
    if ext_config.is_err() { return Err("Invalid Input String. This should be formatted to be a a valid JBDC connection string or .NET connection string.".to_owned()); }
    else { config = ext_config.unwrap(); }
  }
  else { config = conf_instance.0.get_tiberius_config().clone(); }

  //connect to tcp
  let tcp: Result<TcpStream, Error> = TcpStream::connect_named(&config).await;
  if tcp.is_err() { return Err("TCP Error".to_owned()); }

  //connect to sql browser
  match Client::connect(config, tcp.unwrap()).await {
    Ok(client) => {
      let mut browser = browser_instance.0.lock().await;
      browser.insert("client".to_string(), client);
      Ok(())
    },
    Err(error) => Err(error.to_string())
  }
}

#[tauri::command]
async fn disconnect<R: Runtime>(_app: tauri::AppHandle<R>, browser_instance: State<'_, BrowserInstance>) -> Result<(), String> {
  let mut browser_map = browser_instance.0.lock().await;
  match browser_map.get_mut("client") {
    Some(browser) => {
      browser_map.remove("client");
      Ok(())
    },
    None => Err(errors::no_active_connection(None))
  }
}

#[tauri::command]
async fn query<R: Runtime>(_app: tauri::AppHandle<R>, browser_instance: State<'_, BrowserInstance>, tsql: Option<String>) -> Result<String, String> {

  //Check for Active Connection
  let mut browser_map = browser_instance.0.lock().await;
  if !browser_map.contains_key("client"){ return Err(errors::no_active_connection(None));}
  let browser = browser_map.get_mut("client").unwrap();

  //Check that TSQL query is present
  if tsql.is_none(){ return Err(errors::general("Requires TSQL to be set to run query on database")); }

  //run query
  let query = browser.simple_query(tsql.unwrap()).await;

  match query {
    Ok(qs) => {
      //get results
      let results = qs.into_results().await;
      if results.is_err() { return Err(results.err().unwrap().to_string()); }
      
      /***Convert Results into JSON */
      let result_sets = results.unwrap();
      let mut record_sets: Vec<Vec<HashMap<String, String>>> = Vec::new();

      for result_set in result_sets {
        let mut record_set: Vec<HashMap<String, String>> = Vec::new();
        
        //get rows
        for row in result_set {
          let columns: Vec<String> = row.columns().iter().map(|f| f.name().to_string()).collect();
          let mut output: HashMap<String, String> = HashMap::new();

          /***sql into strings */
          for (id, item) in row.into_iter().enumerate() {
            let value: String = match item {
              ColumnData::Binary(_) => "<binary data>".into(),
              ColumnData::Bit(val) => val.unwrap_or_default().to_string(),
              ColumnData::String(val) => val.unwrap_or_default().to_string(),
              ColumnData::I16(val) => val.unwrap_or_default().to_string(),
              ColumnData::I32(val) => val.unwrap_or_default().to_string(),
              ColumnData::I64(val) => val.unwrap_or_default().to_string(),
              ColumnData::Numeric(val) => val.unwrap().to_string(),
              ColumnData::F32(val) => val.unwrap_or_default().to_string(),
              ColumnData::F64(val) => val.unwrap_or_default().to_string(),
              _ => "nada".into()
            };

            output.insert(columns[id].clone(), value); //output to hashmap
          }
          record_set.push(output); //push output to record set
        }
        record_sets.push(record_set); //push record_set to records_sets
      }

      return Ok(json!({"recordsets": record_sets}).to_string());
    },
    Err(error) => { return Err(error.to_string()); } //return error if problem with query
  }
}

#[command]
async fn default_config<R: Runtime>(_app: tauri::AppHandle<R>, conf_instance: State<'_, ConfInstance>) -> Result<String, String> {
  let conf = &conf_instance.0;

  Ok(json!({
    "host": conf.host,
    "database": conf.database,
    "instanceName": conf.instance_name,
    "applicationName": conf.application_name,
    "user": conf.auth.username,
  }).to_string())
}