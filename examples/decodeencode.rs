extern crate rulid;

// fn printbn(bin: &[u8]) -> () {
//     for b in bin {
//         print!("\t {:0>8b}", b);
//     }
//     println!("");
// }

fn main() {
    // let input = "01B5GRHEQHP3XGATPW33SC14YH";
    let input = rulid::ulid();
    // println!("input: \t {:?}", input);
    let vec = rulid::as_vec(input.as_str()).unwrap();
    // println!("input: \t {:?}", vec);
    // printbn(&vec);
    let _ = rulid::from_vec(&vec);
    // println!("output: \t {:?}", output);
}
