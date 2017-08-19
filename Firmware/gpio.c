#include "main.h"
#include "gpio.h"

// PORT B
gpio BO = {.net = PB0, .ddr = &DDRB, .port = &PORTB, .pin = &PINB};
gpio B1 = {.net = PB1, .ddr = &DDRB, .port = &PORTB, .pin = &PINB};
gpio B2 = {.net = PB2, .ddr = &DDRB, .port = &PORTB, .pin = &PINB};
gpio B3 = {.net = PB3, .ddr = &DDRB, .port = &PORTB, .pin = &PINB};
gpio B4 = {.net = PB4, .ddr = &DDRB, .port = &PORTB, .pin = &PINB};
gpio B5 = {.net = PB5, .ddr = &DDRB, .port = &PORTB, .pin = &PINB};
gpio B6 = {.net = PB6, .ddr = &DDRB, .port = &PORTB, .pin = &PINB};
gpio B7 = {.net = PB7, .ddr = &DDRB, .port = &PORTB, .pin = &PINB};

// PORT C
gpio CO = {.net = PC0, .ddr = &DDRC, .port = &PORTC, .pin = &PINC};
gpio C1 = {.net = PC1, .ddr = &DDRC, .port = &PORTC, .pin = &PINC};
gpio C2 = {.net = PC2, .ddr = &DDRC, .port = &PORTC, .pin = &PINC};
gpio C3 = {.net = PC3, .ddr = &DDRC, .port = &PORTC, .pin = &PINC};
gpio C4 = {.net = PC4, .ddr = &DDRC, .port = &PORTC, .pin = &PINC};
gpio C5 = {.net = PC5, .ddr = &DDRC, .port = &PORTC, .pin = &PINC};
gpio C6 = {.net = PC6, .ddr = &DDRC, .port = &PORTC, .pin = &PINC};

// PORT D
gpio DO = {.net = PD0, .ddr = &DDRD, .port = &PORTD, .pin = &PIND};
gpio D1 = {.net = PD1, .ddr = &DDRD, .port = &PORTD, .pin = &PIND};
gpio D2 = {.net = PD2, .ddr = &DDRD, .port = &PORTD, .pin = &PIND};
gpio D3 = {.net = PD3, .ddr = &DDRD, .port = &PORTD, .pin = &PIND};
gpio D4 = {.net = PD4, .ddr = &DDRD, .port = &PORTD, .pin = &PIND};
gpio D5 = {.net = PD5, .ddr = &DDRD, .port = &PORTD, .pin = &PIND};
gpio D6 = {.net = PD6, .ddr = &DDRD, .port = &PORTD, .pin = &PIND};
gpio D7 = {.net = PD7, .ddr = &DDRD, .port = &PORTD, .pin = &PIND};

void gpio_pin_mode(gpio pin, uint8_t mode)
{
	*pin.ddr =  mode ? (*pin.ddr | (1 << pin.net)) : (*pin.ddr & ~(1 << pin.net));
}

void gpio_pin_state(gpio pin, uint8_t state)
{
	// Needs to be revised
	//*pin.pin =  state ? (*pin.pin | (1 << pin.net)) : (*pin.pin & ~(1 << pin.net));
}

void gpio_pin_set(gpio pin, uint8_t level)
{
	*pin.port =  level ? (*pin.port | (1 << pin.net)) : (*pin.port & ~(1 << pin.net));
}
