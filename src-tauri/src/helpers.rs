use std::thread;

pub fn _create_thread(_thread_name: &str) {
    let _thread = thread::spawn(move || loop {
        panic!("dale");
    });
}
