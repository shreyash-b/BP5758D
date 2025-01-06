# Rust implementation of BP5758d LED Driver

## BP5758d
- BP5758d is a LED driver IC which can be used for building smart lighting applications.
- It allows controlling 5 channels of LEDs with individual output current control and 10-bit level control.  

More capabilities and details about it can be found in the [datasheet](docs/BP5758D_EN_DS_Rev.1.1.pdf)

---

This crate implements the controlling for BP5758d in using Rust applications.

Features:   
[x] Current and sleep mode configuration.
[x] Can associate individual color to each channel.
[x] Control each channel individually or collectively.
[x] Can be used with any device that implements I2c traits from embedded-hal.
[x] no-alloc and no-std compatible.