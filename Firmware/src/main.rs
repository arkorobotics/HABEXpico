#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]


extern crate panic_halt;

use cortex_m_rt::entry;
//use stm32l0xx_hal::{pac, prelude::*, rcc::Config, serial, spi};

//use core::fmt::Write;
//use nb::block;

mod console;
mod gps;
mod pal;

#[entry]
fn main() -> ! {

    let mut pal = pal::PAL::new();

    let mut console = console::CONSOLE::new(&mut pal.console_tx, 
                                            &mut pal.console_rx);

    console.cprint("- - - - HABEXpico v0.0.1 - - - -\r");
    console.cprint("Booting...\r");

    let mut gps = gps::GPS::new(&mut pal.gps_tx, 
                                &mut pal.gps_rx,
                                &mut pal.gps_en);
    gps.init();
    
    console.cprint("- - - - BOOT COMPLETE - - - -\r");

    loop {
        
        console.print_char(gps.get_packet());
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
