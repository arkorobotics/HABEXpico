#![deny(warnings)]
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

use stm32l0xx_hal::{pac::{self, interrupt, Interrupt}, timer::Timer};

use core::ops::DerefMut;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;

static TIMER: Mutex<RefCell<Option<Timer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));
static ST: Mutex<RefCell<Option<u16>>> = Mutex::new(RefCell::new(None));

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
    
    console.print_char(gps.get_packet());

    let mut timer: Timer<pac::TIM2> = pal.timer;

    timer.listen();
    
    let mut st: u16 = 0;
    st = st + 1;
    
    // Store timer in mutex refcells to make them available from the
    // timer interrupt.
    cortex_m::interrupt::free(|cs| {
        *ST.borrow(cs).borrow_mut() = Some(st);
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
    });
    
   
    // Enable the timer interrupt in the NVIC.
    unsafe { NVIC::unmask(Interrupt::TIM2); }

    
    loop {
        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut st) = ST.borrow(cs).borrow_mut().deref_mut() {    
                console.cprint_telem("Time=", *st);
            }
        });
    }
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