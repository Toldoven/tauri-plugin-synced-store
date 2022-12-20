#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Serialize, Deserialize};

use tauri_plugin_synced_state::{SyncState, SyncStateToml, PluginBuilder};

use ts_rs::TS;


#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
struct Count {
    count: i32,
}

impl Default for Count {
    fn default() -> Self {
        Self {
            count: 0
        }
    }
}

#[tauri::command]
async fn get_count(count: SyncState<'_, Count>) -> Result<Count, ()> {
    Ok(count.get().await)
}

#[tauri::command]
async fn get_count_toml(count: SyncStateToml<'_, Count>) -> Result<Count, ()> {
    Ok(count.get().await)
}

#[tauri::command]
async fn plus_count(count: SyncState<'_, Count>) -> Result<(), ()> {
    
    count.mutate(|count| count.count += 1 ).await;

    Ok(())
}

#[tauri::command]
async fn plus_count_toml(count: SyncStateToml<'_, Count>) -> Result<(), ()> {

    count.mutate(|count| count.count += 1 ).await;

    Ok(())
}

#[tauri::command]
async fn reset_count(count: SyncState<'_, Count>) -> Result<(), ()> {
    
    count.mutate(|count| *count = Count::default() ).await;

    Ok(())
}

#[tauri::command]
async fn reset_count_toml(count: SyncStateToml<'_, Count>) -> Result<(), ()> {

    count.mutate(|count| *count = Count::default() ).await;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::new()
            .manage::<Count>("count")
            .manage_toml::<Count>("count_toml", "count.toml")
            .build()
        )
        .invoke_handler(tauri::generate_handler![
            get_count,
            plus_count,
            reset_count,
            get_count_toml,
            plus_count_toml,
            reset_count_toml,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
