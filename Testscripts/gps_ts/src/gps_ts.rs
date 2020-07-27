#[path="../../../Firmware/src/nmea.rs"]
mod nmea;

fn main() -> () {
    let mut packet: [u8; 100] = [0; 100];
    
    //packet = "$GNGGA,231605.00,,,,,0,00,99.99,,,,,,*7B".as_bytes();

    nmea::parse_field_u32(packet, 0);
}