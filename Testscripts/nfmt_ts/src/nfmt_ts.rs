#[path="../../../Firmware/src/nfmt.rs"]
mod nfmt;


/// Nano Format Unit Test Script
fn main() -> () {

    // TODO: -2147483648 is not supported at the moment. Need to fix that before continuing.
    let test_array: [i32; 2] = [-2147483647, 2147483647];

    for x in 0..test_array.len() {
        let l_i32: i32 = test_array[x];
        let l_i32_ns = nfmt::i32_to_ns(l_i32);
        print_ns(l_i32_ns);
    }
}

pub fn print_ns(ns: [char; 11]) -> () {
    for i in 0..11 {
        print!("{}", ns[i]);
    }
    println!("");
}