# HABEXpico - Rust Firmware

## Installation

1. Download and install the arm-none-eabi gcc toolchain

https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads
We recommend installing the precompiled binaries to '/usr/local'. 
Add the bin folders (/bin & /arm-none-eabi/bin) to your environments variable 'PATH'.

2. Install STLink Tool

https://github.com/texane/stlink

3. Install OpenOCD

`$ brew update openocd`

4. Install Rust & Cargo

`$ curl https://sh.rustup.rs -sSf | sh`

5. Set toolchain to default nightly build

`$ rustup override set nightly-2018-08-28`

6. Install Rust Sources (needed?)
`
$ cargo install svd2rust
$ cargo add stm32l0
`

7. Set Target (needed?)
`
 $ rustup target add thumbv6m-none-eabi
`

## Build

1. Build command

`cargo build --release --verbose`

## Flash

1. Generate Hex File
`arm-none-eabi-objcopy -O ihex target/thumbv6m-none-eabi/release/habexpico habexpico.ihex`

2. Flash command
`st-flash --format ihex write habexpico.ihex`

## Credits

Base toolchain:  https://github.com/therealprof/stm32f042

USB HID Library: https://github.com/Determinant/bluepill-momo2