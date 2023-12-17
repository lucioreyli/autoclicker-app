use std::{thread, time::Duration};

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    state: bool,
}

pub fn sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
