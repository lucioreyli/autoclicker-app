use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};
use tauri::{App, Manager};

pub fn setup_hotkey() {
    let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::KeyD);
    let manager = GlobalHotKeyManager::new().unwrap();
    manager.register(hotkey).unwrap();
}

pub fn setup_window(app: &mut App) -> Result<(), &'static str> {
    let window = app.get_window("main").unwrap();

    let always_on_top = cfg!(debug_assertions);
    window.set_always_on_top(!always_on_top).unwrap();

    window.set_resizable(false).unwrap();

    Ok(())
}
