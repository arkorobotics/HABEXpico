#![deny(warnings)]
#![deny(unsafe_code)]

use stm32l0xx_hal::{pac, prelude::*, gpio, spi};

pub struct RADIO<'a> {
    pub radio_spi: &'a mut spi::Spi< pac::SPI1, (gpio::gpioa::PA5<gpio::Input<gpio::Floating>>, 
        gpio::gpioa::PA6<gpio::Input<gpio::Floating>>, 
        gpio::gpioa::PA7<gpio::Input<gpio::Floating>>)>,
    pub radio_nss: &'a mut gpio::gpioa::PA4<gpio::Output<gpio::PushPull>>,
}

impl<'a> RADIO<'a> {
    pub fn new( radio_spi: &'a mut spi::Spi< pac::SPI1, (gpio::gpioa::PA5<gpio::Input<gpio::Floating>>, 
        gpio::gpioa::PA6<gpio::Input<gpio::Floating>>, 
        gpio::gpioa::PA7<gpio::Input<gpio::Floating>>)>, 
                radio_nss: &'a mut gpio::gpioa::PA4<gpio::Output<gpio::PushPull>>
              ) -> Self {
        RADIO {
            radio_spi: radio_spi,
            radio_nss: radio_nss,
        }
    }

    pub fn init(&mut self) -> () {
        self.radio_nss.set_low().unwrap();
        self.radio_spi.write(&[0, 1]).unwrap();
        self.radio_nss.set_high().unwrap();
    }
}