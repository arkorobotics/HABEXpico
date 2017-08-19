#include "main.h"
#include "uart.h"

uint8_t uart_init(uint32_t baud) 
{
  uint16_t ubrr = FOSC/16/baud-1;

  UBRR0H = (uint8_t)(ubrr>>8);			// Set baud rate 
  UBRR0L = (uint8_t)ubrr;

  UCSR0B = (1<<RXEN0)|(1<<TXEN0);		// Enable receiver and transmitter 
  UCSR0C = (1<<USBS0)|(3<<UCSZ00);		// Set frame format: 8data, 2stop bit 

  return 1;
}

void uart_tx(uint8_t data) 
{
	while ( !( UCSR0A & (1<<UDRE0)) );	// Wait for empty transmit buffer
	UDR0 = data;						// Put data into buffer, sends the data
}

uint8_t uart_rx(void) 
{
	while ( !(UCSR0A & (1<<RXC0)) );	// Wait for data to be received 
	return UDR0;						// Get and return received data from buffer
}

uint8_t uart_flush(void) 
{
	uint8_t dummy = 0;
	while ( UCSR0A & (1<<RXC0) )
	{
		dummy = UDR0;
	}
	return dummy;
}