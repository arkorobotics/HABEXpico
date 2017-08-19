#include "main.h"
#include "i2c.h"

void i2c_init(uint32_t i2c_freq)
{
	TWSR = 0x00; 					//set prescaler bits to zero (prescaler = 1)
	TWBR = ((FOSC/i2c_freq)-16)/2; 	//SCL frequency is 100K for XTAL = 7.3728M
	TWCR = (1<<TWEN); 				//enab1e TWI module
}

void i2c_start()
{
	TWCR = ((1<<TWINT) | (1<<TWSTA) | (1<<TWEN));
	while (!(TWCR & (1<<TWINT)));
}

void i2c_stop(void)
{
	TWCR = ((1<< TWINT) | (1<<TWEN) | (1<<TWSTO));
	_delay_us(10); 
}

void i2c_write(unsigned char dat)
{
	TWDR = dat ;
	TWCR = ((1<< TWINT) | (1<<TWEN));
	while (!(TWCR & (1 <<TWINT)));
}

unsigned char i2c_read(unsigned char ack)
{
	TWCR = ((1<< TWINT) | (1<<TWEN) | (ack<<TWEA));
	while ( !(TWCR & (1 <<TWINT)));
	return TWDR;
}