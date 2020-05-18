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
    }

    pub fn get_packet(&mut self) -> NMEA {

        // TODO: Add interrupt mask here to prevent TIM2 from firing during packet rx?

        let mut packet: [u8; 100] = [0; 100];
        let mut gga_valid: u8 = 0;

        while gga_valid == 0 {
            
            for i in 0..100 {
                packet[i] = 0;
            }

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
}