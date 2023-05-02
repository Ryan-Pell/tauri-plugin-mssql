/**
 *
 * @param {string} query This is the query that the server will run and return the data.
 * @param {string} connection This is a optional connection string that you can use to connect to another database that is not the default. The default is the one that is defined within the Tauri setup. The string should be used as as a JDBC string (starting with jdbc:sqlserver://...), or a .NET string (e.g, Data Source=myServerName\myInstance;Database=myDatabase;Integrated Security=false;User ID=sa;Password=<password>;Encrypt=true;TrustServerCertificate=true;).
 * @returns {Promise<{raw: string, json: void}>} This will return a object with the raw string from the database query and a json function to get a parsed version of the query.
 */
export declare function query(query: string, connection?: string): Promise<{
    raw: string;
    json: () => any;
}>;
export declare function defaultConnectionString(): Promise<void>;
