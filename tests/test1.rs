extern crate rulid;
extern crate chrono;

use std::{thread, time};
use chrono::UTC;

#[test]
fn test2millistime() {
    let two_millis = time::Duration::from_millis(2);
    let mut now = UTC::now();
    let mut time = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
    let enct1 = rulid::encode_time(time);

    thread::sleep(two_millis);

    now = UTC::now();
    time = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
    let enct2 = rulid::encode_time(time);

    assert!(enct2 > enct1);
}

#[test]
fn test1millistime() {
    let two_millis = time::Duration::from_millis(1);
    let mut now = UTC::now();
    let mut time = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
    let enct1 = rulid::encode_time(time);

    thread::sleep(two_millis);

    now = UTC::now();
    time = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
    let enct2 = rulid::encode_time(time);

    assert!(enct2 > enct1);
}
