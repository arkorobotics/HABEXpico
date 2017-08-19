#ifndef UART_H
#define UART_H

uint8_t uart_init(uint32_t baud);
void uart_tx(uint8_t data);
uint8_t uart_rx(void);
uint8_t uart_flush(void);

#endif