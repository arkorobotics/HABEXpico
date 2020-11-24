//#![deny(warnings)]
#![deny(unsafe_code)]

use stm32l0xx_hal::{prelude::*, gpio, serial};
use nb::block;

use super::nmea;

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

    pub fn read_char(&mut self) -> char {
        return block!(self.gps_rx.read()).unwrap() as char;
    }

    #[allow(dead_code)]
    pub fn get_packet(&mut self) -> [char; 100] {

        let mut packet: [char; 100] = [0 as char; 100];
        let mut gga_valid: u8 = 0;

        let mut _nmea = nmea::NMEA::new();

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
                packet[0] = self.read_char(); //block!(self.gps_rx.read()).unwrap();
            }

            packet[1] = self.read_char();
            packet[2] = self.read_char();
            packet[3] = self.read_char();
            packet[4] = self.read_char();
            packet[5] = self.read_char();
            
            if (packet[3] == 'G') && (packet[4] == 'G') && (packet[5] == 'A') {
                gga_valid = 1;
            }   
        }

        for i in 6..100 {
            packet[i] = self.read_char();
            if packet[i] == '\n' { break; }
        }

        return packet;
    }
}