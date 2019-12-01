#![deny(warnings)]
#![deny(unsafe_code)]

use stm32l0xx_hal::{serial};

use core::fmt::Write;

pub struct GPS<'a> {
    pub detx: &'a mut serial::Tx<serial::USART2>,
}

impl<'a> GPS<'a> {
    pub fn new(debug_tx: &'a mut serial::Tx<serial::USART2>) -> Self {
        GPS {
            detx: debug_tx
        }
    }

    pub fn hello(&mut self) -> () {
        writeln!(self.detx, "- - - - HABEX - - - -\r").unwrap();
    }
}