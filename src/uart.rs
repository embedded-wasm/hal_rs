
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

    #[link(wasm_import_module = "uart")]
    extern {
        /// Initialise the specified uart device
        pub fn init(dev: u32, baud: u32, tx: i32, rx: i32, handle: &mut i32) -> i32;

        /// Deinitialise the specified Uart peripheral
        pub fn deinit(handle: i32) -> i32;

        /// Write to an Uart device
        pub fn write(handle: i32, flags: u32, data: &RBytes) -> i32;

        /// Read from an Uart device
        pub fn read(handle: i32, flags: u32, buff: &WBytes) -> i32;
    }
}

/// Uart device instance
#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Uart {
    pub(crate) handle: i32,
}

impl Default for Uart {
    fn default() -> Self {
        Self { handle: -1 }
    }
}


impl Uart {
    /// Initialise an Uart device
    pub fn init(dev: u32, baud: u32, tx: i32, rx: i32) -> Result<Self, Error> {
        println!("Initialising UART device: {}", dev);

        let mut handle = 0;
        let res = unsafe { api::init(dev, baud, tx, rx, &mut handle) };
        if res < 0 {
            return Err(Error::Runtime(res))
        }
        
        println!("Received UART handle: {}", handle);

        Ok(Self{handle})
    }

    /// De-initialise an Uart device
    pub fn deinit(&mut self) -> Result<(), Error> {
        let _res = unsafe { api::deinit(self.handle) };
        // TODO: check res

        Ok(())
    }

    /// Read function, not currently exposed by e-h..?
    pub fn read<'w>(&mut self, data: &'w mut [u8]) -> Result<(), Error> {

        let mut b = WBytes{
            ptr: data.as_mut_ptr(),
            len: data.len() as u32,
        };

        let res = unsafe { api::read(self.handle, 0, &mut b) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(())
    }
}

impl embedded_hal::serial::ErrorType for Uart {
    type Error = Error;
}


impl embedded_hal::serial::blocking::Write for Uart {
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {

        let b = RBytes{
            ptr: data.as_ptr(),
            len: data.len() as u32,
        };

        let res = unsafe { api::write(self.handle, 0, &b) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
