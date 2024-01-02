#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};
use global_hotkey::{GlobalHotKeyEvent, HotKeyState};

use std::{sync::Mutex, thread};
use tauri::Manager;

mod helpers;
mod setup;

struct Status(Mutex<bool>);

#[tauri::command]
fn is_started(state: tauri::State<Status>) -> bool {
    *state.0.lock().unwrap()
}

#[tauri::command]
fn toggle_state(state: tauri::State<Status>) -> bool {
    let mut prev_value = *state.0.lock().unwrap();
    prev_value = !prev_value;
    prev_value
}

fn main() {
    let hotkey = HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyD);
    let manager = GlobalHotKeyManager::new().unwrap();
    manager.register(hotkey).unwrap();
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Status(false.into()));
            thread::spawn(move || loop {
                let receiver = GlobalHotKeyEvent::receiver();
                let event = receiver.try_recv();
                if event.is_ok() {
                    let event = event.unwrap();
                    if event.state() == HotKeyState::Released {
                        println!("enemies manda diss");
                    }
                }
                helpers::sleep(1000);
            });
            setup::setup_window(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![is_started, toggle_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
