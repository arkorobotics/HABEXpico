//#![deny(warnings)]
#![deny(unsafe_code)]

use stm32l0xx_hal::{prelude::*, gpio, serial};
use nb::block;

use super::nmea;
use super::habex;

pub struct GPS<'a> {
    pub gps_tx: &'a mut serial::Tx<serial::LPUART1>,
    pub gps_rx: &'a mut serial::Rx<serial::LPUART1>,
    pub gps_en: &'a mut gpio::gpioa::PA1<gpio::Output<gpio::PushPull>>,
    pub nmea: nmea::NMEA,
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
            nmea: nmea::NMEA::new(),
        }
    }

    pub fn init(&mut self) -> () {
        let _gps_err = self.gps_en.set_low();
        self.gps_rx.clear_errors();
    }

    pub fn clear_errors(&mut self) -> (){
        self.gps_rx.clear_errors();
    }

    pub fn read_char(&mut self) -> char {
        match block!(self.gps_rx.read()) {
            Ok(s) => { 
                return s as char; 
            }
            Err(_e) => { 
                // Clear any UART errors
                self.gps_rx.clear_errors();
                return 0 as char; 
            }
        }
    }

    #[allow(dead_code)]
    /// Read GPS UART interface and update the NMEA struct
    pub fn get_packet(&mut self) -> Result<(), habex::Ecode> {

        let mut packet: [char; 100] = [0 as char; 100];
        let mut gga_valid: u8 = 0;

        // Look for valid $GNGGA header
        while gga_valid == 0 {

            // Clear header buffer
            packet[0] = 0 as char;
            packet[1] = 0 as char;
            packet[2] = 0 as char;
            packet[3] = 0 as char;
            packet[4] = 0 as char;

            // Sync to start delimiter "$" (0x24)
            while packet[0] != '$' {
                packet[0] = self.read_char();
            }

            // Read the header
            packet[1] = self.read_char();
            packet[2] = self.read_char();
            packet[3] = self.read_char();
            packet[4] = self.read_char();
            
            // Look for "$xxGGx"
            if (packet[3] == 'G') && (packet[4] == 'G') {
                gga_valid = 1;
            }
        }

        // Quickly buffer the remaining characters
        for i in 5..100 {
            packet[i] = self.read_char();
            if packet[i] == '\n' { break; }
        }

        // Parse the $GNGGA packet
        match self.nmea.parse_gga_packet_to_nmea(packet) {
            Ok(_s) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}