#!/bin/sh
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/debug/habexpico program.bin
stm32flash -e 0 -w program.bin -v -g 0x0 /dev/tty.usbserial-A601L0OF