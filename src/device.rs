use std::time::{SystemTime, UNIX_EPOCH};

enum GlowState {
    
}

pub struct Device {
    last_connected: Option<u128>,
    state: State,
}

pub fn enumerate(& mut self) {
    let start = SystemTime::now();
    let since_the_epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards").as_millis();
    self.last_connected = since_the_epoch;
}
