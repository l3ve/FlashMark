#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, RunEvent};

mod ipc;
mod menu;

fn main() {
    let context = tauri::generate_context!("./tauri.conf.json");

    let mut app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ipc::log_operation,
            ipc::perform_request
        ])
        .system_tray(menu::get_tray_menu())
        .build(context)
        .expect("error while running tauri application");

    app.set_activation_policy(tauri::ActivationPolicy::Regular);
    // app.get_window("main").unwrap().open_devtools();

    app.run(|app_handle, e| match e {
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    })
}
