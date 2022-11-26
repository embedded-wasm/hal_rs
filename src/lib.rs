
use embedded_hal::{
    spi::{ErrorKind as SpiError},
    i2c::{ErrorKind as I2cError},
    serial::{ErrorKind as UartError},
};

pub mod spi;

use api::HandleArray;
pub use spi::Spi;

pub mod i2c;
pub use i2c::I2c;

pub mod gpio;
pub use gpio::{Gpio, Input, Output};

pub mod uart;
pub use uart::Uart;

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
#[derive(PartialEq, Debug)]
pub struct Device<'a> {
    pub spi: &'a mut [Spi],
    pub i2c: &'a mut [I2c],
    pub io_in: &'a mut [Gpio<Input>],
    pub io_out: &'a mut [Gpio<Output>],
}


pub(crate) mod api {
    use super::*;

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct HandleArray<T> {
        pub ptr: *mut T,
        pub len: u32,
    }
    
    #[link(wasm_import_module = "device")]
    extern {
        pub fn spi(spi: *mut HandleArray<Spi>) -> i32;

        pub fn i2c(i2c: *mut HandleArray<I2c>) -> i32;

        pub fn gpio_in(io: *mut HandleArray<Gpio<Input>>) -> i32;

        pub fn gpio_out(io: *mut HandleArray<Gpio<Output>>) -> i32;
    }
}

impl <'a> Device<'a> {

    pub fn load(&mut self) -> Result<(), Error> {

        let mut h = HandleArray{ptr: self.spi.as_mut_ptr(), len: self.spi.len() as u32};
        let res = unsafe { api::spi(&mut h) };
        if res < 0 {
            return Err(Error::Runtime(res));
        }

        let mut h = HandleArray{ptr: self.i2c.as_mut_ptr(), len: self.i2c.len() as u32};
        let res = unsafe { api::i2c(&mut h) };
        if res < 0 {
            return Err(Error::Runtime(res));
        }

        let mut h = HandleArray{ptr: self.io_in.as_mut_ptr(), len: self.io_in.len() as u32};
        let res = unsafe { api::gpio_in(&mut h) };
        if res < 0 {
            return Err(Error::Runtime(res));
        }

        let mut h = HandleArray{ptr: self.io_out.as_mut_ptr(), len: self.io_out.len() as u32};
        let res = unsafe { api::gpio_out(&mut h) };
        if res < 0 {
            return Err(Error::Runtime(res));
        }
        
        Ok(())
    }

}