use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};

pub fn setup_hotkey() {
    let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::KeyD);
    let manager = GlobalHotKeyManager::new().unwrap();
    manager.register(hotkey).unwrap();
}
