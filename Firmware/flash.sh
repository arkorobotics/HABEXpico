#!/bin/sh
if (( $# == 1)); then
    arm-none-eabi-objcopy -O ihex "$1" program.hex
    st-flash --format ihex write program.hex
else
    arm-none-eabi-objcopy -O ihex "target/thumbv6m-none-eabi/release/habexpico" program.hex
    st-flash --format ihex write program.hex
fi