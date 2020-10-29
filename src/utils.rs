use std::{thread, time};

pub fn sleep(duration: u64) {
    let millis = time::Duration::from_millis(duration);
    thread::sleep(millis);
}
