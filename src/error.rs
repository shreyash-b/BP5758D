use embedded_hal::i2c::Error as I2cError;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("i2c driver error")]
    I2c,
    #[error("invalid argument")]
    InvalidArg,
}

impl<T: I2cError> From<T> for Error {
    fn from(_value: T) -> Self {
        Self::I2c
    }
}
