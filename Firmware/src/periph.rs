#![deny(warnings)]
#![deny(unsafe_code)]

use stm32l0xx_hal::{pac, prelude::*, rcc::Config, serial}; //, spi};

//use core::fmt::Write;
//use nb::block;

pub fn setup() -> ( serial::Tx<serial::USART2>, // Debug TX
                    serial::Rx<serial::USART2>  // Debug RX
                  ) {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure Debug Serial Pins
    let debug_tx_pin = gpioa.pa9;
    let debug_rx_pin = gpioa.pa10;

    // Configure GPS
    //let mut gps_en_pin = gpioa.pa1.into_push_pull_output();

    // Configure GPS Serial Pins
    //let gps_tx_pin = gpioa.pa2;
    //let gps_rx_pin = gpioa.pa3;

    // Enable GPS Power
    //let _gps_err = gps_en_pin.set_low();

    // //Configure the debug serial peripheral
    let debug_serial = dp
       .USART2
       .usart((debug_tx_pin, debug_rx_pin), serial::Config::default(), &mut rcc)
       .unwrap();

    let (debug_tx, debug_rx) = debug_serial.split();

    return (debug_tx, debug_rx)
    
    /*
    writeln!(debug_tx, "- - - - HABEX - - - -\r").unwrap();
    writeln!(debug_tx, "DEBUG: Serial port configuration complete.\r").unwrap();
    
    // Configure the GPS serial peripheral
    let gps_serial = dp
        .LPUART1
        .usart((gps_tx_pin, gps_rx_pin), serial::Config::default(), &mut rcc)
        .unwrap();

    let (mut _gps_tx, mut gps_rx) = gps_serial.split();

    writeln!(debug_tx, "GPS: Serial port configuration complete.\r").unwrap();

    let mut nss = gpioa.pa4.into_push_pull_output();
    let sck = gpioa.pa5;
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7;

    // Initialise the SPI peripheral.
    let mut spi = dp
        .SPI1
        .spi((sck, miso, mosi), spi::MODE_0, 100_000.hz(), &mut rcc);

    writeln!(debug_tx, "RADIO: SPI configuration complete...\r").unwrap();

    let mut adc = dp.ADC.constrain(&mut rcc);

    // Configure PA0 as analog.
    let mut vstore_pin = gpioa.pa0.into_analog();

    writeln!(debug_tx, "ADC: Analog configuration complete.\r").unwrap();

    writeln!(debug_tx, "MAIN: Entering main loop.\r").unwrap();
    */
}