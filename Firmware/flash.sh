arm-none-eabi-objcopy -O ihex target/thumbv6m-none-eabi/debug/habexpico habexpico.hex
st-flash --format ihex write habexpico.hex