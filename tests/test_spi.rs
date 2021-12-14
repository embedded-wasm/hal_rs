
use wasm_embedded_hal::spi::*;
use embedded_hal::spi::blocking::*;

fn main() {
    println!("Opening SPI device");

    let mut s = Spi::init(0, 4_000_000, -1, -1, -1, -1).unwrap();

    println!("SPI Write");
    let a = [0xaa, 0xbb, 0xcc];
    s.write(&a).unwrap();

    println!("SPI transfer");
    let mut b = [0xaa, 0xbb, 0xcc, 0xdd];
    s.transfer(&mut b).unwrap();
    println!("RX data: {:02x?}", b);
    assert_eq!(&b, &[0x11, 0x22, 0x33, 0x44]);

    println!("SPI deinit");
    s.deinit().unwrap();
}
