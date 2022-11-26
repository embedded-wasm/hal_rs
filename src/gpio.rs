use core::marker::PhantomData;

use embedded_hal::digital::{blocking::*, ErrorType};

use crate::Error;
use api::{GpioMode};

use self::api::GpioValue;

mod api {
    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum GpioMode {
        Input = 0,
        Output = 1,
    }

    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum GpioValue {
        Low = 0,
        High = 1,
    }

    #[link(wasm_import_module = "gpio")]
    extern "C" {
        /// Initialise the provided GPIO pin in input or output mode
        pub fn init(port: i32, pin: i32, mode: i32, handle: &mut i32) -> i32;

        /// Deinitialise the specified GPIO pin
        pub fn deinit(handle: i32) -> i32;

        /// Write to a GPIO pin
        pub fn set(handle: i32, value: u32) -> i32;

        // Read from a GPIO pin
        pub fn get(handle: i32, value: *const u32) -> i32;
    }
}


// GPIO pin instance
#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Gpio<MODE> {
    pub(crate) handle: i32,
    pub(crate) mode: PhantomData<MODE>,
}


impl <MODE> Default for Gpio<MODE> {
    fn default() -> Self {
        Self { handle: -1, mode: PhantomData }
    }
}


/// Input marker type
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Input;

/// Output marker type
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Output;

impl Gpio<Input> {
    /// Initialise a GPIO input pin
    pub fn input(port: i32, pin: i32) -> Result<Self, Error> {
        let mut handle = 0;
        
        let res = unsafe { api::init(port, pin, GpioMode::Input as i32, &mut handle) };
        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(Self{handle, mode: PhantomData})
    }
}

impl Gpio<Output> {
    /// Initialise a GPIO output pin
    pub fn output(port: i32, pin: i32) -> Result<Self, Error> {
        let mut handle = 0;
        
        let res = unsafe { api::init(port, pin, GpioMode::Output as i32, &mut handle) };
        if res < 0 {
            return Err(Error::Runtime(res))
        }

        Ok(Self{handle, mode: PhantomData})
    }
}

impl <MODE> Gpio<MODE> {
    pub fn deinit(&mut self) {
        unsafe {
            api::deinit(self.handle);
        }
    }
}

impl <MODE> ErrorType for Gpio<MODE> {
    type Error = Error;
}

impl OutputPin for Gpio<Output> {
    fn set_high(&mut self) -> Result<(), Self::Error> {

        let res = unsafe { api::set(self.handle, GpioValue::High as u32) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }
        
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {

        let res = unsafe { api::set(self.handle, GpioValue::Low as u32) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }
        
        Ok(())
    }
}

impl InputPin for Gpio<Input> {
    fn is_high(&self) -> Result<bool, Self::Error> {
        let mut v: u32 = 0;
        let p = &mut v;

        let res = unsafe { api::get(self.handle, p) };

        if res < 0 {
            return Err(Error::Runtime(res))
        }
        
        Ok(v != 0)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        let v = self.is_high()?;

        Ok(v == false)
    }
}

