## Build

1. Build command
```
$ cargo build
```

## Flash

1. Generate and Flash Hex File
``` 
$ ./flash.sh
```

## Debug

Terminal 1 - OpenOCD Session:
    ``` 
    $ ./openocd_session.sh
    ```

Terminal 2 - Dashboard:
    ``` 
    $ tty
    ```
    Note the tty session. We will use this in the following steps. (i.e. "/dev/ttys001")

Terminal 3 - GDB Py Session:
    ``` 
    $ ./gdb_session.sh target/thumbv6m-none-eabi/release/habexpico -d
    >>> dashboard -output /dev/ttys001
    ```
