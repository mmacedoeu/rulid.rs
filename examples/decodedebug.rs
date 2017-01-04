extern crate rulid;
extern crate chrono;

use std::ascii::AsciiExt;

const CROCKFORD_INV_ALPHABET: [i8; 43] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1,
                                          -1, 10, 11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20,
                                          21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31];

fn printbn(bin: &[u8]) -> () {
    for b in bin {
        print!("\t {:0>8b}", b);
    }
    println!("");
}

pub fn decode(data: &str) -> Option<Vec<u8>> {
    if !data.is_ascii() {
        return None;
    }
    println!("data str \t {:?}", data);
    let data = data.as_bytes();
    println!("data bytes \t {:?}", data);
    let alphabet = CROCKFORD_INV_ALPHABET;

    let unpadded_data_length = data.len();
    println!("data len \t {:?}", unpadded_data_length);

    let output_length = unpadded_data_length * 5 / 8;
    println!("output_length \t {:?} \t capacity \t {:?}",
             output_length,
             (output_length + 4) / 5 * 5);
    let mut ret = Vec::with_capacity((output_length + 4) / 5 * 5);
    for chunk in data.chunks(8) {
        println!("chunk \t {:?}", chunk);
        let buf = {
            let mut buf = [0u8; 8];
            for (i, &c) in chunk.iter().enumerate() {
                println!("i \t {:?} \t c \t {} \t C \t {}",
                         i,
                         c,
                         c.to_ascii_uppercase().wrapping_sub(b'0') as usize);
                match alphabet.get(c.to_ascii_uppercase().wrapping_sub(b'0') as usize) {
                    Some(&-1) | None => return None,
                    Some(&value) => buf[i] = value as u8,
                };
            }
            buf
        };
        println!("buf \t {:?}", buf);
        println!("ret \t {:?}", ret);
        printbn(&ret);
        ret.push((buf[0] << 3) | (buf[1] >> 2));
        println!("ret \t {:?}", ret);
        ret.push((buf[1] << 6) | (buf[2] << 1) | (buf[3] >> 4));
        println!("ret \t {:?}", ret);
        ret.push((buf[3] << 4) | (buf[4] >> 1));
        println!("ret \t {:?}", ret);
        ret.push((buf[4] << 7) | (buf[5] << 2) | (buf[6] >> 3));
        println!("ret \t {:?}", ret);
        ret.push((buf[6] << 5) | buf[7]);
    }

    println!("ret \t {:?}", ret);
    Some(ret)
}


fn main() {
    let input = "01B5HZP8RM";
    let output = decode(input).unwrap();
    println!("output: \t {:?}", output);
}
