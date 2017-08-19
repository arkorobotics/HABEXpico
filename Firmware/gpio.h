#ifndef GPIO_H
#define GPIO_H

#define INPUT 0
#define OUTPUT 1

#define LOW 0
#define HIGH 1

typedef struct 
{
	volatile uint8_t net;
	volatile uint8_t * ddr;
	volatile uint8_t * port;
	volatile uint8_t * pin;
} gpio;

// PORT B
extern gpio BO;
extern gpio B1;
extern gpio B2;
extern gpio B3;
extern gpio B4;
extern gpio B5;
extern gpio B6;
extern gpio B7;

// PORT C
extern gpio CO;
extern gpio C1;
extern gpio C2;
extern gpio C3;
extern gpio C4;
extern gpio C5;
extern gpio C6;

// PORT D
extern gpio DO;
extern gpio D1;
extern gpio D2;
extern gpio D3;
extern gpio D4;
extern gpio D5;
extern gpio D6;
extern gpio D7;

void gpio_pin_mode(gpio pin, uint8_t mode);
void gpio_pin_state(gpio pin, uint8_t pull);
void gpio_pin_set(gpio pin, uint8_t level);

#endif