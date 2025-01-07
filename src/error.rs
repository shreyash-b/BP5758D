use embedded_hal::i2c::Error as I2cError;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
/// Error types
pub enum Error {
    /// I2C operation error as defined by [`embedded_hal::i2c::Error`]
    #[error("i2c driver error")]
    I2c,
    /// Invalid argument passed to the method
    #[error("invalid argument")]
    InvalidArg,
}

impl<T: I2cError> From<T> for Error {
    fn from(_value: T) -> Self {
        Self::I2c
    }
}
