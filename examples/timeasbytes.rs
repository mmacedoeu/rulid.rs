extern crate rulid;
extern crate chrono;

use std::{thread, time};
use chrono::Utc;

fn main() {
    let two_millis = time::Duration::from_millis(2);
    for _ in 0..16 {
        let now = Utc::now();
        let time = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
        let mut enct = rulid::timeasbytearray(time);
        println!("{:?}", enct);
        rulid::left_shift_6bit(&mut enct[1..8]);
        thread::sleep(two_millis);
        println!("{:?}", enct);
    }
}
