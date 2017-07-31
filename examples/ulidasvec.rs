extern crate rulid;
extern crate chrono;

use std::{thread, time};

fn main() {
    let two_millis = time::Duration::from_millis(2);
    for _ in 0..16 {
        // let time = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
        let ulid = rulid::ulid();
        // let now = Utc::now();
        let as_vec = rulid::as_vec(ulid.as_str()).unwrap();
        let (umsr, ulsr) = rulid::as_u64tuple(ulid.as_str()).unwrap();
        let (imsr, ilsr) = rulid::as_i64tuple(ulid.as_str()).unwrap();
        println!("ulid: {} \t asVec: {:?} \t msr u: {} \t msr i: {} \t lsr u: {} \t lsr i: {}",
                 ulid,
                 as_vec,
                 umsr,
                 imsr,
                 ulsr,
                 ilsr);

        thread::sleep(two_millis);
    }
}
