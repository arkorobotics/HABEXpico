#[path="../../../Firmware/src/nmea.rs"]
mod nmea;

#[path="../../../Firmware/src/nfmt.rs"]
mod nfmt;

#[path="../../../Firmware/src/habex.rs"]
mod habex;

#[cfg(test)]
mod nmea_ts {

    use std::result;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use super::nmea;
    use super::nfmt;
    use super::habex;

    /// NMEA Unit Test Script
    #[test]
    fn nmea_test() -> () {
        
        // Error Count Variables
        let mut NmeaCsFail_count: u32 = 0;
        let mut NmeaInvalidGga_count: u32 = 0;

        // Open the file in read-only mode (ignoring errors)
        let filename = "../../Datastore/FieldTestData/HABEXPico8_A_fieldtest_20726_hermosa_beach.txt";
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        // Initialize NMEA packet
        let mut packet: [char; 100] = [0 as char; 100];
        
        // Read GPS NMEA message
        let nmea_string = String::from("");

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (index, line) in reader.lines().enumerate() {
            let nmea_string = line.unwrap(); // Ignore errors.
            
            // [debug] Show the line and its number.
            // println!("[nmea] File Line #{}: {}", index + 1, nmea_string);

            // Convert String to GPS Packet type
            packet = nmea_string_to_nmea_packet(nmea_string).unwrap();

            let mut nmea = nmea::NMEA::new();

            match nmea.parse_gga_packet_to_nmea(packet) {
                Ok(s) => { 
                    // Print parsed nmea packet
                    println!("[nmea] Parsed NMEA GGA Packet - \
                    utc: {},      \
                    lat_deg: {},  \
                    lat_min: {},  \
                    lat_NS: {},   \
                    long_deg: {}, \
                    long_min: {}, \
                    long_WE: {},  \
                    alt: {},      \
                    cs: 0x{:X?},  \
                    calc_cs: 0x{:X?}",
                    nmea.utc, 
                    nmea.lat_deg, 
                    nmea.lat_min, 
                    nmea.lat_NS,
                    nmea.long_deg, 
                    nmea.long_min, 
                    nmea.long_WE,
                    nmea.alt,
                    nmea.cs,
                    nmea.calc_cs);
                    },
                Err(e) => {
                    let err = e;
                    match err {
                        habex::Ecode::NmeaCsFail => { NmeaCsFail_count += 1; }
                        habex::Ecode::NmeaInvalidGga => { NmeaInvalidGga_count += 1; }
                        _ => { panic!("[nmea] parse_gga_packet_to_nmea: error code - {}", err as u8); }
                    }
                    
                },
            }
        }

        println!("[nmea] Ignored NmeaCsFail Error Count: {}", NmeaCsFail_count);
        println!("[nmea] Ignored NmeaInvalidGga Packet Count: {}", NmeaInvalidGga_count);
        println!("[nmea] Test Complete!")
    }

    /// Converts a NMEA string to a NMEA Packet type
    fn nmea_string_to_nmea_packet(nmea_string: String) -> Result<[char; 100], &'static str> {
        
        // Convert nmea string to vector array
        let nmea_vec = nmea_string.chars().collect::<Vec<char>>();

        let mut nmea_packet: [char; 100] = [0 as char; 100];

        // Check string length
        if nmea_vec.len() > 100 
        {
            return Err("gps_ts::string_to_packet - String too long.");
        }

        // Debug - Print packet
        // print_nmea_vec(nmea_vec.clone());

        for (i, x) in nmea_vec.iter().enumerate()
        {
            nmea_packet[i] = *x as char;
        }

        return Ok(nmea_packet);
    }

    /// Debug print NMEA vector
    fn print_nmea_vec(vec: Vec<char>)
    {
        // Print vector length
        println!("Vector Length: {}", vec.len());
    
        // Print vector as ascii
        println!("Vector (ascii): {:?}", vec);

        // Print vector as decimal
        print!("Vector (dec): [");
        for (i, x) in vec.iter().enumerate()
        {
            print!("{:?}", *x as u8);
            
            if i < (vec.len() - 1)
            {
                print!(", ");
            }
            else 
            {
                println!("]");
            }
        }

        // Print vector as hexidecimal
        print!("Vector (hex): [");
        for (i, x) in vec.iter().enumerate()
        {
            print!("0x{:X?}", *x as u8);
            
            if i < (vec.len() - 1)
            {
                print!(", ");
            }
            else 
            {
                println!("]");
            }
        }
        
    }
}