/// Nano Format (nfmt)

/// nfmt uses fixed size char arrays to store the string
/// resprenstation of an unsigned value. The goal here is to
/// use fixed size arrays to avoid heap allocation.

// TODO: Add error handling!!!

/// Converts I32 value into a nanostring
pub fn i32_to_ns<'a>(input: i32) -> [char; 11] {

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

    return s;
}

/// Converts a nanostring into a I32 value
pub fn ns_to_i32(ns: [char; 11]) -> i32 {

    // TODO: Add error handling for values out side the min/max range of i32

    let mut ns_i32: i32 = 0;
    let mut result: i64 = 0;

    for i in 1..11 {
        if ns[i] != (0 as char) {
            result = (result * 10) + ((ns[i] as i64) - ('0' as i64));
        }
    }

    if ns[0] == '-' {
        result = result * -1;
    }

    if result >= -2147483648 && result <= 2147483647 {
        ns_i32 = result as i32;
    }

    return ns_i32;
}

/// Converts a two element char array representing an 8-bit hex value to u8
pub fn h2_to_u8(h2: [char; 2]) -> u8 {

    let mut h2_u8: u8 = 0;

    for i in 0..2 {
        match h2[i] {
            '0'..='9' => { h2_u8 = h2_u8 + (((h2[i] as u8) - ('0' as u8)) << ((1-i)*4)) },
            'a'..='f' => { h2_u8 = h2_u8 + (((h2[i] as u8) - ('a' as u8) + 10) << ((1-i)*4)) },
            'A'..='F' => { h2_u8 = h2_u8 + (((h2[i] as u8) - ('A' as u8) + 10) << ((1-i)*4)) },
            _ => (),
        }
    }

    return h2_u8;
}