//! BP5758d LED Driver

#![no_std]
#![allow(dead_code)]

mod error;
mod fmt;

use embedded_hal::i2c::I2c;
pub use error::{Error, Result};

const ADDR_BASE: u8 = 0x80;

const SLEEP_ENABLE_MASK: u8 = 0;
const SLEEP_DISABLE_MASK: u8 = 0x20;

const OUT_ENABLE_MASK: u8 = 0;

const OUT1_CURRENT_MASK: u8 = 0x1;
const OUT2_CURRENT_MASK: u8 = 0x2;
const OUT3_CURRENT_MASK: u8 = 0x3;
const OUT4_CURRENT_MASK: u8 = 0x4;
const OUT5_CURRENT_MASK: u8 = 0x5;

const OUT1_GRAYSCALE_MASK: u8 = 0x6;
const OUT2_GRAYSCALE_MASK: u8 = 0x8;
const OUT3_GRAYSCALE_MASK: u8 = 0xA;
const OUT4_GRAYSCALE_MASK: u8 = 0xC;
const OUT5_GRAYSCALE_MASK: u8 = 0xE;

const OUT_ALL_ENABLE: u8 = 0x1F;
const OUT_ALL_DISABLE: u8 = 0x00;

/// Bp5758d channels for performing operations
pub enum Bp5758dChannel {
    OUT1,
    OUT2,
    OUT3,
    OUT4,
    OUT5,
}

impl Bp5758dChannel {
    const fn get_grayscale_mask(&self) -> u8 {
        match self {
            Bp5758dChannel::OUT1 => OUT1_GRAYSCALE_MASK,
            Bp5758dChannel::OUT2 => OUT2_GRAYSCALE_MASK,
            Bp5758dChannel::OUT3 => OUT3_GRAYSCALE_MASK,
            Bp5758dChannel::OUT4 => OUT4_GRAYSCALE_MASK,
            Bp5758dChannel::OUT5 => OUT5_GRAYSCALE_MASK,
        }
    }
}

/// Struct representing BP5758d driver controller.
pub struct Bp5758d<T: I2c> {
    sleeping: bool,
    mapping: [u8; 5], // rgbcw channels
    max_current: [u8; 5],
    i2c: T,
}

impl<T: I2c> Bp5758d<T> {
    /// Initializes Bp5758d driver
    ///
    /// Arguments:
    ///
    /// * `driver` : I2C driver implementing [`embedded_hal::i2c::I2c`] trait
    /// * `channel_mapping` : driver output channels (OUT1~OUT5) for colors r,g,b,c,w respectively
    /// * `max_current` : current settings for the 5 channels respectively
    pub fn new(driver: T, channel_mapping: [u8; 5], mut max_current: [u8; 5]) -> Result<Self> {
        for i in channel_mapping {
            if i > 5 {
                return Err(Error::InvalidArg);
            }
        }

        validate_transform_current(&mut max_current)?;

        Ok(Self {
            sleeping: true,
            i2c: driver,
            mapping: channel_mapping,
            max_current,
        })
    }

    /// Set grayscale value for specified channel
    ///
    /// It will disable sleep mode, if enabled previously
    pub fn set_channel(&mut self, channel: Bp5758dChannel, value: u16) -> Result<()> {
        if value >= 1024 {
            return Err(Error::InvalidArg);
        }

        let mut addr = ADDR_BASE | SLEEP_DISABLE_MASK;
        let mut data = [0; 2];

        addr |= channel.get_grayscale_mask();

        data[0] = (value & 0x1F) as u8;
        data[1] = (value >> 5) as u8;

        if self.sleeping {
            self.set_sleep(false)?;
        }

        self.write(addr, &data)?;
        Ok(())
    }

    /// Set values for all the 5 channels simultaneously
    ///
    /// It will diable sleep mode, if enabled previously
    pub fn set_rgbcw(&mut self, r: u16, g: u16, b: u16, c: u16, w: u16) -> Result<()> {
        let addr = ADDR_BASE | SLEEP_DISABLE_MASK | OUT1_GRAYSCALE_MASK;
        let mut data = [0; 10];

        let values = [r, g, b, c, w];

        for (i, value) in values.iter().enumerate() {
            if *value >= 1024 {
                return Err(Error::InvalidArg);
            }

            let chann = self.mapping[i] as usize - 1;

            data[2 * chann] = (value & 0x1F) as u8;
            data[2 * chann + 1] = (value >> 5) as u8;
        }

        if self.sleeping {
            self.set_sleep(false)?;
        }

        self.write(addr, &data)?;
        Ok(())
    }

    /// Enable/Disable sleep mode
    ///
    /// All channel grayscale values will be set to 0 sleep is enabled.
    /// Grayscale value is not restored when sleep is disabled.
    pub fn set_sleep(&mut self, sleep: bool) -> Result<()> {
        let mut addr = ADDR_BASE | OUT_ENABLE_MASK;
        let mut data = [0; 6];

        if sleep {
            addr |= SLEEP_ENABLE_MASK;
            data[0] = OUT_ALL_DISABLE;
            self.set_shutdown()?;
            self.write(addr, &data)?;
            self.sleeping = true;
        } else {
            addr |= SLEEP_DISABLE_MASK;
            data[0] = OUT_ALL_ENABLE;
            data[1..6].copy_from_slice(&self.max_current);
            self.write(addr, &data)?;
            self.sleeping = false;
        }

        Ok(())
    }

    fn set_shutdown(&mut self) -> Result<()> {
        let addr = ADDR_BASE | SLEEP_DISABLE_MASK | OUT1_GRAYSCALE_MASK;
        let data = [0; 10];

        self.write(addr, &data)?;

        Ok(())
    }

    #[inline(always)]
    fn write(&mut self, addr: u8, data: &[u8]) -> Result<()> {
        info!("writing: {:x?} at {:x}", data, addr);
        self.i2c.write(addr, data)?;

        Ok(())
    }
}

impl<T: I2c> Drop for Bp5758d<T> {
    /// Destructor for [`Bp5758d`]
    ///
    /// Will set driver in sleep mode when struct is dropped.
    fn drop(&mut self) {
        if !self.sleeping {
            if let Err(err) = self.set_sleep(true) {
                error!("Failed to set sleep: {}", err);
            }
        }
    }
}

fn validate_transform_current(values: &mut [u8; 5]) -> Result<()> {
    for i in values {
        if *i > 90 {
            return Err(Error::InvalidArg);
        }

        if *i > 64 {
            *i -= 62;
            *i |= 0x60;
        }
    }

    Ok(())
}
