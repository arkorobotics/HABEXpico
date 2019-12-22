// Nano Format (nfmt)

pub fn u16_to_string<'a>(num: u16) -> [u8; 10] {
    let mut s: [u8; 10] = [0; 10];
    s[0] = 'H' as u8;
    s[1] = 'E' as u8;
    s[2] = 'Y' as u8;
    return s;
}
