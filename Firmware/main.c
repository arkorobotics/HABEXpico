// HABEXpico 7
// By: Arko

// FUSE SETTINGS: EXTENDED 0xFF  FIRST, CHECK THESE BEFORE PROCEEDING, ERASE THIS LINE AFTERWARDS
//                HIGH     0xD8
//                LOW      0x62  (Internal 8Mhz osc) THIS NEEDS TO CHANGE TO 2Mhz for 1.8v!!!!!!

#include "main.h"
#include "gpio.h"
#include "i2c.h"
#include "spi.h"
#include "uart.h"

int main(void) 
{
  //uart_init(UART0_BAUD);
  //spi_master_init();

  gpio_pin_mode(GPS_EN, OUTPUT);
  gpio_pin_set(GPS_EN, LOW);

  while(1)
  {
    
    //gpio_pin_set(LED, HIGH);
    //_delay_ms(50); 
    //gpio_pin_set(LED, LOW);
    _delay_ms(50);

    //uart_tx(0x30);

    //spi_tx(0xAA);
  }

  return 0;
}
