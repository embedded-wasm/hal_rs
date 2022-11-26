
use wasm_embedded_hal::i2c::*;
use embedded_hal::i2c::blocking::{I2c as _};

fn main() {
    println!("Opening I2c device");

    let mut s = I2c::init(0, 4_000_000, -1, -1).unwrap();
    let addr = 0x0a;

    println!("I2C Write");
    let a = [0xaa, 0xbb, 0xcc];
    s.write(addr, &a).unwrap();

    println!("I2C Read");
    let mut b = [0u8; 4];
    s.read(addr, &mut b).unwrap();
    println!("RX data: {:02x?}", b);
    assert_eq!(&b, &[0x11, 0x22, 0x33, 0x44]);

    println!("I2C WriteRead");
    s.write_read(addr, &a, &mut b).unwrap();
    assert_eq!(&b, &[0x22, 0x33, 0x44, 0x55]);

}
