#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    api::notification::Notification, CustomMenuItem, Icon, Manager, SystemTray, SystemTrayMenu,
    SystemTrayMenuItem,
};

#[tauri::command]
fn dosomething(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let context = tauri::generate_context!("./tauri.conf.json");
    let identifier = context.config().tauri.bundle.identifier.clone();
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![dosomething])
        .setup(move |app| {
            app.get_window("main").unwrap().open_devtools();
            app.tray_handle()
                .set_icon(Icon::Raw(include_bytes!("../icons/FlashMark.ico").to_vec()))
                .unwrap();
            Notification::new(&identifier)
                .title("Tauri")
                .body("Tauri is awesome!")
                .show()
                .unwrap();

            Ok(())
        })
        .system_tray(tray)
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .run(context)
        .expect("error while running tauri application");
}
