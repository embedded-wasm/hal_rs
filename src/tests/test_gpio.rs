
use wasm_embedded_hal::gpio::*;
use embedded_hal::digital::blocking::*;

fn main() {
    println!("Opening GPIO output pin");
    let mut o = Gpio::output(2, 3).unwrap();

    println!("GPIO set");
    o.set_high().unwrap();

    println!("Opening GPIO input pin");
    let i = Gpio::input(2, 4).unwrap();

    println!("GPIO get");
    let v = i.is_high().unwrap();

    println!("Value: {:?}", v);
    assert_eq!(v, false);
}
