//! An I2C detect example using wasm-embedded
//!
// Copyright 2020 Ryan Kurte

use embedded_hal::i2c::blocking::*;
use wasm_embedded_hal::i2c::I2c;

const BUS: u32 = 2;


fn main() {
    // Connect to I2C device
    let mut i2c = match I2c::init(BUS, 0, -1, -1) {
        Ok(v) => v,
        Err(_e) => return,
    };

    println!("Scanning addresses on bus: {}", BUS);

    // For each possible address
    for i in 0..128 {
        if i % 16 == 0 {
            print!("0x{:02x}: ", i);
        }

        // Attempt a read
        let mut d = [0u8; 1];
        match i2c.read(i, &mut d) {
            Ok(_) => print!("{:02x} ", i),
            Err(_) => print!("-- "),
        }

        if i % 16 == 15 {
            print!("\r\n");
        }
    }

    // Shutdown the I2C device
    i2c.deinit();

    return;
}