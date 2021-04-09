#![no_std]
#![no_main]

// This uses an attiny in combination with a CD4026B to drive a 7 segment display.

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use atmega168_hal::prelude::*;

#[atmega168_hal::entry]
fn main() -> ! {
    let dp = atmega168_hal::pac::Peripherals::take().unwrap();
    unimplemented!()
}
