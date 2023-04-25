

use chrono::{NaiveDateTime, DateTime, Duration, NaiveTime, Utc};
use serde::{ser::Serializer, Serialize};
use serde_json::{value, json};
use tauri::utils::config::{self, ClipboardAllowlistConfig};
use tauri::{command, plugin::{Builder, TauriPlugin}, AppHandle, Manager, Runtime, State, Window};
use tiberius::{ColumnData, FromSqlOwned};
use std::str::FromStr;
use std::{collections::HashMap};
use serde::Deserialize;
use tiberius::{Client, Config, AuthMethod, SqlBrowser, error::Error, FromSql, Row};
use async_std::{net::TcpStream};



/**SQL CONFIGURATION INIT INPUT */
#[derive(Default, Debug)]
struct ConfInstance(Config);

pub struct SqlConfig {
  application_name: Option<String>,
  host: String,
  instance_name: String,
  database: Option<String>,
  auth: Auth
}

struct Auth {
  username: String,
  password: String
}

impl SqlConfig {
  pub fn new() -> Self { Self {
    application_name: None,
    host: "".to_owned(), 
    instance_name: "".to_owned(), 
    database: None, auth: 
    Auth { username: "".to_owned(), password: "".to_owned() }
  }}

  pub fn application_name(&mut self, app_name: impl ToString) { self.application_name = Some(app_name.to_string()); }
  pub fn host(&mut self, host: impl ToString) { self.host = host.to_string(); }
  pub fn instance_name(&mut self, instance_name: impl ToString) { self.instance_name = instance_name.to_string(); }
  pub fn database(&mut self, database: impl ToString) { self.database = Some(database.to_string()); }
  pub fn auth(&mut self, usr: impl ToString, pwd: impl ToString) { self.auth = Auth { username: usr.to_string(), password: pwd.to_string() } }
}
/**************/


/**INITIALISING THE PLUGIN */
pub fn init<R: Runtime>(init_config: SqlConfig) -> TauriPlugin<R> {
  Builder::new("mssql")
    .invoke_handler(tauri::generate_handler![query])
    .setup(|app| {
      let mut config = ConfInstance::default();

      config.0.host(init_config.host);
      config.0.instance_name(init_config.instance_name);
      config.0.authentication(AuthMethod::sql_server(init_config.auth.username, init_config.auth.password));
      config.0.trust_cert();

      if init_config.application_name.is_some() { config.0.application_name(init_config.application_name.unwrap().to_string()); }
      if init_config.database.is_some() { config.0.database(init_config.database.unwrap().to_string()); }

      app.manage(config);

      Ok(())
    })
    .build()
}
/**************/


/**RUST COMMANDS */
async fn connect(conf_instance: State<'_, ConfInstance>, db: Option<String>) -> Result<Client<TcpStream>, String> {
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
  else { config = conf_instance.0.clone(); }

  //connect to tcp
  let tcp: Result<TcpStream, Error> = TcpStream::connect_named(&config).await;
  if tcp.is_err() { return Err("TCP Error".to_owned()); }

  //connect to sql browser
  match Client::connect(config, tcp.unwrap()).await {
    Ok(c) => Ok(c),
    Err(error) => Err(error.to_string())
  }
}

/**PLUGIN COMMANDS */
#[command]
async fn query<R: Runtime>(_app: tauri::AppHandle<R>, conf_instance: State<'_, ConfInstance>, db: Option<String>, tsql: Option<String>) -> Result<String, String> {
  if tsql.is_none(){ return Err("Requires TSQL to be set to run query on database.".into()); }

  match connect(conf_instance, db).await {
    Ok(mut client) => {
      //run query
      let query = client.simple_query(tsql.unwrap()).await;

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
      
    },
    Err(error) => { return Err(error.into()); }
  }
}