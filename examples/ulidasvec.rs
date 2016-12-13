extern crate rulid;
extern crate chrono;

use std::{thread, time};
use chrono::UTC;

fn main() {
    let two_millis = time::Duration::from_millis(2);
    for _ in 0..16 {
        let now = UTC::now();
        let time = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
        let ulid = rulid::ulid();
        let asVec = rulid::as_vec(ulid.as_str()).unwrap();
        let (msr, lsr) = rulid::as_u64tuple(ulid.as_str()).unwrap();
        thread::sleep(two_millis);

        println!("ulid: {} \t asVec: {:?} \t msr: {}", ulid, asVec, msr);
    }
}
