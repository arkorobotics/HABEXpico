#[path="../../../Firmware/src/nmea.rs"]
mod nmea;

use std::fs::File;
use std::io::{BufRead, BufReader};

/// GPS Unit Test Script
fn main() -> () {
    
    // Open the file in read-only mode (ignoring errors)
    let filename = "../../Datastore/FieldTestData/HABEXPico8_A_fieldtest_20726_hermosa_beach.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Initialize GPS packet
    let mut packet: [char; 100] = [0 as char; 100];
    
    // Read GPS NMEA message
    let gps_string = String::from("");

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let gps_string = line.unwrap(); // Ignore errors.

        // Next line for easy reading
        println!("");

        // Show the line and its number.
        println!("File Line #{}: {}", index + 1, gps_string);
        
        // Convert String to GPS Packet type
        // TODO: handle OK,ERR
        packet = nmea_string_to_nmea_packet(gps_string).unwrap();

        let nmea_ = nmea::parse_packet_to_nmea(packet);
        println!("{}", nmea_.utc);
    }
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

    // Print packet
    print_nmea_vec(nmea_vec.clone());

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