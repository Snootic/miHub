pub mod checkin_handler {
  use std::{fs::File, io::BufReader};
  
  use crate::process_call;
  use process_call::handle_python_call;
  use serde::{Deserialize, Serialize};
  
  #[derive(Serialize, Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct CheckinArgs {
    game: String,
    account: Option<String>,
    password: Option<String>,
    hoyolab_id: Option<String>,
    cookies: Option<String>,
  }
  
  #[tauri::command]
  pub fn daily_checkin(args: CheckinArgs) -> Result<String, String> {
    let mut python_args = Vec::new();
    
    python_args.push(("game", args.game.as_str()));
    
    if let Some(account) = args.account.as_deref() {
      python_args.push(("account", account));
    }
    if let Some(password) = args.password.as_deref() {
      python_args.push(("password", password));
    }
    if let Some(hoyolab_id) = args.hoyolab_id.as_deref() {
      python_args.push(("hoyolab-id", hoyolab_id));
    }
    if let Some(cookies) = args.cookies.as_deref() {
      python_args.push(("cookies", cookies)); // Make sure to use &str here
    }
    
    let arg_refs: Vec<(&str, &str)> = python_args.iter().map(|(k, v)| (*k, *v)).collect();
    
    handle_python_call("daily_checkin", arg_refs)
  }
  
  #[tauri::command]
  pub fn run_checkin() -> Result<String, String> {
    // Lê o arquivo data.json
    let task_data = File::open(mihub_lib::get_task_data().unwrap_or_default())
    .map_err(|e| e.to_string())?;
    let reader = BufReader::new(task_data);
    
    // Parse o JSON
    let json_data: serde_json::Value =
    serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    
    // Verifica se existe a estrutura scripts.daily_checkin
    let checkins = json_data["scripts"]["daily_checkin"]
    .as_array()
    .ok_or_else(|| "No daily_checkin tasks found".to_string())?;
    
    let mut results = Vec::new();
    
    // Processa cada objeto daily_checkin
    for checkin in checkins {
      // Converte o objeto JSON em CheckinArgs
      let args: CheckinArgs = serde_json::from_value(checkin.clone())
      .map_err(|e| format!("Failed to parse checkin data: {}", e))?;
      
      // Executa o daily_checkin para este conjunto de argumentos
      match daily_checkin(args) {
        Ok(result) => results.push(result),
        Err(e) => results.push(format!("Error: {}", e)),
      }
    }
    
    // Retorna todos os resultados concatenados
    Ok(results.join("\n"))
  }
}
