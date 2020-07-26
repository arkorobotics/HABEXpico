//#![deny(warnings)]
#![deny(unsafe_code)]

use stm32l0xx_hal::{prelude::*, serial};
//use stm32l0xx_hal::{prelude::*, serial};

//use core::fmt::Write;

use nb::block;

pub struct CONSOLE<'a> {
    pub console_tx: &'a mut serial::Tx<serial::USART2>,
    pub console_rx: &'a mut serial::Rx<serial::USART2>,
}

impl<'a> CONSOLE<'a> {
    pub fn new( console_tx: &'a mut serial::Tx<serial::USART2>, 
                console_rx: &'a mut serial::Rx<serial::USART2>
              ) -> Self {
        CONSOLE {
            console_tx: console_tx,
            console_rx: console_rx
        }
    }

    pub fn print(&mut self, c: u8 ) -> () {
            block!(self.console_tx.write(c)).ok();
    }

    pub fn sprint(&mut self, s: &str) -> () {
        for c in s.chars() { 
            block!(self.console_tx.write(c as u8)).ok();
        }
    }

    pub fn sprintln(&mut self, s: &str) -> () {
        for c in s.chars() { 
            block!(self.console_tx.write(c as u8)).ok();
        }
        self.sprint("\r\n");
    }

    pub fn cprint(&mut self, c: &[u8] ) -> () {
        for i in 0..c.len() {
            block!(self.console_tx.write(c[i])).ok();
        }
    }

    pub fn cprintln(&mut self, c: &[u8] ) -> () {
        for i in 0..c.len() {
            block!(self.console_tx.write(c[i])).ok();
        }
        self.sprint("\r\n");
    }

    pub fn scprint(&mut self, s: &str, c: &[u8]) -> () {
        self.sprint(s);
        self.cprint(c);
    }

    pub fn scprintln(&mut self, s: &str, c: &[u8]) -> () {
        self.sprint(s);
        self.cprint(c);
        self.sprint("\r\n");
    }
}