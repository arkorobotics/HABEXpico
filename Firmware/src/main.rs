//#![deny(warnings)]
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

use stm32l0xx_hal::{pac::{self, interrupt, Interrupt}, timer::Timer};

extern crate panic_halt;

use core::ops::DerefMut;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;

mod console;
mod gps;
mod habex;
mod nmea;
mod nfmt;
mod pal;
mod power;
mod radio;

static TIMER: Mutex<RefCell<Option<Timer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));
static ST: Mutex<RefCell<Option<u32>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {

    let mut pal = pal::PAL::new();

    let mut console = console::CONSOLE::new(&mut pal.console_tx, 
                                            &mut pal.console_rx);

    console.sprintln("- - - - HABEXpico v0.0.1 - - - -");

    let mut gps = gps::GPS::new(&mut pal.gps_tx, 
                                &mut pal.gps_rx,
                                &mut pal.gps_en);
    gps.init();

    let mut radio = radio::RADIO::new(&mut pal.radio_spi,
                                      &mut pal.radio_nss);
    
    radio.init();

    let mut power = power::POWER::new(&mut pal.adc,
                                      &mut pal.adc_vstore);

    let vstore = power.read_vstore();

    // Setup Timekeeper
    let mut timer: Timer<pac::TIM2> = pal.timer;

    timer.listen();
    
    let mut stime: u32 = 0;
    stime = stime + 1;          // Start time at 1. Any time value less than 1 means there was an error.

    unsafe { NVIC::unmask(Interrupt::TIM2); }  // Enable the timer interrupt in the NVIC.

    cortex_m::interrupt::free(|cs| {
        *ST.borrow(cs).borrow_mut() = Some(stime);
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
    });
    
    console.sprintln("- - - - BOOT COMPLETE - - - -");

    loop {
        /* DEBUG MODE */
        console.print(gps.read_char());
        
        /* OPERATING MODE 
        console.sprintln("- - - -");

        let nmea = gps.get_packet();

        let utc = nfmt::u32_to_string(nmea.utc);
        let lat = nfmt::u32_to_string(nmea.lat);
        let long = nfmt::u32_to_string(nmea.long);
        let alt = nfmt::u32_to_string(nmea.alt);

        console.scprintln("UTC = ", &utc);
        console.scprintln("LAT = ", &lat);
        console.scprintln("LONG = ", &long);
        console.scprintln("ALT = ", &alt);
        
        console.scprintln("ST = ", &nfmt::u32_to_string(get_stime()));
        */
    }
}

fn get_stime() -> u32 {
    let mut current_time: u32 = 0;
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut stime) = ST.borrow(cs).borrow_mut().deref_mut() {    
            current_time = *stime;
        }
        return current_time;
    });
    return current_time;
}

#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
            // Clear the interrupt flag.
            timer.clear_irq();

            if let Some(ref mut st) = ST.borrow(cs).borrow_mut().deref_mut() {
                *st = *st + 1;
            }
        }
    });
}
