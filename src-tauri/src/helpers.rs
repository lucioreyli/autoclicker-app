use std::thread;

pub fn create_thread() -> thread::JoinHandle<()> {
    let created_thread = thread::spawn(move || loop {
        panic!("dale");
    });
    created_thread
}
