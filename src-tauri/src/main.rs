// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod system_tasks;
mod checkin;
mod process_call;
mod promo_code;
mod handlers;
use mihub_lib;
use tauri::{
  menu::{Menu, MenuItem},
  tray::TrayIconBuilder,
};

use serde_json::json;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::{env, sync::Mutex};
use tauri::Manager;

use checkin::checkin_handler;
use promo_code::promo_code_handler;

#[tauri::command]
fn save_data(app: tauri::AppHandle, mode: String, data: Value) -> Result<(), String> {
  let data_dir = app.path().app_data_dir().unwrap();
  let file_path = data_dir.join("data.json").to_string_lossy().to_string();
  
  // Tenta abrir e ler o arquivo existente
  let mut existing_data = String::new();
  if let Ok(mut file) = OpenOptions::new().read(true).open(file_path.clone()) {
    file.read_to_string(&mut existing_data)
    .map_err(|e| e.to_string())?;
  }
  
  // Tenta parsear o JSON existente, ou cria um novo se falhar
  let mut json_data: Value =
  serde_json::from_str(&existing_data).unwrap_or_else(|_| json!({ "scripts": [] }));
  
  // Garante que a chave do modo existe e é um array
  if !json_data["scripts"].is_object() {
    json_data["scripts"] = json!({});
  }
  
  if !json_data["scripts"]
  .get(mode.clone())
  .map_or(false, |v| v.is_array())
  {
    json_data["scripts"][mode.clone()] = json!([]);
  }
  
  if let Some(entries) = json_data["scripts"][mode].as_array_mut() {
    entries.push(data);
  }
  
  // Escreve o JSON atualizado no arquivo
  let mut file = OpenOptions::new()
  .write(true)
  .create(true)
  .truncate(true)
  .open(file_path)
  .map_err(|e| e.to_string())?;
  file.write_all(json_data.to_string().as_bytes())
  .map_err(|e| e.to_string())?;
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
  tauri::Builder::default()
  .plugin(tauri_plugin_dialog::init())
  .plugin(tauri_plugin_updater::Builder::default().build())
  .manage(Mutex::new(mihub_lib::SideTasks {
    updater: false,
    dependencies: false,
  }))
  .setup(|app| {
    let handle = app.handle().clone();
    let window = app.get_webview_window("main").unwrap();
    window.on_window_event(move |event| {
      if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        let main_window = &handle.get_webview_window("main").unwrap();
        main_window.hide().unwrap();
      }
    });
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i, &show_i])?;
    let _tray = TrayIconBuilder::new()
    .icon(app.default_window_icon().unwrap().clone())
    .menu(&menu)
    .show_menu_on_left_click(true)
    .on_menu_event(|app, event| match event.id.as_ref() {
      "quit" => {
        println!("quit menu item was clicked");
        app.exit(0);
      }
      "show" => {
        println!("show menu item was clicked");
        let window = app.get_webview_window("main").unwrap();
        if window.is_visible().unwrap() {
          window.hide().unwrap();
        } else {
          window.show().unwrap();
          window.set_focus().unwrap();
        }
      }
      _ => {
        println!("menu item {:?} not handled", event.id);
      }
    })
    .build(app)?;
    let data_dir = app.path().app_data_dir().unwrap();
    let initial_config_json_path = data_dir.join("windows_initial_config.json");
    
    if cfg!(target_os = "windows") {
      let initial_config_content =
      match std::fs::read_to_string(&initial_config_json_path) {
        Ok(content) => content,
        Err(e) => {
          eprintln!("Error reading windows initial config: {}", e);
          String::new()
        }
      };
      let initial_config_json: Value = match serde_json::from_str(&initial_config_content)
      {
        Ok(json) => json,
        Err(e) => {
          eprintln!("Error parsing windows initial config: {}", e);
          Value::Object(serde_json::Map::new())
        }
      };
      
      if !initial_config_json_path.exists()
      || initial_config_json["initial_config"] == true
      {
        println!("Running initial config for windows");
        let _ = mihub_lib::windows_initial_config(app);
        return Ok(());
      }
    }
    
    let libs_binding = cfg!(target_os = "windows")
    .then(|| data_dir.clone())
    .unwrap_or(data_dir.join("Python311"));
    let lib_path = libs_binding.to_str().unwrap();
    
    let python_path_binding = cfg!(target_os = "windows")
    .then(|| libs_binding.join("Python311"))
    .unwrap_or(libs_binding.join("python3.11"));
    let python_path = python_path_binding.to_str().unwrap();
    
    let sys_path = env::var("PATH").unwrap_or_default();
    
    let bin_binding = cfg!(target_os = "windows")
    .then(|| libs_binding.join("Python311\\Scripts"))
    .unwrap_or(libs_binding.join("bin"));
    let bin_path = bin_binding.to_str().unwrap();
    
    let path = cfg!(target_os = "windows")
    .then(|| format!("{};{};{}", sys_path, lib_path, bin_path))
    .unwrap_or(format!("{}:{}:{}", sys_path, lib_path, bin_path));
    
    env::set_var("PATH", path);
    env::set_var(
      "PYTHONPATH",
      python_path_binding.join("Python311.zip").to_str().unwrap(),
    );
    env::set_var("PYTHONUSERBASE", lib_path);
    
    if cfg!(target_os = "windows") {
      env::set_var("PYTHONHOME", python_path);
    }
    
    mihub_lib::initialize_modules(&app);
    
    let _ = process_call::set_sys_path(libs_binding);

    mihub_lib::handle_dependencies(app);
    
    Ok(())
  })
  .invoke_handler(tauri::generate_handler![
    checkin_handler::daily_checkin,
    checkin_handler::run_checkin,
    promo_code_handler::redeem_promo_code,
    promo_code_handler::run_code_redeem,
    handlers::updater::check_for_updates,
    handlers::updater::start_update,
    save_data
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  }
  