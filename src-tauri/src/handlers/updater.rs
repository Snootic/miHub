use tauri_plugin_updater::UpdaterExt;
use tauri::{Emitter};
use serde::Serialize;

#[derive(Clone, Serialize)]
struct AppUpdate {
  total_size: Option<u64>,
  downloaded_size: usize,
  version: String,
}

pub async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
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
      },
    )
    .await?;
    
    println!("update installed");
    app.restart();
  } else {
    println!("no update available");
    
    Ok(())
  }
}

