use super::nfmt;
use super::habex;

pub struct NMEA {
    pub utc: i32,
    pub lat_deg: i32,
    pub lat_min: i32,
    pub lat_ns: char,
    pub long_deg: i32,
    pub long_min: i32,
    pub long_we: char,
    pub alt: i32,
    pub cs: u8,
    pub calc_cs: u8,
}

impl NMEA {
    #[allow(dead_code)]
    pub fn new() -> Self {
        NMEA {
            utc: 0,
            lat_deg: 0,
            lat_min: 0,     // MM.mm
            lat_ns: ' ',
            long_deg: 0,
            long_min: 0,    // MM.mm
            long_we: ' ',
            alt: 0,
            cs: 0,
            calc_cs: 0,
        }
    }

    /// Parse GGA Packet
    #[allow(dead_code)]
    pub fn parse_gga_packet_to_nmea(&mut self, packet: [char; 100]) -> Result<(), habex::Ecode> {
                
        // Look for GGA Header
        if (packet[3] == 'G') && (packet[4] == 'G') && (packet[5] == 'A') {
        
            self.calc_cs = 0;

            let mut comma_idx = 0;
            let mut i = 1;

            while i < 100 {

                if i == 1 {
                    self.calc_cs = packet[1] as u8;
                }
                else if packet[i] != '*'{
                    self.calc_cs = self.calc_cs ^ (packet[i] as u8);
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

                                match nfmt::ns_to_i32(utc_ns) {
                                    Ok(s) => { self.utc = s; },
                                    Err(e) => { return Err(e); },
                                }
                            }, 
                            3 => {  // Lat "DDMM.mmxxx,"

                                {   // Parse Latitude Degrees
                                    let mut lat_deg_ns: [char; 11] = [0 as char; 11];
                                    lat_deg_ns[10] = packet[(i-1)-8];
                                    lat_deg_ns[9] = packet[(i-1)-9];
                                    
                                    match nfmt::ns_to_i32(lat_deg_ns) {
                                        Ok(s) => { self.lat_deg = s; },
                                        Err(e) => { return Err(e); },
                                    }
                                }

                                {   // Parse Latitude Minutes
                                    let mut lat_min_ns: [char; 11] = [0 as char; 11];
                                    lat_min_ns[10] = packet[(i-1)-3];
                                    lat_min_ns[9] = packet[(i-1)-4];
                                    lat_min_ns[8] = packet[(i-1)-6];
                                    lat_min_ns[7] = packet[(i-1)-7];

                                    match nfmt::ns_to_i32(lat_min_ns) {
                                        Ok(s) => { self.lat_min = s; },
                                        Err(e) => { return Err(e); },
                                    }
                                }
                            }, 
                            4 => {  // Lat polarity "X,"
                                    
                                    self.lat_ns = packet[(i-1)];
                            }, 
                            5 => {  // Long "DDDMM.mmxxx,"

                                {   // Parse Longitude Degrees
                                    let mut long_deg_ns: [char; 11] = [0 as char; 11];
                                    long_deg_ns[10] = packet[(i-1)-8];
                                    long_deg_ns[9] = packet[(i-1)-9];
                                    long_deg_ns[8] = packet[(i-1)-10];

                                    match nfmt::ns_to_i32(long_deg_ns) {
                                        Ok(s) => { self.long_deg = s; },
                                        Err(e) => { return Err(e); },
                                    }
                                }

                                {   // Parse Longitude Minutes
                                    let mut long_min_ns: [char; 11] = [0 as char; 11];
                                    long_min_ns[10] = packet[(i-1)-3];
                                    long_min_ns[9] = packet[(i-1)-4];
                                    long_min_ns[8] = packet[(i-1)-6];
                                    long_min_ns[7] = packet[(i-1)-7];

                                    match nfmt::ns_to_i32(long_min_ns) {
                                        Ok(s) => { self.long_min = s; },
                                        Err(e) => { return Err(e); },
                                    }
                                }
                            }, 
                            6 => {  // Long polarity "X,"

                                self.long_we = packet[(i-1)];
                            }, 
                            10 => {  // Altitude "XXXXX.x,"
                                // Loop towards the left till you hit a comma or count more than 5
                                let mut alt_ns: [char; 11] = [0 as char; 11];

                                for x in 2..7 {
                                    if packet[(i-1)-x] == ',' {
                                        match nfmt::ns_to_i32(alt_ns) {
                                            Ok(s) => { self.alt = s; },
                                            Err(e) => { return Err(e); },
                                        }
                                        
                                        break;
                                    }

                                    alt_ns[10-(x-2)] = packet[(i-1)-x];
                                }

                                match nfmt::ns_to_i32(alt_ns) {
                                    Ok(s) => { self.alt = s; },
                                    Err(e) => { return Err(e); },
                                }
                            }, 
                            
                            _ => (),
                        }
                    }
                }
                else if packet[i] == '*' {  // Checksum "HH"

                    let mut cs_ns: [char; 2] = [0 as char; 2];

                    cs_ns[1] = packet[(i+2)];
                    cs_ns[0] = packet[(i+1)];

                    match nfmt::h2_to_u8(cs_ns) {
                        Ok(s) => { self.cs = s; },
                        Err(e) => { return Err(e); },
                    }

                    break;
                }

                i += 1;
            }

            if self.calc_cs == self.cs {
                return Ok(());
            }
            else {
                return Err(habex::Ecode::NmeaCsFail);
            }
        }
        else {
            // eventually returns an error
            return Err(habex::Ecode::NmeaInvalidGga);
        }
    }
}