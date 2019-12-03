#![deny(warnings)]
#![deny(unsafe_code)]

use stm32l0xx_hal::{prelude::*, gpio, serial};

use nb::block;

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

    pub fn get_packet(&mut self) -> u8 {
        let received = block!(self.gps_rx.read()).unwrap();
        return received;
    }
}