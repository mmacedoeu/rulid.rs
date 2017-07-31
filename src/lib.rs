extern crate base32;
extern crate chrono;
extern crate byteorder;
extern crate rand;

use byteorder::{ByteOrder, BigEndian};
use base32::Alphabet;
use chrono::Utc;
use rand::os::OsRng;
use rand::Rng;
use std::ascii::AsciiExt;

const CROCKFORD_ALPHABET: &'static [u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const CROCKFORD_INV_ALPHABET: [i8; 43] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1,
                                          -1, 10, 11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20,
                                          21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31];

// fn printbn(bin: &[u8]) -> () {
//     for b in bin {
//         print!("\t {:0>8b}", b);
//     }
//     println!("");
// }

// *               outputs: 1        2        3        4        5
// * inputs: 1 = ---11111 = 11111---
// *         2 = ---222XX = -----222 XX------
// *         3 = ---33333 =          --33333-
// *         4 = ---4XXXX =          -------4 XXXX----
// *         5 = ---5555X =                   ----5555 X-------
// *         6 = ---66666 =                            -66666--
// *         7 = ---77XXX =                            ------77 XXX-----
// *         8 = ---88888 =                                     ---88888

// * truncate 2 bits on left
// *               outputs: 1        2        3        4        5
// * inputs: 1 = -----111 = 111-----
// *         2 = ---22222 = ---22222 --------
// *         3 = ---33333 =          33333---
// *         4 = ---444XX =          -----444 44------
// *         5 = ---55555 =                   --55555- --------
// *         6 = ---6XXXX =                   -------6 6666----
// *         7 = ---7777X =                            ----7777 7-------
// *         8 = ---88888 =                                     -88888--
// *         9 = ---99XXX =                                     ------99

pub fn base32d_time(data: &str) -> Option<Vec<u8>> {
    // println!("data str \t {:?}", data);
    let data = data.as_bytes();
    // println!("data bytes \t {:?}", data);
    let alphabet = CROCKFORD_INV_ALPHABET;

    let mut ret = Vec::with_capacity(7);
    let mut buf = vec![0u8;10];

    for (i, &c) in data.iter().enumerate() {
        // println!("i \t {:?} \t c \t {} \t C \t {}",
        //          i,
        //          c,
        //          c.to_ascii_uppercase().wrapping_sub(b'0') as usize);
        match alphabet.get(c.to_ascii_uppercase().wrapping_sub(b'0') as usize) {
            Some(&-1) | None => return None,
            Some(&value) => buf[i] = value as u8,
        };

        // println!("buf \t {:?}", buf);
    }

    for i in 0..data.len() {
        match i {
            0 => {
                // println!("buf[0] \t {:0>8b}", buf[0]);
                // println!("buf[1] \t {:0>8b}", buf[1]);
                ret.push((buf[0] << 5) | buf[1]);
                // println!("ret \t {:?}", ret);
            }
            1 => {
                // println!("buf[2] \t {:0>8b}", buf[2]);
                // println!("buf[3] \t {:0>8b}", buf[3]);
                ret.push((buf[2] << 3) | (buf[3] >> 2));
                // println!("ret \t {:?}", ret);
            }
            2 => {
                // println!("buf[3] \t {:0>8b}", buf[3]);
                // println!("buf[4] \t {:0>8b}", buf[4]);
                // println!("buf[5] \t {:0>8b}", buf[5]);
                ret.push((buf[3] << 6) | (buf[4] << 1) | (buf[5] >> 4));
                // println!("ret \t {:?}", ret);
            }
            3 => {
                // println!("buf[5] \t {:0>8b}", buf[5]);
                // println!("buf[6] \t {:0>8b}", buf[6]);
                ret.push((buf[5] << 4) | (buf[6] >> 1));
                // println!("ret \t {:?}", ret);
            }
            4 => {
                // println!("buf[6] \t {:0>8b}", buf[6]);
                // println!("buf[7] \t {:0>8b}", buf[7]);
                // println!("buf[8] \t {:0>8b}", buf[8]);
                ret.push((buf[6] << 7) | buf[7] << 2 | buf[8] >> 3);
                // println!("ret \t {:?}", ret);
            }
            5 => {
                // println!("buf[8] \t {:0>8b}", buf[8]);
                // println!("buf[9] \t {:0>8b}", buf[9]);
                ret.push((buf[8] << 5) | buf[9]);
                // println!("ret \t {:?}", ret);
            }
            _ => {}
        };
        // printbn(&ret);
    }

    Some(ret)
}

// *               outputs: 1        2        3        4        5
// * inputs: 1 = ---11111 = 11111---
// *         2 = ---222XX = -----222 XX------
// *         3 = ---33333 =          --33333-
// *         4 = ---4XXXX =          -------4 XXXX----
// *         5 = ---5555X =                   ----5555 X-------
// *         6 = ---66666 =                            -66666--
// *         7 = ---77XXX =                            ------77 XXX-----
// *         8 = ---88888 =                                     ---88888


pub fn base32e_time(data: &[u8]) -> String {
    // println!("data str \t {:?}", data);
    let alphabet = CROCKFORD_ALPHABET;
    let mut ret = Vec::with_capacity(10);

    // printbn(&data);
    // println!("buf[0] \t {:0>8b}", data[0]);
    let mut index = (data[0] >> 3) as usize;
    // println!("index 1\t {:?}", index);
    ret.push(alphabet[index]);
    // println!("ret \t {:?}", ret);
    // println!("buf[0] \t {:0>8b}", data[0]);
    // println!("buf[1] \t {:0>8b}", data[1]);
    index = (((data[0] & 0x07) << 2) | (data[1] >> 6)) as usize;
    // println!("index 2\t {:?}", index);
    ret.push(alphabet[index]);
    // println!("ret \t {:?}", ret);
    index = ((data[1] & 0x3E) >> 1) as usize;
    // println!("index 3\t {:?}", index);
    ret.push(alphabet[index]);
    // println!("ret \t {:?}", ret);
    // println!("buf[1] \t {:0>8b}", data[1]);
    // println!("buf[2] \t {:0>8b}", data[2]);
    index = (((data[1] & 0x01) << 4) | ((data[2] & 0xF0) >> 4)) as usize;
    // println!("index 4\t {:?}", index);
    ret.push(alphabet[index]);
    // println!("ret \t {:?}", ret);
    index = (((data[2] & 0x0F) << 1) | (data[3] >> 7)) as usize;
    // println!("index 5\t {:?}", index);
    ret.push(alphabet[index]);
    // println!("ret \t {:?}", ret);
    // println!("data[3] \t {:0>8b}", data[3]);
    // println!("0x7C \t {:0>8b}", 0x7C);
    // println!("data[3] & 0x7C \t {:0>8b}", (data[3] & 0x7C));
    // println!("data[3] & 0x7C >> 2 \t {:0>8b}", (data[3] & 0x7C) >> 2);
    index = ((data[3] & 0x7C) >> 2) as usize;
    // println!("index 6\t {:?} \t {}",
    //          index,
    //          String::from_utf8(vec![alphabet[index]]).unwrap());
    ret.push(alphabet[index]);
    // println!("ret \t {:?}", ret);
    // println!("data[3] \t {:0>8b}", data[3]);
    // println!("0x03 \t {:0>8b}", 0x03);
    // println!("data[3] & 0x03 \t {:0>8b}", (data[3] & 0x03));
    // println!("data[3] & 0x03 << 3 \t {:0>8b}", (data[3] & 0x03) << 3);
    // println!("data[4] \t {:0>8b}", data[4]);
    // println!("0xE0 \t {:0>8b}", 0xE0);
    // println!("data[4] & 0xE0 \t {:0>8b}", (data[4] & 0xE0));
    // println!("data[4] & 0xE0 >> 5 \t {:0>8b}", (data[4] & 0xE0) >> 5);
    // println!("((data[3] & 0x03) << 3) | ((data[4] & 0xE0) >> 5) \t {:0>8b}",
    //  ((data[3] & 0x03) << 3) | ((data[4] & 0xE0) >> 5));
    index = (((data[3] & 0x03) << 3) | ((data[4] & 0xE0) >> 5)) as usize;
    // println!("index 7\t {:?} \t {}",
    //          index,
    //          String::from_utf8(vec![alphabet[index]]).unwrap());
    ret.push(alphabet[index]);
    // println!("ret \t {:?}", ret);
    // println!("data[4] \t {:0>8b}", data[4]);
    // println!("0x1F \t {:0>8b}", 0x1F);
    // println!("data[4] & 0x1F \t {:0>8b}", (data[4] & 0x1F));
    index = (data[4] & 0x1F) as usize;
    // println!("index 8\t {:?}  \t {}",
    //          index,
    //          String::from_utf8(vec![alphabet[index]]).unwrap());
    ret.push(alphabet[index]);
    // println!("ret \t {:?}", ret);

    // println!("buf[5] \t {:0>8b}", data[5]);
    index = (data[5] >> 3) as usize;
    // println!("index 9\t {:?}", index);
    ret.push(alphabet[index]);
    // println!("ret \t {:?}", ret);

    // println!("buf[6] \t {:0>8b}", data[6]);
    index = (((data[5] & 0x07) << 2) | ((data[6] & 0xC0) >> 6)) as usize;
    // println!("index 10\t {:?}", index);
    ret.push(alphabet[index]);
    // println!("ret \t {:?}", ret);
    String::from_utf8(ret).unwrap()
}

pub fn right_shift_2bit(buf: &mut [u8]) -> () {
    let mut m = (buf[0] & 0x04) << 6;
    let mut c: u8;
    // printbn(&buf);
    buf[0] = buf[0] >> 2;
    let length = buf.len();
    for i in 1..length - 1 {
        // printbn(&buf);
        c = (buf[i] & 0x04) << 6;
        buf[i] = buf[i] >> 2 | m;
        m = c.clone();
    }
    buf[length - 1] = m;
    ()
}

pub fn left_shift_6bit(buf: &mut [u8]) -> () {
    let length = buf.len();
    // printbn(&buf);
    for i in 0..length - 1 {
        let m = buf[i + 1] >> 2;
        // print!("buf+1 \t {:0>8b}", buf[i + 1]);
        // println!("\t m \t {:0>8b}", m);
        // print!("buf \t {:0>8b}", buf[i]);
        // println!("\t buf \t {:0>8b}", buf[i] << 6);
        buf[i] = buf[i] << 6 | m;
        // printbn(&buf);
    }
    buf[length - 1] = buf[length - 1] << 6;
    // printbn(&buf);
    ()
}

pub fn timeasbytearray(time: u64) -> Vec<u8> {
    let mut buf = vec![0; 8];
    BigEndian::write_u64(&mut buf, time);
    buf
}

pub fn encode_time(time: u64) -> String {
    let mut vec = timeasbytearray(time);
    // println!("vec \t {:?}", &vec[1..8]);
    left_shift_6bit(&mut vec[1..8]);
    // println!("after left shift");
    // printbn(&vec[1..8]);
    base32e_time(&vec[1..8])
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

pub fn gen_random<R: Rng>(mut rng: R) -> Vec<u8> {
    let mut buf = vec![0u8; 10];
    rng.fill_bytes(&mut buf);
    buf
}

pub fn ulid() -> String {
    let now = Utc::now();
    let time: u64 = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
    let enct = encode_time(time);
    String::new() + &enct[enct.len() - 10..] + encode_random().as_str()
}

pub fn from_vec(v: &[u8]) -> String {
    let mut x = vec![0u8; 7];
    x[1..].clone_from_slice(&v[0..6]);
    left_shift_6bit(&mut x);
    // printbn(&x);
    let enct = base32e_time(&x);
    String::new() + &enct[enct.len() - 10..] +
    base32::encode(Alphabet::Crockford, &v[6..16]).as_str()
}

pub fn from_u64(msr: u64, lsr: u64) -> String {
    let mut x: Vec<u8> = vec![0; 16];
    BigEndian::write_u64(&mut x[0..8], msr);
    BigEndian::write_u64(&mut x[8..16], lsr);
    // println!("length: {} \t asVec: {:?}", x.len(), x);
    from_vec(&x)
}

pub fn from_i64(msr: i64, lsr: i64) -> String {
    let mut x: Vec<u8> = vec![0; 16];
    BigEndian::write_i64(&mut x[0..8], msr);
    BigEndian::write_i64(&mut x[8..16], lsr);
    // println!("length: {} \t asVec: {:?}", x.len(), x);
    from_vec(&x)
}

pub fn as_vec(ulid: &str) -> Option<Vec<u8>> {
    if let Some(t) = base32d_time(&ulid[0..10]) {
        // printbn(&t);
        let mut x: Vec<u8> = vec![0; 16];
        x[0..6].clone_from_slice(&t[0..6]);
        if let Some(r) = base32::decode(Alphabet::Crockford, &ulid[10..26]) {
            x[6..16].clone_from_slice(&r[0..10]);
            Some(x)
        } else {
            Option::None
        }
    } else {
        Option::None
    }
}

pub fn as_u64tuple(ulid: &str) -> Option<(u64, u64)> {
    if let Some(bytes) = as_vec(ulid) {
        let msr = BigEndian::read_u64(&bytes[0..8]);
        let lsr = BigEndian::read_u64(&bytes[8..16]);
        Option::Some((msr, lsr))
    } else {
        Option::None
    }
}

pub fn as_i64tuple(ulid: &str) -> Option<(i64, i64)> {
    if let Some(bytes) = as_vec(ulid) {
        let msr = BigEndian::read_i64(&bytes[0..8]);
        let lsr = BigEndian::read_i64(&bytes[8..16]);
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
    let now = Utc::now();
    let time: u64 = now.timestamp() as u64 * 1000 + now.timestamp_subsec_millis() as u64;
    let _ = timeasbytearray(time);
    assert!(true);
}

#[test]
fn test_encode_time() {
    let _ = encode_time(Utc::now().timestamp() as u64);
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
