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

        // Try adding:
        // self.gps_rx.clear_errors();
    }

    pub fn read_char(&mut self) -> char {
        return block!(self.gps_rx.read()).unwrap() as char;
    }

    #[allow(dead_code)]
    pub fn get_packet(&mut self) -> Result<(), habex::Ecode> {

        let mut packet: [char; 100] = [0 as char; 100];
        let mut gga_valid: u8 = 0;

        // Clear buffer
        for i in 0..100 {
            packet[i] = 0 as char;
        }
        
        // 
        while gga_valid == 0 {

            // Clear any UART errors. 
            // TODO: This is not good practice. Future builds should include error checking and resolving.
            self.gps_rx.clear_errors();

            // Sync to start delimiter "$" (0x24)
            while packet[0] != '$' {
                packet[0] = self.read_char();
            }

            packet[1] = self.read_char();
            packet[2] = self.read_char();
            packet[3] = self.read_char();
            packet[4] = self.read_char();
            
            // Look for "$xxGG"
            // This is a time sensitive if-statement. If the program spends too much time checking
            // for the correct characters, it will miss the remaining characters and crash.
            // TODO: Resolve the time sensitive if-statement bug mentioned above.
            if (packet[3] == 'G') && (packet[4] == 'G'){
                gga_valid = 1;
            }
        }

        for i in 5..100 {
            packet[i] = self.read_char();
            if packet[i] == '\n' { break; }
        }

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