#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]


extern crate panic_halt;

use cortex_m_rt::entry;
//use stm32l0xx_hal::{pac, prelude::*, rcc::Config, serial, spi};

//use core::fmt::Write;
//use nb::block;

mod gps;
mod periph;

#[entry]
fn main() -> ! {

    let (mut debug_tx, _debug_rx) = periph::setup();

    let mut bob = gps::GPS::new(&mut debug_tx);
    bob.hello();

    loop {
        
        
        /*
        // Echo what is received on the serial link
        let received = block!(gps_rx.read()).unwrap();
        block!(debug_tx.write(received)).ok();

        nss.set_low().unwrap();
        spi.write(&[0, 1]).unwrap();
        nss.set_high().unwrap();

        let vstore: u16 = adc.read(&mut vstore_pin).unwrap();

        writeln!(debug_tx, "ADC: VSTORE={}\r", vstore).unwrap();
        */
    }
}
