# HABEXpico
Small High Altitude Balloon Payload

# Build Firmware
See /Firmware/README.md

# Run Test Suite
```
$ cd Testscripts
$ cargo test -- --nocapture
```

# TODO

- Write NMEA Parser and GPS lib
- Test GPS lib using test script
- Write Radio lib
- Test Radio lib using test script
- Write Power Management lib
- Test Power Management lib using test scripts
- Write C&DH + state machine
- Test C&DH + state machine
- Bonus: Add VOR rx/triangulation with GPS ground truth and/or fallback (going to need a lot more memory)
