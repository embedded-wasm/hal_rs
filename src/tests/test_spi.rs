
use wasm_embedded_hal::spi::*;
use embedded_hal::spi::blocking::{SpiBus as _, SpiBusRead as _, SpiBusWrite as _};

fn main() {
    println!("Opening SPI device");

    let mut s = Spi::init(0, 4_000_000, -1, -1, -1, -1).unwrap();

    println!("SPI Write");
    let a = [0xaa, 0xbb, 0xcc];
    s.write(&a).unwrap();

    println!("SPI read");
    let mut b = [0x00; 5];
    s.read(&mut b).unwrap();
    assert_eq!(b, [0xab; 5]);

    println!("SPI transfer inplace");
    let mut b = [0xaa, 0xbb, 0xcc, 0xdd];
    s.transfer_in_place(&mut b).unwrap();
    println!("RX data: {:02x?}", b);
    assert_eq!(&b, &[0x11, 0x22, 0x33, 0x44]);

    println!("SPI transfer");
    let w = [0xaa, 0xbb, 0xcc, 0xdd];
    let mut r = [0x00; 4];
    s.transfer(&mut r, &w).unwrap();
    println!("RX data: {:02x?}", r);
    assert_eq!(&r, &[0x11, 0x22, 0x33, 0x44]);

    println!("SPI deinit");
    s.deinit().unwrap();
}
