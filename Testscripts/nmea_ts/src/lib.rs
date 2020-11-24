#[path="../../../Firmware/src/nmea.rs"]
mod nmea;

#[path="../../../Firmware/src/nfmt.rs"]
mod nfmt;

#[path="../../../Firmware/src/habex.rs"]
mod habex;

mod nmea_stress_cases;

/// NMEA Module Test Script
#[cfg(test)]
mod nmea_ts {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use super::nmea;
    use super::habex;

    use super::nmea_stress_cases::*;

    /// NMEA Replayer Test Script
    #[test]
    #[allow(dead_code)]
    fn nmea_replayer_test() -> () {
        
        println!(" ");
        println!("[nmea] Starting Replayer Test...");

        let testfile = vec!["../../Datastore/FieldTestData/HABEXPico8_A_fieldtest_20726_hermosa_beach.txt"];

        for filename in testfile.iter() {

            // Error Count Variables
            let mut nmea_cs_fail_count: u32 = 0;
            let mut nmea_invalid_gga_count: u32 = 0;

            // Open the file in read-only mode (ignoring errors)
            let file = File::open(filename).unwrap();
            let reader = BufReader::new(file);

            // Initialize empty nmea packet
            let mut packet: [char; 100] = [0 as char; 100];
            
            // Read GPS NMEA message
            let _nmea_string = String::from("");

            // Read the file line by line using the lines() iterator from std::io::BufRead.
            for (_index, line) in reader.lines().enumerate() {
                let _nmea_string = line.unwrap(); // Ignore errors.
                
                // [debug] Show the line and its number.
                // println!("[nmea] File Line #{}: {}", _index + 1, _nmea_string);

                // Convert String to GPS Packet type
                match nmea_string_to_nmea_packet(_nmea_string) {
                    Ok(s) => { packet = s; }
                    Err(e) => { println!("[nmea] nmea_string_to_nmea_packet: Ignored error code - {}", e); }
                }

                // Create NMEA Struct
                let mut nmea = nmea::NMEA::new();

                // Attempt to parse the NMEA GGA packet
                match nmea.parse_gga_packet_to_nmea(packet) {
                    Ok(_s) => { 
                        // Print parsed nmea packet
                        println!("[nmea] Parsed NMEA GGA Packet - \
                        utc: {},      \
                        lat_deg: {},  \
                        lat_min: {},  \
                        lat_ns: {},   \
                        long_deg: {}, \
                        long_min: {}, \
                        long_we: {},  \
                        alt: {},      \
                        cs: 0x{:X?},  \
                        calc_cs: 0x{:X?}",
                        nmea.utc, 
                        nmea.lat_deg, 
                        nmea.lat_min, 
                        nmea.lat_ns,
                        nmea.long_deg, 
                        nmea.long_min, 
                        nmea.long_we,
                        nmea.alt,
                        nmea.cs,
                        nmea.calc_cs);
                        },
                    Err(e) => {
                        let err = e;
                        match err {
                            habex::Ecode::NmeaCsFail => { nmea_cs_fail_count += 1; }
                            habex::Ecode::NmeaInvalidGga => { nmea_invalid_gga_count += 1; }
                            _ => { println!("[nmea] parse_gga_packet_to_nmea: error code - {}", err as u8); }
                        }
                    },
                }
            }

            println!("[nmea] Ignored NmeaCsFail Error Count: {}", nmea_cs_fail_count);
            println!("[nmea] Ignored NmeaInvalidGga Packet Count: {}", nmea_invalid_gga_count);
        }

        println!("[nmea] Replayer Test Complete!");
    }

    /// NMEA Stress Test Script
    #[test]
    #[allow(dead_code)]
    fn nmea_stress_test() -> () {
        println!(" ");
        println!("[nmea] Starting Stress Test...");

        // Error Count Variables
        let mut nmea_cs_fail_count: u32 = 0;
        let mut nmea_invalid_gga_count: u32 = 0;

        // Loop through stress cases and check parsed data against expected results
        for i in 0..STRESS_CASES.len() {
            println!("{}", STRESS_CASES[i].nmea_string.to_string());

            // Initialize empty nmea packet
            let mut packet: [char; 100] = [0 as char; 100];

            // Convert String to GPS Packet type
            match nmea_string_to_nmea_packet(STRESS_CASES[i].nmea_string.to_string()) {
                Ok(s) => { packet = s; }
                Err(e) => { println!("[nmea] nmea_string_to_nmea_packet: Ignored error code - {}", e); }
            }

            // Create NMEA Struct
            let mut nmea = nmea::NMEA::new();

            // Attempt to parse the NMEA GGA packet
            match nmea.parse_gga_packet_to_nmea(packet) {
                Ok(_s) => { 
                    // Print parsed nmea packet
                    print!("[nmea] Parsed NMEA GGA Packet: ");
                    print!("utc parsed: {} - expect: {}, ", nmea.utc, STRESS_CASES[i].utc);
                    assert_eq!(nmea.utc, STRESS_CASES[i].utc);
                    print!("lat_deg parsed: {} - expect: {}, ", nmea.lat_deg, STRESS_CASES[i].lat_deg);
                    assert_eq!(nmea.lat_deg, STRESS_CASES[i].lat_deg);
                    print!("lat_min parsed: {} - expect: {}, ", nmea.lat_min, STRESS_CASES[i].lat_min);
                    assert_eq!(nmea.lat_min, STRESS_CASES[i].lat_min);
                    print!("lat_ns parsed: {} - expect: {}, ", nmea.lat_ns, STRESS_CASES[i].lat_ns);
                    assert_eq!(nmea.lat_ns, STRESS_CASES[i].lat_ns);
                    print!("long_deg parsed: {} - expect: {}, ", nmea.long_deg, STRESS_CASES[i].long_deg);
                    assert_eq!(nmea.long_deg, STRESS_CASES[i].long_deg);
                    print!("long_min parsed: {} - expect: {}, ", nmea.long_min, STRESS_CASES[i].long_min);
                    assert_eq!(nmea.long_min, STRESS_CASES[i].long_min);
                    print!("long_we parsed: {} - expect: {}, ", nmea.long_we, STRESS_CASES[i].long_we);
                    assert_eq!(nmea.long_we, STRESS_CASES[i].long_we);
                    print!("alt parsed: {} - expect: {}", nmea.alt, STRESS_CASES[i].alt);
                    assert_eq!(nmea.alt, STRESS_CASES[i].alt);
                    println!(" ");
                }
                Err(e) => {
                    let err = e;
                    match err {
                        habex::Ecode::NmeaCsFail => { nmea_cs_fail_count += 1; }
                        habex::Ecode::NmeaInvalidGga => { nmea_invalid_gga_count += 1; }
                        _ => { println!("[nmea] parse_gga_packet_to_nmea: error code - {}", err as u8); }
                    }
                },
            }
        }
        
        println!("[nmea] Ignored NmeaCsFail Error Count: {}", nmea_cs_fail_count);
        println!("[nmea] Ignored NmeaInvalidGga Packet Count: {}", nmea_invalid_gga_count);

        println!("[nmea] Stress Test Complete!");
    }

    /// Converts a NMEA string to a NMEA Packet type
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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