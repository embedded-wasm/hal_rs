

use embedded_hal::{
    spi::{ErrorKind as SpiError},
    i2c::{ErrorKind as I2cError},
    serial::{ErrorKind as UartError},
};

/// WASM API Error types
#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    Runtime(i32),
    Spi(SpiError),
    I2c(I2cError),
    Uart(UartError),
    Unimplemented,
}

impl embedded_hal::spi::Error for Error {
    fn kind(&self) -> SpiError {
        match self {
            Error::Spi(e) => *e,
            _ => SpiError::Other,
        }
    }
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> I2cError {
        match self {
            Error::I2c(e) => *e,
            _ => I2cError::Other,
        }
    }
}

impl embedded_hal::serial::Error for Error {
    fn kind(&self) -> UartError {
        match self {
            Error::Uart(e) => *e,
            _ => UartError::Other,
        }
    }
}

