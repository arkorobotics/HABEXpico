## Build

1. Build command
```
$ cargo build
```

## Dependecies for flashing with UART bootloader

1. Download and install the latest `stm32flash`:
   
   https://sourceforge.net/p/stm32flash/code/ci/master/tree/

## Flash with UART bootloader

1. Press and hold the RESET and BOOT0 button, release the RESET button, then release the BOOT0 button

2. Program the MCU using the following command:
    ```
    $ arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/debug/habexpico program.bin
    $ stm32flash -e 0 -w program.bin -v -g 0x0 /dev/tty.usbserial-A601L0OF
    ```

## Flash using St-Flash

1. Use the following command:
    ```
    $ ./flash.sh target/thumbv6m-none-eabi/debug/habexpico
    ```

## Flash and Debug

1. Terminal 1 - OpenOCD Session:
    ``` 
    $ ./openocd_session.sh
    ```

2. Terminal 2 - Dashboard:
    ``` 
    $ tty
    ```
    Note the tty session. We will use this in the following steps. (i.e. "/dev/ttys001")

3. Terminal 3 - GDB Py Session:
    ``` 
    $ ./gdb_session.sh target/thumbv6m-none-eabi/debug/habexpico -d
    ```
