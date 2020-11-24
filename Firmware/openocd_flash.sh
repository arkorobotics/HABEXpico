#!/bin/sh
if (( $# != 1 )); then
        echo "Usage:"
        echo "$0 <filename of firmware in ELF format>"
        exit 1
fi

/usr/local/opt/gnu-mcu-eclipse/openocd/0.10.0-12-20190422-2015/bin/openocd -f openocd.cfg -c "program $1 verify reset exit"
