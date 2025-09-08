use tauri_plugin_updater::UpdaterExt;
use tauri::{Emitter};
use serde::Serialize;

#[derive(Clone, Serialize)]
struct AppUpdate {
  total_size: Option<u64>,
  downloaded_size: usize,
  version: String,
}

#[tauri::command]
pub async fn check_for_updates(app: tauri::AppHandle) -> Result<bool, String> {
  match app.updater() {
    Ok(updater) => {
      match updater.check().await {
        Ok(Some(_update)) => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => Err(format!("Failed to check for updates: {}", e)),
      }
    }
    Err(e) => Err(format!("Updater not available: {}", e)),
  }
}

#[tauri::command]
pub async fn start_update(app: tauri::AppHandle) -> Result<(), String> {
  match update(app).await {
    Ok(_) => Ok(()),
    Err(e) => Err(format!("Update failed: {}", e)),
  }
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    let mut downloaded = 0;
    
    update
    .download_and_install(
      |chunk_length, content_length| {
        downloaded += chunk_length;
        println!("downloaded {downloaded} from {content_length:?}");
        app.emit(
          "update-progress",
          AppUpdate {
            total_size: content_length,
            downloaded_size: downloaded,
            version: update.version.clone(),
          },
        )
        .unwrap();
      },
      || {
        println!("download finished");
        let _ = app.emit("update-complete", ());
      },
    )
    .await?;
    
    println!("update installed");
    app.restart();
  } else {
    println!("no update available");
    let _ = app.emit("no-update-available", ());
  }
  
  Ok(())
}

