#[path="../../../Firmware/src/nfmt.rs"] 
mod nfmt;

#[path="../../../Firmware/src/habex.rs"]
mod habex;

#[cfg(test)]
mod nfmt_ts {

    use super::nfmt;
    use super::habex;

    /// Nano Format Unit Test Script
    #[test]
    fn nfmt_test() -> () {

        // Test "i32_to_ns"
        println!("[nfmt] Testing i32_to_ns...");

        // Min/max values and tricky ranges
        let test_array: [i32; 17] = [-2147483648, -2147483647, -100, -11, -10, -9, -2, -1, 0, 1, 2, 9, 10, 11, 100, 2147483646, 2147483647];

        for x in 0..test_array.len() {
            let l_i32: i32 = test_array[x];
            print!("[nfmt] i32: {}, ", l_i32);

            let mut l_i32_ns: [char; 11] = [0 as char; 11];
            match nfmt::i32_to_ns(l_i32) {
                Ok(s) => { 
                    l_i32_ns = s;
                    print!("i32_to_ns: ");
                    print_ns(l_i32_ns); 
                },
                Err(e) => { 
                    panic!("[nfmt] i32_to_ns: error code - {}", e as u8);
                },
            }

            let mut l_ns_i32: i32 = 0;
            match nfmt::ns_to_i32(l_i32_ns) {
                Ok(s) => { 
                    l_ns_i32 = s;
                    print!(", ns_to_i32: {}", l_ns_i32);
                },
                Err(e) => { 
                    panic!("[nfmt] ns_to_i32: error code - {}", e as u8);
                },
            }

            println!("");
            
            assert_eq!(l_i32, l_ns_i32);
        }

        // Test "h2_to_u8"
        println!("[nfmt] Testing h2_to_u8...");
        let h2: [char; 2] = ['A','5'];
        let expected_h2_u8: u8 = 0xA5;

        let mut h2_u8: u8 = 0;

        match nfmt::h2_to_u8(h2) {
            Ok(s) => { 
                h2_u8 = s;
                println!("[nfmt] Expected H2 as U8: {:X?}", expected_h2_u8);
                println!("[nfmt] Converted H2 as U8: {:X?}", h2_u8);
                assert_eq!(expected_h2_u8, h2_u8);
            },
            Err(e) => { 
                panic!("[nfmt] h2_to_u8: error code - {}", e as u8);
            },
        }

        println!("[nfmt] Test Complete!")
    }

    /// Print nano string
    pub fn print_ns(ns: [char; 11]) -> () {
        for i in 0..11 {
            print!("{}", ns[i]);
        }
    }
}