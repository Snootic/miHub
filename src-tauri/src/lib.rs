use std::{
  collections::HashMap,
  path::PathBuf,
  process::Stdio,
  sync::{Mutex, Once},
};

use tauri::{path::BaseDirectory, App, Manager, State};
use tokio::io::{AsyncBufReadExt, BufReader};

static INIT: Once = Once::new();
use std::sync::OnceLock;
static DAILY_CHECKIN: OnceLock<String> = OnceLock::new();
static PROMO_CODE: OnceLock<String> = OnceLock::new();
static TASK_DATA: OnceLock<String> = OnceLock::new();

pub fn initialize_modules(app: &App) {
  INIT.call_once(|| {
    let binding = app
      .path()
      .resolve(
        "src/hoyolab/auto_start/daily_checkin.py",
        BaseDirectory::Resource,
      )
      .unwrap();
    DAILY_CHECKIN.set(binding.to_str().unwrap().to_string()).ok();

    let binding = app
      .path()
      .resolve(
        "src/hoyolab/auto_start/promo_code_redeem.py",
        BaseDirectory::Resource,
      )
      .unwrap();
    PROMO_CODE.set(binding.to_str().unwrap().to_string()).ok();

    let binding = app
      .path()
      .resolve("data.json", BaseDirectory::AppData)
      .unwrap();
    TASK_DATA.set(binding.to_str().unwrap().to_string()).ok();
  });
}

pub fn get_checkin() -> Option<&'static str> {
  DAILY_CHECKIN.get().map(|s| s.as_str())
}

pub fn get_promo_code() -> Option<&'static str> {
  PROMO_CODE.get().map(|s| s.as_str())
}
pub fn get_task_data() -> Option<&'static str> {
  TASK_DATA.get().map(|s| s.as_str())
}

pub struct SideTasks {
  pub updater: bool,
  pub dependencies: bool,
}

// the following code was just copied from the ai-translator project
#[tauri::command]
async fn set_complete(
  app: tauri::AppHandle,
  state: State<'_, Mutex<SideTasks>>,
  task: String,
) -> Result<(), ()> {
  let mut state = state.lock().unwrap();
  
  match task.as_str() {
    "updater" => state.updater = true,
    "dependencies" => state.dependencies = true,
    _ => (),
  }
  
  if state.dependencies {
    let main_window = app.get_webview_window("main").unwrap();
    main_window.show().unwrap();
  }
  
  Ok(())
}

pub fn handle_dependencies(app: &App) {
  let binding = app.path().app_data_dir().unwrap();
  println!("installing dependencies");
  
  let deps_json_path = binding.join("installed_dependencies.json");
  
  let win_python = app
  .path()
  .app_data_dir()
  .unwrap()
  .join("Python311")
  .join("python.exe");
  
  let python_executable = cfg!(target_os = "windows")
  .then(|| win_python.to_str().unwrap().to_string())
  .unwrap_or("python3".to_string());
  
  // let python_executable = app.path().resolve(python, BaseDirectory::Resource).unwrap();
  
  let temp_dir = app.path().temp_dir().unwrap();
  
  let requirements_url = "https://gist.githubusercontent.com/Snootic/ab325894aa85e8184714dea8a9a34925/raw/07f75e6c6dad064a9b6e61daa5cabce2cfab94d9/requirements.txt";
  
  let app_handle = app.handle().clone();
  tauri::async_runtime::spawn(async move {
    let requirements_response = reqwest::get(requirements_url)
    .await
    .map_err(|_| ())
    .unwrap()
    .text()
    .await
    .map_err(|_| ())
    .unwrap();
    
    let splited_requirements: Vec<&str> = requirements_response.split("\n").collect();
    let requirements: std::collections::HashMap<String, String> = splited_requirements
    .iter()
    .filter(|&r| !r.is_empty())
    .map(|r| {
      let parts: Vec<&str> = r.split("==").collect();
      (parts[0].to_string(), parts[1].to_string())
    })
    .collect();
    
    for (package, version) in requirements.iter() {
      let installed_dependencies_json =
      std::fs::read_to_string(deps_json_path.to_str().unwrap()).unwrap_or_default();
      
      let installed_dependencies: HashMap<String, String> =
      serde_json::from_str(&installed_dependencies_json).unwrap_or_default();
      
      if !installed_dependencies.contains_key(package)
      || installed_dependencies.get(package).unwrap() != version
      {
        let _ = install_dependencies(
          &requirements_url,
          python_executable.clone(),
          temp_dir.clone(),
          deps_json_path.clone(),
          requirements.clone(),
        )
        .await;
        
        println!("Restarting app");
        app_handle.restart();
      }
    }
    let _ = set_complete(
      app_handle.clone(),
      app_handle.state::<Mutex<SideTasks>>(),
      "dependencies".to_string(),
    )
    .await;
  });
}

async fn install_dependencies(
  requirements_url: &str,
  python_executable: String,
  temp_dir: PathBuf,
  deps_json_path: PathBuf,
  dependencies: HashMap<String, String>,
) -> Result<(), ()> {
  let get_pip = temp_dir.join("get-pip.py");
  
  if !get_pip.exists() {
    let get_pip_url = "https://bootstrap.pypa.io/get-pip.py";
    
    let get_pip_request = reqwest::get(get_pip_url)
    .await
    .map_err(|_| ())
    .unwrap()
    .bytes()
    .await
    .map_err(|_| ())
    .unwrap();
    
    std::fs::write(&get_pip, get_pip_request).expect("Failed to write get-pip file");
  }
  
  let mut install_pip = tokio::process::Command::new(python_executable.clone())
  .args(&[
    get_pip.to_str().unwrap(),
    "--user",
    "--break-system-packages",
  ])
  .stdout(Stdio::piped())
  .spawn()
  .expect("failed to spawn command");
    
  let stdout = install_pip.stdout.take().expect("Failed to get stdout");
  let mut stdout_reader = BufReader::new(stdout).lines();
  
  while let Some(line) = stdout_reader.next_line().await.unwrap() {
    println!("{}", line);
  }
  
  let _ = install_pip
  .wait()
  .await
  .expect("child process encountered an error");
  
  let mut cmd = tokio::process::Command::new(python_executable.clone());
  
  cmd.args(&[
    "-m",
    "pip",
    "install",
    "-r",
    requirements_url,
    "--user",
    "--break-system-packages",
  ]);
  cmd.stdout(Stdio::piped());
  
  let mut child = cmd.spawn().expect("failed to spawn command");
  let stdout = child.stdout.take().expect("Failed to get stdout");
  let mut stdout_reader = BufReader::new(stdout).lines();
  
  while let Some(line) = stdout_reader.next_line().await.unwrap() {
    println!("{}", line);
  }
  
  let status = child
  .wait()
  .await
  .expect("child process encountered an error");
  
  if status.success() {
    println!("Dependencies installed successfully");
    let json = serde_json::to_string(&dependencies).unwrap();
    std::fs::write(deps_json_path, json).expect("Failed to write dependencies file");
    Ok(())
  } else {
    Err(())
  }
}

fn unzip_win_python_package(temp_dir: PathBuf, data_dir: PathBuf) {
  fn fake_callback() {
    println!("called");
  }
  
  let url = "https://www.python.org/ftp/python/3.11.9/python-3.11.9-embed-amd64.zip";
  let unzip_engine = ripunzip::UnzipEngine::for_uri(url, None, fake_callback);
  let reporter = Box::new(ripunzip::NullProgressReporter);
  
  let options = ripunzip::UnzipOptions {
    output_directory: Some(temp_dir.join("python")),
    password: None,
    single_threaded: false,
    filename_filter: None,
    progress_reporter: reporter,
  };
  
  unzip_engine.unwrap().unzip(options).unwrap();
  
  let _ = std::fs::create_dir_all(data_dir.join("Python311"));
  
  for file in std::fs::read_dir(temp_dir.join("python")).unwrap() {
    let file = file.unwrap();
    let _ = std::fs::copy(
      file.path(),
      data_dir.join("Python311").join(file.file_name()),
    );
  }
}

fn fix_win_pth(data_dir: PathBuf) {
  let pth = data_dir.join("Python311\\python311._pth");
  let mut pth_content = std::fs::read_to_string(&pth).unwrap();
  pth_content = pth_content.replace("#import site", "import site");
  std::fs::write(pth, pth_content).unwrap();
}

pub fn windows_initial_config(app: &App) -> Result<(), ()> {
  let handle = app.handle().clone();
  let temp_dir = app.path().temp_dir().unwrap();
  let data_dir = app.path().app_data_dir().unwrap();
  
  let _ = tauri::async_runtime::spawn_blocking(move || {
    if !data_dir.exists() {
      std::fs::create_dir_all(data_dir.clone()).unwrap();
    }
    
    unzip_win_python_package(temp_dir.clone(), data_dir.clone());
    fix_win_pth(data_dir.clone());
    
    let windows_initial_config_json = serde_json::json!({
      "initial_config": false
    });
    
    let windows_initial_config_path = data_dir.join("windows_initial_config.json");
    
    std::fs::write(
      windows_initial_config_path,
      windows_initial_config_json.to_string(),
    )
    .unwrap();
    
    handle.restart();
});
  Ok(())
}

