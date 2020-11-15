/// Nano Format (nfmt)

/// nfmt uses fixed size char arrays to store the string
/// resprenstation of an unsigned value.

/// Converts U32 value into a nanoformat number string
pub fn i32_to_ns<'a>(input: i32) -> [char; 11] {

    let mut s: [char; 11] = [0 as char; 11];

    if input < 0 {
        s[0] = '-' as char;
    }

    let num: u32 = input.abs() as u32;

    let mut i = 10;

    let mut result: u32 = num / 10;
    let mut rem: u8 = (num % 10) as u8;

    if num < 10 {
        s[i] = (rem + 0x30) as char;
        i -= 1;
    }
    else {
        // u16 to 'string' conversion goes here
        while result >= 10 {
            s[i] = (rem + 0x30) as char;
            i -= 1;
            rem = (result % 10) as u8;
            result = result / 10;
        }
        s[i] = (rem + 0x30) as char;
        i -= 1;
        s[i] = ((result as u8) + 0x30) as char;
    }

    return s;
}