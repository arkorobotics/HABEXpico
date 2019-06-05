## Build

1. Build command
```
$ cargo build
```

## Flash

1. Generate Hex Filec
``` 
$ arm-none-eabi-objcopy -O ihex target/thumbv6m-none-eabi/debug/habexpico habexpico.hex
```

2. Flash command
``` 
$ st-flash --format ihex write habexpico.hex
```