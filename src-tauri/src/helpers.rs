use std::thread;

use global_hotkey::GlobalHotKeyEvent;

pub fn create_thread() -> Result<thread::JoinHandle<()>, &'static str> {
    let created_thread = thread::spawn(move || loop {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            println!("{:?}", event);
        }
    });
    Ok(created_thread)
}
