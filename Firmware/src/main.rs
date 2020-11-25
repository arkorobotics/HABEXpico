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
static ST: Mutex<RefCell<Option<i32>>> = Mutex::new(RefCell::new(None));

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

    let _vstore = power.read_vstore();

    // Setup Timekeeper
    let mut timer: Timer<pac::TIM2> = pal.timer;

    timer.listen();
    
    let mut stime: i32 = 0;
    stime = stime + 1;          // Start time at 1. Any time value less than 1 means there was an error.

    unsafe { NVIC::unmask(Interrupt::TIM2); }  // Enable the timer interrupt in the NVIC.

    cortex_m::interrupt::free(|cs| {
        *ST.borrow(cs).borrow_mut() = Some(stime);
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
    });
    
    console.sprintln("- - - - BOOT COMPLETE - - - -");

    loop {
        /* DEBUG MODE */
        //console.print(gps.read_char());
        
        /* OPERATING MODE */
        console.sprintln("-");
        // GPS Packet acquisition loop is deterministic and can get stuck
        // looping over and missing the $GNGGA Packet
        match gps.get_packet() {
            Ok(_) => {
                console.sprintln("[gps] - New packet!");
                match nfmt::i32_to_ns(gps.nmea.utc) {
                    Ok(s) => { console.scprintln("UTC = ", &s);}
                    Err(_e) => { console.sprintln("UTC ERROR!"); } //Handle Error
                }

                match nfmt::i32_to_ns(gps.nmea.lat_deg) {
                    Ok(s) => { console.scprintln("LAT DD = ", &s);}
                    Err(_e) => { console.sprintln("ERROR!"); } //Handle Error
                }

                match nfmt::i32_to_ns(gps.nmea.lat_min) {
                    Ok(s) => { console.scprintln("LAT MMmm = ", &s);}
                    Err(_e) => { console.sprintln("ERROR!"); } //Handle Error
                }

                console.scprintln("LAT NS = ", &[gps.nmea.lat_ns]);

                match nfmt::i32_to_ns(gps.nmea.long_deg) {
                    Ok(s) => { console.scprintln("LONG DD = ", &s);}
                    Err(_e) => { console.sprintln("ERROR!"); } //Handle Error
                }

                match nfmt::i32_to_ns(gps.nmea.long_min) {
                    Ok(s) => { console.scprintln("LONG MMmm = ", &s);}
                    Err(_e) => { console.sprintln("ERROR!"); } //Handle Error
                }

                console.scprintln("LAT NS = ", &[gps.nmea.long_we]);

                match nfmt::i32_to_ns(gps.nmea.alt) {
                    Ok(s) => { console.scprintln("ALT = ", &s);}
                    Err(_e) => { console.sprintln("ERROR!"); } //Handle Error
                }
            }
            Err(_e) => { 
                // Handle error here
                console.sprintln("[gps] - Ignoring packet!");
                gps.clear_errors();
            }
        }
        
        match nfmt::i32_to_ns(get_stime() as i32) {
            Ok(s) => { console.scprintln("ST = ", &s);}
            Err(_e) => { console.sprintln("ST ERROR!"); } //Handle Error
        }

    }
}

#[allow(dead_code)]
fn get_stime() -> i32 {
    let mut current_time: i32 = 0;
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
                if *st >= 2147483646 {
                    *st = 0;
                }
                else {
                    *st = *st + 1;
                }
            }
        }
    });
}
