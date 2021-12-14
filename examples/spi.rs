
use embedded_hal::spi::blocking::*;
use wasm_embedded_hal::spi::Spi;

fn main() {
    println!("Opening SPI device");

    let mut s = Spi::init(0, 4_000_000, -1, -1, -1, -1).unwrap();

    println!("Write data");
    s.write(&[0xaa, 0xbb, 0xcc]).unwrap();

    println!("Transfer data");
    let mut b = [0xaa, 0xbb, 0xcc];
    s.transfer(&mut b).unwrap();
    println!("RX data: {:02x?}", b);    
}
