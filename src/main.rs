#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use atmega168_hal::clock::*;
use atmega168_hal::delay::Delay;
use atmega168_hal::prelude::*;

#[atmega168_hal::entry]
fn main() -> ! {
    let mut step: i8 = 32;

    let dp = atmega168_hal::pac::Peripherals::take().unwrap();
    let mut port_b = dp.PORTB.split();
    let mut port_c = dp.PORTC.split();
    let mut port_d = dp.PORTD.split();
    let mut adc = atmega168_hal::adc::Adc::new(dp.ADC, Default::default());

    // Reflective Opto Coupler
    let mut optocoupler = port_c.pc0.into_analog_input(&mut adc);
    let mut led = port_b.pb1.into_output(&mut port_b.ddr);

    // Push Button Switch
    let button_1 = port_d.pd0.into_pull_up_input(&mut port_d.ddr);

    // Stepper Motor

    let mut c1 = port_c.pc1.into_output(&mut port_c.ddr);
    let mut c2 = port_c.pc2.into_output(&mut port_c.ddr);
    let mut c3 = port_c.pc3.into_output(&mut port_c.ddr);
    let mut c4 = port_c.pc4.into_output(&mut port_c.ddr);

    let mut delay = Delay::<MHz1>::new();

    loop {
        let value: u16 = nb::block!(adc.read(&mut optocoupler)).void_unwrap();
        if value > 5 {
            led.set_high().void_unwrap();
        } else {
            led.set_low().void_unwrap();
        }

        if button_1.is_high().unwrap_or(false) {
            led.set_high().void_unwrap();
        }

        if value > 5 || button_1.is_high().unwrap_or(false) {
            step += 1;
            step %= 4;
        } else {
            step -= 1;
            if step <= 0 {
                step = 4
            }
        }

        match step % 4 {
            0 => {
                c1.set_high().void_unwrap();
                c2.set_low().void_unwrap();
                c3.set_high().void_unwrap();
                c4.set_low().void_unwrap()
            }
            1 => {
                c1.set_low().void_unwrap();
                c2.set_high().void_unwrap();
                c3.set_high().void_unwrap();
                c4.set_low().void_unwrap()
            }
            2 => {
                c1.set_low().void_unwrap();
                c2.set_high().void_unwrap();
                c3.set_low().void_unwrap();
                c4.set_high().void_unwrap()
            }
            3 => {
                c1.set_high().void_unwrap();
                c2.set_low().void_unwrap();
                c3.set_low().void_unwrap();
                c4.set_high().void_unwrap()
            }
            _ => (),
        }

        delay.delay_ms(1u8);
    }
}
