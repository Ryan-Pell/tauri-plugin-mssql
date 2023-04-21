

use serde::{ser::Serializer, Serialize};
use tauri::utils::config::{self, ClipboardAllowlistConfig};
use tauri::{command, plugin::{Builder, TauriPlugin}, AppHandle, Manager, Runtime, State, Window};
use std::option;
use std::{collections::HashMap};
use serde::Deserialize;
use tiberius::{Client, Config, AuthMethod, SqlBrowser, ExecuteResult, error::Error};
use async_std::{net::TcpStream, sync::Mutex};


#[derive(Default)]
struct DbInstances(Mutex<HashMap<String, Client<TcpStream>>>);

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
    .invoke_handler(tauri::generate_handler![connect, disconnect, is_connection_active])
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


/**PLUGIN COMMANDS */
#[command]
async fn connect<R: Runtime>(_app: AppHandle<R>, conf_instance: State<'_, ConfInstance>, /*db_instance: State<'_, DbInstances>,*/ db: Option<String>) -> Result<(), String> {
  //Check for Open Connection
  if true { return Err("A connection is already active. Disconnect from the current connection then reconnect".to_owned()); }

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



  let tcp: Result<TcpStream, Error> = TcpStream::connect_named(&config).await;
  // check for tcp error
  if tcp.is_err() { return Err("TCP Error".to_owned()); }


  let client = Client::connect(config, tcp.unwrap()).await;

  Ok(())
}

#[command]
async fn disconnect<R: Runtime>(_app: tauri::AppHandle<R>, /*db_instance: State<'_, DbInstances>*/) -> Result<(), String> {
  Ok(())
}

#[command]
async fn is_connection_active<R: Runtime>(_app: tauri::AppHandle<R>, db_instance: State<'_, DbInstances>) -> Result<bool, String> {


  Ok(true)
}