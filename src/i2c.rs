
use embedded_hal::i2c::blocking::{Read, Write, WriteRead};

use crate::Error;
use self::api::{RBytes, WBytes};

// WASM function calls
// TODO: replace with WITX generated ones when viable?
mod api {
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct WBytes {
        pub ptr: *mut u8,
        pub len: u32,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct RBytes {
        pub ptr: *const u8,
        pub len: u32,
    }

    #[link(wasm_import_module = "i2c")]
    extern {
        pub fn init(dev: u32, baud: u32, sda: i32, scl: i32, handle: &mut i32) -> i32;

        /// Deinitialise the specified I2c peripheral
        pub fn deinit(handle: i32) -> i32;

        /// Write to an I2c device
        pub fn write(handle: i32, addr: u16, data: &RBytes) -> i32;

        /// Read from an I2c device
        pub fn read(handle: i32, addr: u16, buff: &WBytes) -> i32;

        /// Write to and read from an I2c device on the specified peripheral
        pub fn write_read(handle: i32, addr: u16, data: &RBytes, buff: &mut WBytes) -> i32;
    }
}

/// I2c device instance
#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct I2c {
    pub(crate) handle: i32,
}

impl Default for I2c {
    fn default() -> Self {
        Self { handle: -1 }
    }
}


impl I2c {
    /// Initialise an I2c device
    pub fn init(dev: u32, baud: u32, sda: i32, scl: i32) -> Result<Self, Error> {
        println!("Initialising I2C device: {}", dev);

        let mut handle = 0;
        let res = unsafe { api::init(dev, baud, sda, scl, &mut handle) };
        if res < 0 {
            return Err(Error::Runtime(res))
        }
        
        println!("Received I2C handle: {}", handle);

        Ok(Self{handle})
    }

    /// De-initialise an I2c device
    pub fn deinit(&mut self) -> Result<(), Error> {
        let _res = unsafe { api::deinit(self.handle) };
        // TODO: check res

        Ok(())
    }
}

impl Write for I2c {
    type Error = Error;

    fn write(&mut self, address: u8, data: &[u8]) -> Result<(), Self::Error> {

        let b = RBytes{
            ptr: data.as_ptr(),
            len: data.len() as u32,
        };

        let res = unsafe { api::write(self.handle, address as u16, &b) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(())
    }
}

impl Read for I2c {
    type Error = Error;

    fn read<'w>(&mut self, address: u8, data: &'w mut [u8]) -> Result<(), Self::Error> {

        let mut b = WBytes{
            ptr: data.as_mut_ptr(),
            len: data.len() as u32,
        };

        let res = unsafe { api::read(self.handle, address as u16, &mut b) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(())
    }
}

impl WriteRead for I2c {
    type Error = Error;

    fn write_read<'w>(&mut self, address: u8, data: &'w [u8], buff: &'w mut [u8]) -> Result<(), Self::Error> {
        //let d = data.as_ptr();
        //let l = data.len() as u32;

        let d = RBytes{
            ptr: data.as_ptr(),
            len: data.len() as u32,
        };

        let mut b = WBytes{
            ptr: buff.as_mut_ptr(),
            len: buff.len() as u32,
        };

        let res = unsafe { api::write_read(self.handle, address as u16, &d, &mut b) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(())
    }
}
