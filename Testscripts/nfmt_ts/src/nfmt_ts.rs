#[path="../../../Firmware/src/nfmt.rs"]
mod nfmt;


/// Nano Format Unit Test Script
fn main() -> () {
    
    let test_array: [i32; 17] = [-2147483648, -2147483647, -100, -11, -10, -9, -2, -1, 0, 1, 2, 9, 10, 11, 100, 2147483646, 2147483647];

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