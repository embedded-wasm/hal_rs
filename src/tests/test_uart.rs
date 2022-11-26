
use wasm_embedded_hal::uart::*;
use embedded_hal::serial::{
    nb::Read,
    blocking::Write,
};

fn main() {
    println!("Opening UART device");

    let mut s = Uart::init(0, 4_000_000, -1, -1).unwrap();

    println!("UART Write");
    let a = [0xaa, 0xbb, 0xcc];
    s.write(&a).unwrap();

    println!("UART Read");
    let mut b = [0u8; 4];
    s.read(&mut b).unwrap();
    println!("RX data: {:02x?}", b);
    assert_eq!(&b, &[0x11, 0x22, 0x33, 0x44]);
}
