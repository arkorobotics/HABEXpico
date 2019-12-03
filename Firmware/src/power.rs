#![deny(warnings)]
#![deny(unsafe_code)]

use stm32l0xx_hal::{prelude::*, adc, gpio};

pub struct POWER<'a> {
    pub adc: &'a mut adc::Adc,
    pub adc_vstore: &'a mut gpio::gpioa::PA0<gpio::Analog>,
}

impl<'a> POWER<'a> {
    pub fn new( adc: &'a mut adc::Adc,
                adc_vstore: &'a mut gpio::gpioa::PA0<gpio::Analog>,
              ) -> Self {
        POWER {
            adc: adc,
            adc_vstore: adc_vstore,
        }
    }

    pub fn read_vstore(&mut self) -> u16 {
        let vstore: u16 = self.adc.read(self.adc_vstore).unwrap();
        return vstore;
    }
}