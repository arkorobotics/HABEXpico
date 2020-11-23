/// Nano Format (nfmt)

/// nfmt uses fixed size char arrays to store the string
/// resprenstation of an unsigned value. The goal here is to
/// use fixed size arrays to avoid heap allocation.

use super::habex;

/// Convert an I32 value into a nanostring
pub fn i32_to_ns(input: i32) -> Result<[char; 11], habex::Ecode> {

    // s[0] = polarity ('-' = negative, '\0' or '+' = positive)
    // s[1..10] = integer
    let mut s: [char; 11] = [0 as char; 11];

    // Set polarity character
    if input < 0 {
        s[0] = '-' as char;
    }

    // Set integer characters
    let num: i32 = input;

    let mut _i = 10;

    let mut result: u32 = (num / 10).abs() as u32;
    let mut rem: u8;

    // Handle the initial remainder values
    if input < 0 {
        rem = ((10 - (num % 10)) %10) as u8;
    }
    else {
        rem = (num % 10) as u8;
    }

    if (num < 10) && (num > -10) {
        s[_i] = (rem + 0x30) as char;
        _i -= 1;
    }
    else {
        // Increment through the remaining integer digits
        while result >= 10 {
            s[_i] = (rem + 0x30) as char;
            _i -= 1;
            rem = (result % 10) as u8;
            result = result / 10;
        }
        s[_i] = (rem + 0x30) as char;
        _i -= 1;
        s[_i] = ((result as u8) + 0x30) as char;
    }

    return Ok(s);
}

/// Convert a nanostring into a I32 value
pub fn ns_to_i32(ns: [char; 11]) -> Result<i32, habex::Ecode> {

    let mut ns_i32: i32 = 0;
    let mut result: i64 = 0;
    let mut sign_flag: bool = false; 

    // Increment through the nanostring array to generate the integer
    for i in 0..11 {
        if ns[i] >= '0' && ns[i] <= '9' {
            result = (result * 10) + ((ns[i] as i64) - ('0' as i64));
        }
        else if ns[i] == (0 as char) {
            // Ignore character
        }
        else if ns[i] == '-' {
            // Set polarity flag to apply after result computation is complete
            sign_flag = true;
        }
        else {
            return Err(habex::Ecode::NfmtNsInvalidChar);
        }
    }

    // Set polarity
    if sign_flag == true {
        result = result * -1;
    }

    // Sanity check the result will fit in an i32 then assign it
    if result >= -2147483648 && result <= 2147483647 {
        ns_i32 = result as i32;
    }
    else {
        return Err(habex::Ecode::NfmtNsOutOfRange);
    }

    return Ok(ns_i32);
}

/// Convert a two element char array representing an 8-bit hex value to u8
pub fn h2_to_u8(h2: [char; 2]) -> Result<u8, habex::Ecode> {

    let mut h2_u8: u8 = 0;
    
    for i in 0..2 {
        match h2[i] {
            '0'..='9' => { h2_u8 = h2_u8 + (((h2[i] as u8) - ('0' as u8)) << ((1-i)*4)) },
            'a'..='f' => { h2_u8 = h2_u8 + (((h2[i] as u8) - ('a' as u8) + 10) << ((1-i)*4)) },
            'A'..='F' => { h2_u8 = h2_u8 + (((h2[i] as u8) - ('A' as u8) + 10) << ((1-i)*4)) },
            _ => { return Err(habex::Ecode::NfmtInvalidH2) },
        }
    }

    return Ok(h2_u8);
}
