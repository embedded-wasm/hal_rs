
use embedded_hal::i2c::{SevenBitAddress};

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

impl embedded_hal::i2c::ErrorType for I2c {
    type Error = Error;
}


impl embedded_hal::i2c::blocking::I2c<SevenBitAddress> for I2c {
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

    //TODO: implement these APIs
    fn write_iter<B>(&mut self, _address: SevenBitAddress, _bytes: B) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8> {
        return Err(Error::Unimplemented)
    }

    //TODO: implement these APIs
    fn write_iter_read<B>(
            &mut self,
            _address: SevenBitAddress,
            _bytes: B,
            _buffer: &mut [u8],
        ) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8> {
        return Err(Error::Unimplemented)
    }

    //TODO: implement these APIs
    fn transaction<'a>(
            &mut self,
            _address: SevenBitAddress,
            _operations: &mut [embedded_hal::i2c::blocking::Operation<'a>],
        ) -> Result<(), Self::Error> {
        return Err(Error::Unimplemented)
    }

    //TODO: implement these APIs
    fn transaction_iter<'a, O>(&mut self, _ddress: SevenBitAddress, _operations: O) -> Result<(), Self::Error>
        where
            O: IntoIterator<Item = embedded_hal::i2c::blocking::Operation<'a>> {
        return Err(Error::Unimplemented)
    }
}
