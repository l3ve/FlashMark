use tauri::{AppHandle, WindowBuilder, WindowUrl};

pub fn new_webview(app: AppHandle) {
    let window = WindowBuilder::new(&app, "add", WindowUrl::App("index.html".into()))
        .inner_size(300.0, 528.0)
        .title("添加闪记")
        .build()
        .unwrap();

    // window.set_always_on_top(true).unwrap();
    // window.show().unwrap();
    // window.set_decorations(false).unwrap();
    window.center().unwrap();
}
