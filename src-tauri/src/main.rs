// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

struct Status(Mutex<bool>);

use helpers::create_thread;
use std::sync::Mutex;
use tauri::Manager;
mod helpers;
mod setup;

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
    setup::setup_hotkey();
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Status(false.into()));

            create_thread()?;

            setup::setup_window(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![is_started, toggle_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
