#![no_std]
#![no_main]

extern crate panic_halt;

use atmega168_hal::prelude::*;

#[atmega168_hal::entry]
fn main() -> ! {
    let mut current_level = 0;
    let mut target_level = 0;

    let dp = atmega168_hal::pac::Peripherals::take().unwrap();
    let mut port_b = dp.PORTB.split();
    let mut port_c = dp.PORTC.split();
    let mut port_d = dp.PORTD.split();
    let mut adc = atmega168_hal::adc::Adc::new(dp.ADC, Default::default());

    // LED
    let mut led = port_b.pb1.into_output(&mut port_b.ddr);

    // Reflective Opto Coupler
    let mut optocoupler_0 = port_c.pc0.into_analog_input(&mut adc);
    let mut optocoupler_1 = port_c.pc1.into_analog_input(&mut adc);

    // Push Button Switch
    let button_0 = port_d.pd0.into_pull_up_input(&mut port_d.ddr);
    let button_1 = port_d.pd1.into_pull_up_input(&mut port_d.ddr);

    // Stepper Motor
    let mut step: i8 = 23;
    let mut c1 = port_b.pb2.into_output(&mut port_b.ddr);
    let mut c2 = port_c.pc2.into_output(&mut port_c.ddr);
    let mut c3 = port_c.pc3.into_output(&mut port_c.ddr);
    let mut c4 = port_c.pc4.into_output(&mut port_c.ddr);

    loop {

        led.set_low().void_unwrap();

        if current_level == target_level {
            step = -1;

            led.set_high().void_unwrap();

            if button_0.is_high().unwrap_or(false) {
                target_level = 0;
            }

            if button_1.is_high().unwrap_or(false) {
                target_level = 1;
            }
        } else if current_level < target_level {
            step += 1;
            step %= 4;
        } else if current_level > target_level {
            step -= 1;
            if step <= 0 {
                step = 4
            }
        }

        let opto_0_value: u16 = nb::block!(adc.read(&mut optocoupler_0)).void_unwrap();
        let opto_1_value: u16 = nb::block!(adc.read(&mut optocoupler_1)).void_unwrap();

        if opto_0_value > 5 {
            current_level = 0
        } else if opto_1_value > 5 {
            current_level = 1
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
    }
}
