
use crate::Error;
use self::api::{Bytes, BytesMut};

// WASM function calls
// TODO: replace with WITX generated ones when viable?
mod api {
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct BytesMut {
        pub ptr: *mut u8,
        pub len: u32,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct Bytes {
        pub ptr: *const u8,
        pub len: u32,
    }

    #[link(wasm_import_module = "spi")]
    extern {
        pub fn init(dev: u32, baud: u32, mosi: i32, miso: i32, sck: i32, cs: i32, handle: &mut i32) -> i32;

        /// Deinitialise the specified SPI peripheral
        pub fn deinit(handle: i32) -> i32;

        /// Write to an SPI device on the specified peripheral
        pub fn write(handle: i32, data: &Bytes) -> i32;

        /// Read from an SPI device on the specified peripheral
        pub fn read(handle: i32, data: &BytesMut) -> i32;

        /// Write to and read from an SPI device on the specified peripheral
        pub fn transfer(handle: i32, read: &mut BytesMut, write: &mut Bytes) -> i32;

        /// Write to and read from an SPI device on the specified peripheral
        pub fn transfer_inplace(handle: i32, data: &mut BytesMut) -> i32;
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

impl embedded_hal::spi::ErrorType for Spi {
    type Error = Error;
}

impl embedded_hal::spi::blocking::SpiBusWrite for Spi {

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        let b = Bytes{
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


impl embedded_hal::spi::blocking::SpiBusRead for Spi {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let b = BytesMut{
            ptr: words.as_mut_ptr(),
            len: words.len() as u32,
        };

        let res = unsafe { api::read(self.handle, &b) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(())
    }
}


impl embedded_hal::spi::blocking::SpiBus for Spi {
    fn transfer<'w>(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let mut r = BytesMut{
            ptr: read.as_mut_ptr(),
            len: read.len() as u32,
        };

        let mut w = Bytes{
            ptr: write.as_ptr(),
            len: write.len() as u32,
        };

        let res = unsafe { api::transfer(self.handle, &mut r, &mut w) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(())
    }

    fn transfer_in_place(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        let mut b = BytesMut{
            ptr: data.as_mut_ptr(),
            len: data.len() as u32,
        };

        let res = unsafe { api::transfer_inplace(self.handle, &mut b) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(())
    }
}


impl embedded_hal::spi::blocking::SpiBusFlush for Spi {
    fn flush(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(unfinished)]
impl Transactional<u8> for Spi {
    type Error = Error;

    fn exec<'w>(&mut self, ops: &mut [Operation<u8>]) -> Result<(), Self::Error> {
        //let d = data.as_ptr();
        //let l = data.len() as u32;

        let mut b = BytesMut{
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
