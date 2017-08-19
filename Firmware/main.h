#ifndef MAIN_H
#define MAIN_H

#include <inttypes.h>
#include <avr/io.h>
#include <avr/interrupt.h>
#include <avr/sleep.h>
#include <util/delay.h>
#include <stdint.h>

#define FOSC       	1000000UL
#define UART0_BAUD 	9600UL
#define I2C_FREQ 	100000UL

// PIN DEFINITIONS
#define GPS_EN		B1
#define RADIO_SS	B2
#define MOSI	 	PB3
#define MISO	 	PB4
#define SCK		  	PB5

#define VBATT 		CO
#define VPANEL 		C1

#define GPS_TX 		D0
#define GPS_RX 		D1
#define NIRQ 		D2
#define AFSK 		D3
#define RADIO_EN 	D5
#define GPIO1 		D6

#define LED 		B5

int main(void);

#endif