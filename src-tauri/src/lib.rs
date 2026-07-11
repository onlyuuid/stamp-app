// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod command;
mod service;
mod repository;
mod entity;
mod dto;
mod db;
mod state;

use command::task_command;
use command::water_log_command;
use tauri::Manager;
use crate::state::AppState;

use crate::db::connection::init_db;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    tauri::Builder::default()
            .setup(|app|{
            let app_handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let db =
                    init_db(&app_handle)
                    .await
                    .expect("数据库初始化失败");
                app_handle.manage(
                    AppState{
                        db
                    }
                );
            });

            Ok(())
        })

        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            task_command::list_task,
            task_command::create_task, 
            task_command::update_task,
            task_command::delete_task,
            task_command::find_by_id,
            task_command::weekly_stats,
   
            water_log_command::list_water_log,
            water_log_command::find_by_log_id,
            water_log_command::find_by_task_id,
            water_log_command::create_water_log,
            water_log_command::update_water_log,
            water_log_command::delete_water_log,
            water_log_command::find_by_date,
            water_log_command::week_total,
            water_log_command::streak_days,
            water_log_command::get_heatmap
            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
