#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]


extern crate panic_halt;

use cortex_m_rt::entry;

//use nb::block;

mod console;
mod gps;
mod pal;
mod power;
mod radio;

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
    
    let mut radio = radio::RADIO::new(&mut pal.radio_spi,
                                      &mut pal.radio_nss);
    
    radio.init();

    let mut power = power::POWER::new(&mut pal.adc,
                                      &mut pal.adc_vstore);

    console.cprint("- - - - BOOT COMPLETE - - - -\r");

    let vstore = power.read_vstore();

    console.cprint_telem("ADC: VSTORE=", vstore);

    loop {
        
        console.print_char(gps.get_packet());
        
    }
}
