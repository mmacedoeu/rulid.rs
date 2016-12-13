extern crate base32;
extern crate chrono;
extern crate byteorder;
extern crate rand;

use byteorder::{ByteOrder, BigEndian};
use base32::Alphabet;
use chrono::UTC;
use rand::os::OsRng;
use rand::Rng;

const CROCKFORD_ALPHABET: &'static [u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

pub fn timeasbytearray(time: u64) -> Vec<u8> {
    let mut buf = vec![0; 9];
    BigEndian::write_u64(&mut buf, time);
    buf
}

pub fn encode_time(time: u64) -> String {
    let vec = timeasbytearray(time);
    base32::encode(Alphabet::Crockford, &vec[2..9])
}

pub fn encode_random() -> String {
    let mut out = String::new();
    let mut rng = OsRng::new().unwrap();
    for _ in 0..16 {
        let n: usize = rng.gen_range(0, 31);
        out.push(CROCKFORD_ALPHABET[n] as char);
    }
    out
}

pub fn ulid() -> String {
    let now = UTC::now();
    let time: u64 = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
    let enct = encode_time(time);
    String::new() + &enct[enct.len() - 10..] + encode_random().as_str()
}

pub fn as_vec(ulid: &str) -> Option<Vec<u8>> {
    base32::decode(Alphabet::Crockford, ulid)
}

pub fn as_u64tuple(ulid: &str) -> Option<(u64, u64)> {
    if let Some(bytes) = base32::decode(Alphabet::Crockford, ulid) {
        let msr = BigEndian::read_u64(&bytes[0..8]);
        let lsr = BigEndian::read_u64(&bytes[8..16]);
        Option::Some((msr, lsr))
    } else {
        Option::None
    }
}

#[test]
fn test_encode_random() {
    let _ = encode_random();
    assert!(true);
}

#[test]
fn test_encode_random_len() {
    let r = encode_random();
    assert_eq!(r.len(), 16);
}

#[test]
fn test_timeasbytearray() {
    let now = UTC::now();
    let time: u64 = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
    let _ = timeasbytearray(time);
    assert!(true);
}

#[test]
fn test_encode_time() {
    let _ = encode_time(UTC::now().timestamp() as u64);
    assert!(true);
}

#[test]
fn test_ulid() {
    let _ = ulid();
    assert!(true);
}

#[test]
fn test_ulid_len() {
    assert_eq!(ulid().len(), 26);
}
