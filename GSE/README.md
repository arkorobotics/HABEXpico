# HABEXpico - Ground Support Equipment

## SDR Hardware

- HackRF Jawbreaker 
- 144Mhz Sawtooth Active Filter
- 140/433 Mhz Antenna
- USB to DC Barrel Adapter
- USB Micro A/B to USB A
- SMA Male to SMA Male

## Hardware Configuration

- Connect the HackRF to the Sawtooth Active Filter "OUT" (SMA to SMA)
- Connect the Sawtooth Active Filter "IN" to the Antenna (SMA to SMA)
- Connect the USB to DC Barrel Adapter to the Sawtooth Active Filter
- Connect the HackRF to the ground station computer via USB

## Software Configuration / Operation

### Terminal 1:
- Start GQRX
- Configure GQRX
    - Select HackRF as source
    - HackRF Settings
        - Frequency: 144.390 Mhz
        - RF Gain: 14 dB
        - IF Gain: 16 dB
        - BB Gain: 24 dB
    - Audio Settings
        - Sample Rate: 48,000 Hz
        - Audio Gain: 2dB (adjust as needed)
    - UDP Settings
        - IP Address: 127.0.0.1
        - Port: 7355
- Press the play button at the top left of the window
- Adjust the center frequency to align with transmissions
- Adjust the audio gain until you see packets and direwolf stops complaining that it's too loud.

### Terminal 2:
Decode APRS Packs
```
$ direwolf -r 48000 udp:7355
```