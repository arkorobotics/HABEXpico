use super::nfmt;

pub struct NMEA {
    pub utc: i32,
    pub lat_deg: i32,
    pub lat_min: i32,
    pub lat_NS: char,
    pub long_deg: i32,
    pub long_min: i32,
    pub long_WE: char,
    pub alt: i32,
    pub cs: u8,
}

impl NMEA {
    pub fn new() -> Self {
        NMEA {
            utc: 0,
            lat_deg: 0,
            lat_min: 0,     // MM.mm
            lat_NS: ' ',
            long_deg: 0,
            long_min: 0,    // MM.mm
            long_WE: ' ',
            alt: 0,
            cs: 0,
        }
    }

    /// Parse GGA Packet
    pub fn parse_gga_packet_to_nmea(&mut self, packet: [char; 100]) -> Result<(), u8> {

        // Look for GGA Header
        if (packet[3] == 'G') && (packet[4] == 'G') && (packet[5] == 'A') {
        
            let mut cs_calc: u8 = 0;

            let mut comma_idx = 0;
            let mut i = 1;

            while i < 100 {

                if i == 1 {
                    cs_calc = packet[1] as u8;
                }
                else if packet[i] != '*'{
                    cs_calc = cs_calc ^ (packet[i] as u8);
                }

                if packet[i] == ',' {         // Find commas delimited fields

                    comma_idx = comma_idx + 1;  // Increment comma index counter

                    if packet[i-1] != ',' {   // Check the field for non-zero length
                    
                        match comma_idx {
                            2 => {  // Time "HHMMSS.ss,"

                                let mut utc_ns: [char; 11] = [0 as char; 11];
        
                                for x in 3..9 {
                                    utc_ns[10-(x-3)] = packet[(i-1)-x];
                                }
        
                                self.utc = nfmt::ns_to_i32(utc_ns);
                            }, 
                            3 => {  // Lat "DDMM.mmxxx,"

                                {   // Parse Latitude Degrees
                                    let mut lat_deg_ns: [char; 11] = [0 as char; 11];
                                    lat_deg_ns[10] = packet[(i-1)-8];
                                    lat_deg_ns[9] = packet[(i-1)-9];
                                    self.lat_deg = nfmt::ns_to_i32(lat_deg_ns);
                                }

                                {   // Parse Latitude Minutes
                                    let mut lat_min_ns: [char; 11] = [0 as char; 11];
                                    lat_min_ns[10] = packet[(i-1)-3];
                                    lat_min_ns[9] = packet[(i-1)-4];
                                    lat_min_ns[8] = packet[(i-1)-6];
                                    lat_min_ns[7] = packet[(i-1)-7];
                                    self.lat_min = nfmt::ns_to_i32(lat_min_ns);
                                }
                            }, 
                            4 => {  // Lat polarity "X,"
                                    
                                    self.lat_NS = packet[(i-1)];
                            }, 
                            5 => {  // Long "DDDMM.mmxxx,"

                                {   // Parse Longitude Degrees
                                    let mut long_deg_ns: [char; 11] = [0 as char; 11];
                                    long_deg_ns[10] = packet[(i-1)-8];
                                    long_deg_ns[9] = packet[(i-1)-9];
                                    long_deg_ns[8] = packet[(i-1)-10];
                                    self.long_deg = nfmt::ns_to_i32(long_deg_ns);
                                }

                                {   // Parse Longitude Minutes
                                    let mut long_min_ns: [char; 11] = [0 as char; 11];
                                    long_min_ns[10] = packet[(i-1)-3];
                                    long_min_ns[9] = packet[(i-1)-4];
                                    long_min_ns[8] = packet[(i-1)-6];
                                    long_min_ns[7] = packet[(i-1)-7];
                                    self.long_min = nfmt::ns_to_i32(long_min_ns);
                                }
                            }, 
                            6 => {  // Long polarity "X,"

                                self.long_WE = packet[(i-1)];
                            }, 
                            10 => {  // Altitude "XXXXX.x,"
                                // Loop towards the left till you hit a comma or count more than 5
                                let mut alt_ns: [char; 11] = [0 as char; 11];

                                for x in 2..6 {
                                    if packet[(i-1)-x] == ',' {
                                        self.alt = nfmt::ns_to_i32(alt_ns);
                                        break;
                                    }

                                    alt_ns[10-(x-2)] = packet[(i-1)-x];
                                }
                                self.alt = nfmt::ns_to_i32(alt_ns);
                            }, 
                            
                            _ => (),
                        }
                    }
                }
                else if packet[i] == '*' {  // Checksum "HH"

                    let mut cs_ns: [char; 2] = [0 as char; 2];

                    cs_ns[1] = packet[(i+2)];
                    cs_ns[0] = packet[(i+1)];

                    self.cs = nfmt::h2_to_u8(cs_ns);
                    break;
                }

                i += 1;
            }

            // Verify checksum
            println!("Calculated Checksum - dec: {}, hex: 0x{:X?}", cs_calc, cs_calc);
            println!("Checksum - dec: {}, hex: 0x{:X?}", self.cs, self.cs);

            if cs_calc == self.cs {
                return Ok(());
            }
            else {
                return Err(2);
            }
        }
        else {
            // eventually returns an error
            return Err(1);
        }
    }
}