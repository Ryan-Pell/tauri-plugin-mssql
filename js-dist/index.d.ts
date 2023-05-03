/**
 *
 * @param {string} connection This is a optional connection string that you can use to connect to another database that is not the default. The default is the one that is defined within the Tauri setup. The string should be used as as a JDBC string (starting with jdbc:sqlserver://...), or a .NET string (e.g, Data Source=myServerName\myInstance;Database=myDatabase;Integrated Security=false;User ID=sa;Password=<password>;Encrypt=true;TrustServerCertificate=true;).
 * @returns This will return nothing if successfully connected. If there is an error the error will be returned.
 */
export declare function connect(connection: string): Promise<unknown>;
/**
 *
 * @returns This will return nothing if successfully disconnected. If an error occurs the error will be returned.
 */
export declare function disconnect(): Promise<unknown>;
declare type Results = {
    recordsets: Array<RecordSet<any>>;
};
export interface RecordSet<T> extends Array<T> {
}
/**
 *
 * @param {string} query This is the query that the server will run and return the data.
 * @returns {Promise<{raw: string, json: void}>} This will return a object with the raw string from the database query and a json function to get a parsed version of the query.
 */
export declare function query(query: string, connection?: string): Promise<{
    raw: string;
    json: () => Results;
}>;
export interface SqlConnection {
    applicationName: string | null;
    database: string | null;
    host: string | null;
    instanceName: string | null;
    user: string | null;
}
/**
 *
 * @returns {Promise<SqlConnection>} This will return the default connection string provided in the Tauri Rust setup.
 */
export declare function defaultConnectionString(): Promise<SqlConnection>;
/**
 *
 * @returns The status of the current connection. This will currently NOT return the active server that is connected.
 */
export declare function connectionStatus(): Promise<boolean>;
export {};
