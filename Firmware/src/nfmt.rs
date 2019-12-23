// Nano Format (nfmt)

pub fn u32_to_string<'a>(num: u32) -> [u8; 10] {

    let mut s: [u8; 10] = [0; 10];

    let mut i = 9;

    let mut result: u32 = num / 10;
    let mut rem: u8 = (num % 10) as u8;

    if num < 10 {
        s[i] = rem + 0x30;
        i -= 1;
    }
    else {
        // u16 to 'string' conversion goes here
        while result >= 10 {
            s[i] = rem + 0x30;
            i -= 1;
            rem = (result % 10) as u8;
            result = result / 10;
        }
        s[i] = rem + 0x30;
        i -= 1;
        s[i] = (result as u8) + 0x30;
    }

    return s;
}
