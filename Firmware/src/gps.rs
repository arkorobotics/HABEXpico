//#![deny(warnings)]
#![deny(unsafe_code)]

use stm32l0xx_hal::{prelude::*, gpio, serial};

use nb::block;

pub struct NMEA {
    pub utc: u32,
    pub lat: u32,
    pub long: u32,
    pub alt: u32,
}

pub struct GPS<'a> {
    pub gps_tx: &'a mut serial::Tx<serial::LPUART1>,
    pub gps_rx: &'a mut serial::Rx<serial::LPUART1>,
    pub gps_en: &'a mut gpio::gpioa::PA1<gpio::Output<gpio::PushPull>>,
}

impl<'a> GPS<'a> {
    pub fn new( gps_tx: &'a mut serial::Tx<serial::LPUART1>, 
                gps_rx: &'a mut serial::Rx<serial::LPUART1>,
                gps_en: &'a mut gpio::gpioa::PA1<gpio::Output<gpio::PushPull>>
              ) -> Self {
        GPS {
            gps_tx: gps_tx,
            gps_rx: gps_rx,
            gps_en: gps_en,
        }
    }

    pub fn init(&mut self) -> () {
        let _gps_err = self.gps_en.set_low();

        // Try adding:
        // self.gps_rx.clear_errors();
    }

    pub fn read_char(&mut self) -> u8 {
        return block!(self.gps_rx.read()).unwrap();
    }

    pub fn get_packet(&mut self) -> NMEA {

        let mut packet: [u8; 100] = [0; 100];
        let mut gga_valid: u8 = 0;

        for i in 0..100 {
            packet[i] = 0;
        }
        
        while gga_valid == 0 {

            // And getting rid of v
            self.gps_rx.clear_errors();

            // Sync to start delimiter "$" (0x24)
            while packet[0] != ('$' as u8) {
                packet[0] = block!(self.gps_rx.read()).unwrap();
            }

            packet[1] = block!(self.gps_rx.read()).unwrap();
            packet[2] = block!(self.gps_rx.read()).unwrap();
            packet[3] = block!(self.gps_rx.read()).unwrap();
            packet[4] = block!(self.gps_rx.read()).unwrap();
            
            if (packet[1] == ('G' as u8)) && (packet[2] == ('N' as u8)) && (packet[3] == ('G' as u8)) && (packet[4] == ('G' as u8)){
                gga_valid = 1;
            }   
        }

        for i in 5..100 {
            packet[i] = block!(self.gps_rx.read()).unwrap();
            if packet[i] == ('\n' as u8) { break; }
        }

        // Parse NMEA Packet
        // let lat = s.parse::<i32>().unwrap();

        let nmea: NMEA = NMEA { utc: 0, lat: 0, long: 0, alt: 0 };
        return nmea;
    }

    /// Parses the specified NMEA packet field and returns its numeric value
    /// TODO: Add floating point support in another function
    pub fn parse_field_u32(packet: [u8; 100], field_index: u16) -> u32 {

        let mut field_start_idx = 0;
        let mut field_stop_idx = 0;

        for i in 0..100 {

            if packet[i] == (',' as u8) {
            
                //if field_index == {
                    
                //}

                //field_count += 1;
            }
        }

        return 0;
    }
}