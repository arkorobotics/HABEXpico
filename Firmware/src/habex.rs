#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_assignments)]
#[allow(unused_variables)]

pub enum Ecode {
    NmeaCsFail = 1,
    NmeaInvalidGga = 2,

    NfmtNsInvalidChar = 3,
    NfmtNsOutOfRange = 4,
    NfmtInvalidH2 = 5,
}