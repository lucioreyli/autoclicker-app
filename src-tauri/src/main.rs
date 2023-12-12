// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

struct Status(Mutex<bool>);

use std::sync::Mutex;
use tauri::Manager;
mod helpers;

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
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Status(false.into()));

            // window config
            let window = app.get_window("main").unwrap();
            window.set_always_on_top(false)?; // change to true on release
            window.set_resizable(true)?; // change to false on release
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![is_started, toggle_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
