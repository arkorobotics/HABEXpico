#ifndef I2C_H
#define I2C_H

void i2c_init(uint32_t i2c_freq);
void i2c_start();
void i2c_stop(void);
void i2c_write(unsigned char dat);
unsigned char i2c_read(unsigned char ack);

#endif