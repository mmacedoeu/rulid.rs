extern crate rulid;
extern crate chrono;
extern crate base32;

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

#[test]
fn testdecodeencodeasvec() {
    let input = "01B5GRHEQHP3XGATPW33SC14YH";
    let vec = rulid::as_vec(input).unwrap();
    let output = rulid::from_vec(&vec);
    assert!(input == output);
}

#[test]
fn testdecodeencodei64() {
    let input = rulid::ulid();
    let (msr, lsr) = rulid::as_i64tuple(&input).unwrap();
    let output = rulid::from_i64(msr, lsr);
    assert!(input == output);
}

#[test]
fn testdecodeencodeu64() {
    let input = rulid::ulid();
    let (msr, lsr) = rulid::as_u64tuple(&input).unwrap();
    let output = rulid::from_u64(msr, lsr);
    assert!(input == output);
}
