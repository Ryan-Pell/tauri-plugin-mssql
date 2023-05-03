import { invoke } from '@tauri-apps/api/tauri'

/**
 * 
 * @param {string} connection This is a optional connection string that you can use to connect to another database that is not the default. The default is the one that is defined within the Tauri setup. The string should be used as as a JDBC string (starting with jdbc:sqlserver://...), or a .NET string (e.g, Data Source=myServerName\myInstance;Database=myDatabase;Integrated Security=false;User ID=sa;Password=<password>;Encrypt=true;TrustServerCertificate=true;). 
 * @returns This will return nothing if successfully connected. If there is an error the error will be returned.
 */
export function connect (connection?: string) {
  return new Promise<boolean>((resolve, reject) => {
    let options: { db?: string} = {}
    if(connection){ options = { ...options, db: connection } }

    invoke('plugin:mssql|connect', options)
      .then(() => resolve(true))
      .catch(error => reject(JSON.parse(error)))
  })
}

/**
 * 
 * @returns This will return nothing if successfully disconnected. If an error occurs the error will be returned.
 */
export function disconnect () {
  return new Promise<boolean>((resolve, reject) => {
    invoke('plugin:mssql|disconnect')
      .then(() => resolve(true))
      .catch(error => reject(JSON.parse(error)))
  })
}

type Results = {
  recordsets: Array<RecordSet<any>>
}

export interface RecordSet<T> extends Array<T>  {}

/**
 * 
 * @param {string} query This is the query that the server will run and return the data.
 * @returns {Promise<{raw: string, json: void}>} This will return a object with the raw string from the database query and a json function to get a parsed version of the query.
 */
export function query (query: string, connection?: string) {
  return new Promise<{raw: string, json: () => Results}>((resolve, reject) => {
    let options = {tsql: query} as { tsql: string, connection?: string}
    if(connection){ options = { ...options, connection } } //add connection string to options

    invoke('plugin:mssql|query', options)
      .then((query: any) => {
        resolve({
          raw: query,
          json: () => JSON.parse(query.toString())
        })
      })
      .catch(err => reject({ error: err, query }))
  })  
}

export interface SqlConnection {
  applicationName: string | null,
  database: string | null,
  host: string| null,
  instanceName: string | null,
  user: string | null
}

/**
 * 
 * @returns {Promise<SqlConnection>} This will return the default connection string provided in the Tauri Rust setup. 
 */
export function defaultConnectionString () {
  return new Promise<SqlConnection>((resolve, reject) => {
    invoke('plugin:mssql|default_config')
      .then((config: any) => resolve(JSON.parse(config)))
      .catch(err => reject(err))
  })
}

/**
 * 
 * @returns The status of the current connection. This will currently NOT return the active server that is connected.
 */
export function connectionStatus () {
  return new Promise<boolean>((resolve) => {
    invoke('plugin:mssql|connection_status')
      .then((status: any) => resolve(String(status).toLowerCase() === 'true'))
  })
}