pub mod promo_code_handler {
  use std::{fs::File, io::BufReader};
  
  use crate::process_call;
  use process_call::handle_python_call;
  use serde::{Deserialize, Serialize};
  
  #[derive(Serialize, Deserialize)]
  pub struct PromoCodeArgs {
    game: String,
    account: Option<String>,
    password: Option<String>,
    hoyolab_id: Option<String>,
    cookies: Option<String>,
    uid: Option<String>,
    code: Option<String>,
  }
  
  #[tauri::command]
  pub fn redeem_promo_code(args: PromoCodeArgs) -> Result<String, String> {
    let mut python_args = Vec::new();
    
    // Add required game argument
    python_args.push(("game", args.game.as_str()));
    
    // Add optional arguments if present
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
      python_args.push(("cookies", cookies));
    }
    if let Some(uid) = args.uid.as_deref() {
      python_args.push(("uid", uid));
    }
    if let Some(code) = args.code.as_deref() {
      python_args.push(("code", code));
    }
    
    let arg_refs: Vec<(&str, &str)> = python_args.iter().map(|(k, v)| (*k, *v)).collect();
    
    println!("{}, {:?}", "promo_code_redeem", arg_refs);
    
    handle_python_call("promo_code_redeem", arg_refs)
  }
  
  #[tauri::command]
  pub fn run_code_redeem() -> Result<String, String> {
    let task_data = File::open(mihub_lib::get_task_data().unwrap_or_default())
    .map_err(|e| e.to_string())?;
    let reader = BufReader::new(task_data);
    
    let json_data: serde_json::Value =
    serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    
    let code_redeems = json_data["scripts"]["redeem_promo_code"]
    .as_array()
    .ok_or_else(|| "No promo_code_redeem tasks found".to_string())?;
    
    let mut results = Vec::new();
    
    for code_redeem in code_redeems {
      let args: PromoCodeArgs = serde_json::from_value(code_redeem.clone())
      .map_err(|e| format!("Failed to parse code redeem data: {}", e))?;
      
      match redeem_promo_code(args) {
        Ok(result) => results.push(result),
        Err(e) => results.push(format!("Error: {}", e)),
      }
    }
    
    Ok(results.join("\n"))
  }
}
