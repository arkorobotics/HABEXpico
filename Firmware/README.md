## Build

1. Build command for debug
```
$ cargo build
```

2. Build command for release
```
$ cargo build --release
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

## Flash and Debug

IMPORTANT: You may experience odd behavior (during boot/operations) when using the `./flash.sh` script to program the MCU. Using the OpenOCD/GDB method will give much more consistent results. You must power both the debugger and target MCU to program the device. Once programmed, device will only boot in one of two modes: 
a) The debugger is connected and powered along with the target (powered separately via USB in this design). The device will not boot if the debugger is connected, but unpowered.
b) The target is powered and the debugger is completely disconnected from the device.

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

## Flash using St-Flash

1. Use the following command to flash the debug build:
    ```
    NOTE THIS MAY NOT FUNCTION CONSISTENTLY. THERE IS CURRENTLY AN UNRESOLVED ISSUE WITH THIS SCRIPT. USE THE OPENOCD/GDB FLASH AND DEBUG INSTRUCTIONS ABOVE.
    $ ./flash.sh target/thumbv6m-none-eabi/debug/habexpico
    ```

2. Use the following command to flash the release build:
    ```
    NOTE THIS MAY NOT FUNCTION CONSISTENTLY. THERE IS CURRENTLY AN UNRESOLVED ISSUE WITH THIS SCRIPT. USE THE OPENOCD/GDB FLASH AND DEBUG INSTRUCTIONS ABOVE.
    $ ./flash.sh
    ```
