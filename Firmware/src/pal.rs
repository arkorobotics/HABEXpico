// Peripheral Abstraction Layer

use stm32l0xx_hal::{pac, prelude::*, rcc::Config, adc, gpio, serial, spi, timer::*};

pub struct PAL {
    pub console_tx: serial::Tx<serial::USART2>,                     // Console TX
    pub console_rx: serial::Rx<serial::USART2>,                     // Console RX
    pub gps_tx: serial::Tx<serial::LPUART1>,                        // GPS TX
    pub gps_rx: serial::Rx<serial::LPUART1>,                        // GPS RX
    pub gps_en: gpio::gpioa::PA1<gpio::Output<gpio::PushPull>>,     // GPS Enable Pin
    pub radio_spi: spi::Spi< pac::SPI1, (gpio::gpioa::PA5<gpio::Analog>, // Radio SPI
                                        gpio::gpioa::PA6<gpio::Analog>, 
                                        gpio::gpioa::PA7<gpio::Analog>)>,   
    pub radio_nss: gpio::gpioa::PA4<gpio::Output<gpio::PushPull>>,  // Radio Chip Select Pin
    pub adc: adc::Adc<adc::Ready>,                                    // Analog To Digital Converter
    pub adc_vstore: gpio::gpioa::PA0<gpio::Analog>,                 // Storage Voltage
    pub timer: Timer<pac::TIM2>,
}

impl PAL {
    pub fn new() -> Self {
        // Create Peripheral Struct
        let dp = pac::Peripherals::take().unwrap();

        // Configure the clock
        let mut rcc = dp.RCC.freeze(Config::hsi16());

        // Acquire the GPIOA peripheral
        let gpioa = dp.GPIOA.split(&mut rcc);

        // Configure Debug Serial Pins
        let debug_tx_pin = gpioa.pa9;
        let debug_rx_pin = gpioa.pa10;

        // //Configure the debug serial peripheral
        let debug_serial = dp
            .USART2
            .usart(debug_tx_pin, debug_rx_pin, 
                serial::Config {
                    baudrate: 115_200_u32.bps(),
                    wordlength: serial::WordLength::DataBits8,
                    parity: serial::Parity::ParityNone,
                    stopbits: serial::StopBits::STOP1,
                }, 
                &mut rcc)
            .unwrap();

        let (console_tx, console_rx) = debug_serial.split();
        
        // Configure GPS
        let gps_en = gpioa.pa1.into_push_pull_output();

        // Configure GPS Serial Pins
        let gps_tx_pin = gpioa.pa2;
        let gps_rx_pin = gpioa.pa3;

        // Configure the GPS serial peripheral
        let gps_serial = dp
            .LPUART1
            .usart(gps_tx_pin, gps_rx_pin, 
                serial::Config {
                    baudrate: 9_600_u32.bps(),
                    wordlength: serial::WordLength::DataBits8,
                    parity: serial::Parity::ParityNone,
                    stopbits: serial::StopBits::STOP1,
                },
                &mut rcc)
            .unwrap();

        let (gps_tx, gps_rx) = gps_serial.split();

        // Initialize SPI
        let radio_nss = gpioa.pa4.into_push_pull_output();
        let sck = gpioa.pa5;
        let miso = gpioa.pa6;
        let mosi = gpioa.pa7;

        // Initialise the SPI peripheral.
        let radio_spi = dp
            .SPI1
            .spi((sck, miso, mosi), spi::MODE_0, 100_000.hz(), &mut rcc);

        
        // Configure ADC
        let adc = dp.ADC.constrain(&mut rcc);

        // Configure PA0 as analog.
        let adc_vstore = gpioa.pa0.into_analog();

        // Configure the timer.
        let timer = dp.TIM2.timer(1.hz(), &mut rcc);

        return PAL {
            console_tx: console_tx,
            console_rx: console_rx,
            gps_tx: gps_tx,
            gps_rx: gps_rx,
            gps_en: gps_en,
            radio_spi: radio_spi,
            radio_nss: radio_nss,
            adc: adc,
            adc_vstore: adc_vstore,
            timer: timer
        }
    }
}


