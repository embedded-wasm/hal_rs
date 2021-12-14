
use embedded_hal::spi::blocking::{Transfer, Write};

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

    #[link(wasm_import_module = "spi")]
    extern {
        pub fn init(dev: u32, baud: u32, mosi: i32, miso: i32, sck: i32, cs: i32, handle: &mut i32) -> i32;

        /// Deinitialise the specified SPI peripheral
        pub fn deinit(handle: i32) -> i32;

        /// Write to an SPI device on the specified peripheral
        pub fn write(handle: i32, data: &RBytes) -> i32;

        /// Write to and read from an SPI device on the specified peripheral
        pub fn transfer(handle: i32, data: &mut WBytes) -> i32;
    }
}

/// SPI device instance
#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Spi {
    pub(crate) handle: i32,
}

impl Default for Spi {
    fn default() -> Self {
        Self{ handle: -1 }
    }
}

impl Spi {
    /// Initialise an SPI device
    pub fn init(dev: u32, baud: u32, mosi: i32, miso: i32, sck: i32, cs: i32) -> Result<Self, Error> {
        let mut handle = 0;

        println!("Initialising SPI device: {}", dev);

        let res = unsafe { api::init(dev, baud, mosi, miso, sck, cs, &mut handle) };
        if res < 0 {
            return Err(Error::Runtime(res))
        }

        println!("Received SPI handle: {}", handle);


        Ok(Self{handle})
    }

    /// De-initialise an SPI device
    pub fn deinit(&mut self) -> Result<(), Error> {
        let _res = unsafe { api::deinit(self.handle) };
        // TODO: check res

        Ok(())
    }
}

impl Write<u8> for Spi {
    type Error = Error;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        //let d = data.as_ptr();
        //let l = data.len() as u32;

        let b = RBytes{
            ptr: data.as_ptr(),
            len: data.len() as u32,
        };

        let res = unsafe { api::write(self.handle, &b) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(())
    }
}

impl Transfer<u8> for Spi {
    type Error = Error;

    fn transfer<'w>(&mut self, data: &'w mut [u8]) -> Result<(), Self::Error> {
        //let d = data.as_ptr();
        //let l = data.len() as u32;

        let mut b = WBytes{
            ptr: data.as_mut_ptr(),
            len: data.len() as u32,
        };

        let res = unsafe { api::transfer(self.handle, &mut b) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(())
    }
}

#[cfg(unfinished)]
impl Transactional<u8> for Spi {
    type Error = Error;

    fn exec<'w>(&mut self, ops: &mut [Operation<u8>]) -> Result<(), Self::Error> {
        //let d = data.as_ptr();
        //let l = data.len() as u32;

        let mut b = WBytes{
            ptr: data.as_mut_ptr(),
            len: data.len() as u32,
        };

        let res = unsafe { api::transfer(self.dev, &mut b) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(data)
    }
}
