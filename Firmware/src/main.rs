#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config, serial};

use core::fmt::Write;
use nb::block;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure Debug Serial Pins
    let debug_tx_pin = gpioa.pa9;
    let debug_rx_pin = gpioa.pa10;

    // Configure GPS
    let mut gps_en_pin = gpioa.pa1.into_push_pull_output();

    // Configure GPS Serial Pins
    let gps_tx_pin = gpioa.pa2;
    let gps_rx_pin = gpioa.pa3;

    // Enable GPS Power
    let _gps_err = gps_en_pin.set_low();

    // //Configure the debug serial peripheral
    let debug_serial = dp
       .USART2
       .usart((debug_tx_pin, debug_rx_pin), serial::Config::default(), &mut rcc)
       .unwrap();

    let (mut debug_tx, mut _debug_rx) = debug_serial.split();

    writeln!(debug_tx, "- - - HABEX - - -\r").unwrap();
    writeln!(debug_tx, "Serial Debug: Configuration complete.\r").unwrap();

    // Configure the GPS serial peripheral
    let gps_serial = dp
        .LPUART1
        .usart((gps_tx_pin, gps_rx_pin), serial::Config::default(), &mut rcc)
        .unwrap();

    let (mut _gps_tx, mut gps_rx) = gps_serial.split();

    writeln!(debug_tx, "GPS: Configuration complete.\r").unwrap();
    writeln!(debug_tx, "MAIN: Entering main loop.\r").unwrap();

    loop {
        // Echo what is received on the serial link
        let received = block!(gps_rx.read()).unwrap();
        block!(debug_tx.write(received)).ok();
    }
}
