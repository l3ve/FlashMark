use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

pub fn get_tray_menu() -> SystemTray {
    let add = CustomMenuItem::new("add".to_string(), "Add");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
    .add_item(add)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    return tray;
}
