#[path="../../../Firmware/src/nfmt.rs"]
mod nfmt;


/// Nano Format Unit Test Script
fn main() -> () {

    // Test "i32_to_ns"
    println!("Testing i32_to_ns:");

    // Min/max values and tricky ranges
    let test_array: [i32; 17] = [-2147483648, -2147483647, -100, -11, -10, -9, -2, -1, 0, 1, 2, 9, 10, 11, 100, 2147483646, 2147483647];

    for x in 0..test_array.len() {
        let l_i32: i32 = test_array[x];
        print!("i32: {}, ", l_i32);

        let l_i32_ns = nfmt::i32_to_ns(l_i32);
        print!("i32_to_ns: ");
        print_ns(l_i32_ns);

        let l_ns_i32 = nfmt::ns_to_i32(l_i32_ns);
        print!(", ns_to_i32: {}", l_ns_i32);

        println!("");
        
        assert_eq!(l_i32, l_ns_i32);
    }

    // Test "h2_to_u8"
    let h2: [char; 2] = ['A','5'];
    let expected_h2_u8: u8 = 0xA5;

    let h2_u8: u8 = nfmt::h2_to_u8(h2);
    println!("Expected H2 as U8: {:X?}", expected_h2_u8);
    println!("Converted H2 as U8: {:X?}", h2_u8);
    assert_eq!(expected_h2_u8, h2_u8);
}

/// Print nano string
pub fn print_ns(ns: [char; 11]) -> () {
    for i in 0..11 {
        print!("{}", ns[i]);
    }
}