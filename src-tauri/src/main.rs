use tauri::{Manager, RunEvent, SystemTrayEvent, WindowEvent};

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
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { position, .. } => {
                let window = app.get_window("main").unwrap();
                window.set_position(position).unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "add" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                    window.set_always_on_top(true).unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .build(context)
        .expect("error while running tauri application");

    // 隐藏 dock 里的图标
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    let window = app.get_window("main").unwrap();

    window.open_devtools();
    window.hide().unwrap();
    // 无边框模式
    window.set_decorations(false).unwrap();

    app.run(|app_handle, e| match e {
        RunEvent::WindowEvent { label, event, .. } => match event {
            WindowEvent::CloseRequested { api, .. } => {
                app_handle.get_window(&label).unwrap().hide().unwrap();
                api.prevent_close();
            }
            WindowEvent::Focused(focused) => {
                if !focused {
                    app_handle.get_window(&label).unwrap().hide().unwrap();
                }
            }
            _ => {}
        },
        _ => {}
    })
}
