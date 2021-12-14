
use wasm_embedded_hal::gpio::Gpio;
use embedded_hal::digital::blocking::*;


fn main() {
    println!("Opening GPIO pins");

    println!("Creating input pin");
    let i = Gpio::input(0, 0).unwrap();

    println!("Reading input");
    i.is_high().unwrap();

    println!("Creating output pin");
    let mut o = Gpio::output(0, 0).unwrap();

    println!("Writing output");
    o.set_high().unwrap();
}
