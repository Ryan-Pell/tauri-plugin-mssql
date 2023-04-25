import { invoke } from '@tauri-apps/api/tauri'

/**
 * 
 * @param {string} query This is the query that the server will run and return the data.
 * @param {string} connection This is a optional connection string that you can use to connect to another database that is not the default. The default is the one that is defined within the Tauri setup. The string should be used as as a JDBC string (starting with jdbc:sqlserver://...), or a .NET string (e.g, Data Source=myServerName\myInstance;Database=myDatabase;Integrated Security=false;User ID=sa;Password=<password>;Encrypt=true;TrustServerCertificate=true;). 
 * @returns {Promise<{raw: string, json: void}>} This will return a object with the raw string from the database query and a json function to get a parsed version of the query.
 */
export async function query (query: string, connection?: string) {
  let options = {tsql: query} as { tsql: string, connection?: string}

  if(connection){ options = { ...options, connection } } //add connection string to options

  let returns: string = await invoke('plugin:mssql|query', options);
  return {
    raw: returns,
    json: () => JSON.parse(returns)
  }
}
