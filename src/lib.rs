

mod error;
pub use error::Error;

pub mod spi;

pub use spi::Spi;

pub mod i2c;
pub use i2c::I2c;

pub mod gpio;
pub use gpio::{Gpio, Input, Output};

pub mod uart;
pub use uart::Uart;

/// Mutable byte array helper
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct BytesMut {
    pub ptr: *mut u8,
    pub len: u32,
}

/// Immutable byte array helper
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct Bytes {
    pub ptr: *const u8,
    pub len: u32,
}

pub(crate) mod api {
    use super::*;

    #[link(wasm_import_module = "core")]
    extern {
        // Execute a syscall via the runtime
        pub (crate) fn exec(cla: u32, ins: u32, flags: u32, handle: i32, cmd: &Bytes, resp: &BytesMut) -> i32;
    }
}

pub fn exec(cla: u32, ins: u32, flags: u32, handle: i32, cmd: &[u8], resp: &mut [u8]) -> i32 {
    // Setup array types
    let c = Bytes{
        ptr: cmd.as_ptr(),
        len: cmd.len() as u32,
    };
    let r = BytesMut{
        ptr: resp.as_mut_ptr(),
        len: resp.len() as u32,
    };

    // Call handler
    unsafe { api::exec(cla, ins, flags, handle, &c, &r) }
}