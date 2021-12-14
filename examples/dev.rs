use wasm_embedded_hal::{Device, I2c, Spi, Gpio};


fn main() {
    // Setup device with required peripherals
    let mut d = Device{
        spi: &mut [Spi::default(), Spi::default()],
        i2c: &mut [I2c::default(), I2c::default()],
        io_in: &mut [Gpio::default()],
        io_out: &mut [Gpio::default()],
    };

    // Load peripheral information
    d.load().unwrap();

    println!("Loaded: {:?}", d);
}
