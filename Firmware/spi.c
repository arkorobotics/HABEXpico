#include "main.h"
#include "spi.h"

void spi_master_init(void) 
{
	DDRB |= (1<<MOSI)|(1<<SCK);				// Set MOSI and SCK output, all others input
	SPCR = (1<<SPE)|(1<<MSTR)|(1<<SPR0);	// Enable SPI, Master, set clock rate fck/16 (500kHz @ FOSC=8MHz)
    sei();									// Enable Global Interrupts
}

void spi_tx(uint8_t cData) 
{
	SPDR = cData;							// Start transmission
	while(!(SPSR & (1<<SPIF))); 			// Wait for transmission complete
}

uint8_t spi_rx(void) 
{
	while(!(SPSR & (1<<SPIF)));				// Wait for reception complete
	return SPDR;							// Return Data Register
}

uint8_t spi_tx_rx(uint8_t data)
{
    SPDR = data;							// Load data into the buffer
    while(!(SPSR & (1<<SPIF) ));			// Wait until transmission complete
    return(SPDR);							// Return received data
}

void spi_slave_init(void) 
{
	DDRB |= (1<<MISO);						// Set MISO output, all others input
	SPCR = (1<<SPE);						// Enable SPI
}
