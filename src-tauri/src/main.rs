use tauri::{
    generate_context, generate_handler, ActivationPolicy::Accessory, Builder, Manager,
    PhysicalPosition, RunEvent, SystemTrayEvent, WindowEvent,
};

mod ipc;
mod menu;
mod webview_center;

fn main() {
    let context = generate_context!("./tauri.conf.json");    
    let mut app = Builder::default()
        // 注册前后端 ipc 通信
        .invoke_handler(generate_handler![
            ipc::log_operation,
            ipc::perform_request,
            ipc::alert
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
    app.set_activation_policy(Accessory);
    
    // 调试代码
    let window = app.get_window("main").unwrap();
    window
        .set_position(PhysicalPosition {
            x: 2182.0,
            y: -2112.0,
        })
        .unwrap();

    app.run(|app_handle, e| {
        match e {
            RunEvent::WindowEvent { label, event, .. } => match event {
                WindowEvent::CloseRequested { api, .. } => {
                    println!("{:#?}", label);
                    if label == "main" {
                        app_handle.get_window(&label).unwrap().hide().unwrap();
                        api.prevent_close();
                    }
                }
                WindowEvent::Focused(focused) => {
                    if !focused && label == "main" {
                        app_handle.get_window(&label).unwrap().hide().unwrap();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    });
}
