#ifndef SPI_H
#define SPI_H

void spi_master_init(void);
void spi_tx(uint8_t cData); 
uint8_t spi_rx(void);
uint8_t spi_tx_rx(uint8_t data);

void spi_slave_init(void);  

#endif