import { invoke } from '@tauri-apps/api/tauri'

/**
 * 
 * @param connectionString Data Source=AB-SQL02\Timesheet;Database=Intranet;Integrated Security=false;User ID=sa;Password=<password>;Encrypt=true;TrustServerCertificate=true;
 */
export async function connect(connectionString?: string) {
  connectionString = "Data Source=AB-SQL02\Timesheet;Database=Intranet;Integrated Security=false;User ID=sa;Password=<password>;Encrypt=true;TrustServerCertificate=true;";
  console.log(connectionString);
  await invoke('plugin:mssql|connect', {db: connectionString})
}
