use embedded_hal::i2c::Error as I2cError;
use thiserror::Error;

#[derive(Error, Debug)]
/// Error types
pub enum Error<T: I2cError> {
    /// I2C operation error as defined by [`embedded_hal::i2c::Error`]
    #[error("i2c driver error")]
    I2c(T),
    /// Invalid argument passed to the method
    #[error("invalid argument")]
    InvalidArg,
}

impl<T: I2cError> From<T> for Error<T> {
    fn from(value: T) -> Self {
        Self::I2c(value)
    }
}
