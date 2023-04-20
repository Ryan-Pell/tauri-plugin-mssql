/**
 *
 * @param connectionString Server=myServerName\myInstanceName;Database=myDataBase;User Id=myUsername;Password=myPassword;Trust_Connection:True;
 */
export declare function connect(connectionString?: string): Promise<void>;
