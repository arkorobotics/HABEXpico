# Dependecies for building and flashing
0. Install rustup and set target
    Install rustup by following the instructions here https://rustup.rs then run the following commands:
    ```
    $ rustup target add thumbv6m-none-eabi
    ```

1. Download and install the arm-none-eabi toolchain

	https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads
	We recommend installing the precompiled binaries to '/usr/local/opt'. 
	Add the bin folders (/bin & /arm-none-eabi/bin) to your environments variable 'PATH'.

    On Linux:
    ```
    $ sudo apt-get update
    $ sudo apt-get install gcc-arm-none-eabi
    ```

2. Install STLink Tool (>=v1.5.1)

	https://github.com/texane/stlink

3. Install OpenOCD (OPTIONAL)

    NOTE: OpenOCD v0.10.0 does not fully support the stm32l0 family MCU. We recommend using `gnu-mcu-eclipse/openocd` instead:

    https://gnu-mcu-eclipse.github.io/openocd/install/
    We recommend installing the precompiled binaries to '/usr/local/opt'. 
	Add the bin folders (i.e. - /usr/local/opt/gnu-mcu-eclipse/openocd/0.10.0-12-20190422-2015/bin) to your environments variable 'PATH'.

4. Install GDB Dashboard (OPTIONAL)

	https://github.com/cyrus-and/gdb-dashboard

5. Install VSCode and the "cortex-debug" extension (OPTIONAL)

6. Download and install the latest `stm32flash`: (OPTIONAL)
   https://sourceforge.net/p/stm32flash/code/ci/master/tree/

## Build

1. Build command for debug
```
$ cargo build
```

2. Build command for release
```
$ cargo build --release
```

## Flash and debug using OpenOCD

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

## Flash and Debug using VSCode

1. Open HABEXpico/Firmware folder in VSCode

2. Run->Start Debugging (F5)

## Flash-only using St-Flash

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

## Flash with UART bootloader

1. Press and hold the RESET and BOOT0 button, release the RESET button, then release the BOOT0 button

2. Program the MCU using the following command:
    ```
    $ arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/debug/habexpico program.bin
    $ stm32flash -e 0 -w program.bin -v -g 0x0 /dev/tty.usbserial-A601L0OF
    ```