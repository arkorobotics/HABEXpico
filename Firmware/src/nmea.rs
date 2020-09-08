pub struct NMEA {
    pub utc: u32,
    pub lat: u32,
    pub long: u32,
    pub alt: u32,
}

/// Parses the specified NMEA packet field and returns its numeric value.
/// TODO: Add floating point support in another function
pub fn parse_field_u32(packet: [char; 100], field_index: u16) -> u32 {

    let mut field_start_idx = 0;
    let mut field_stop_idx = 0;

    for i in 0..100 {

        if packet[i] == ',' {
        
            //if field_index == {
                
            //}

            //field_count += 1;
        }
    }

    return 0;
}
