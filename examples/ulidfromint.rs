extern crate rulid;

use std::{thread, time};

fn main() {
    let two_millis = time::Duration::from_millis(2);
    for _ in 0..16 {
        // let now = UTC::now();
        // let time = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
        let ulid = rulid::ulid();
        let as_vec = rulid::as_vec(ulid.as_str()).unwrap();
        let (umsr, ulsr) = rulid::as_u64tuple(ulid.as_str()).unwrap();
        let (imsr, ilsr) = rulid::as_i64tuple(ulid.as_str()).unwrap();
        let ulid_from_u64 = rulid::from_u64(umsr, ulsr);
        let ulid_from_i64 = rulid::from_i64(imsr, ilsr);
        let ulid_from_vec = rulid::from_vec(&as_vec);
        println!("ulid: {} \t asVec: {:?} \t msr u: {} \t msr i: {} \t lsr u: {} \t lsr i: {} \t \
                  ulid_from_i64: {} \t ulid_from_u64: {} \t ulid_from_vec:{}",
                 ulid,
                 as_vec,
                 umsr,
                 imsr,
                 ulsr,
                 ilsr,
                 ulid_from_i64,
                 ulid_from_u64,
                 ulid_from_vec);

        thread::sleep(two_millis);
    }
}
