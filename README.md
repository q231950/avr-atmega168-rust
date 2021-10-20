# Party w/ atmega168 🥳

## Prerequisites

```
rustup default nightly-2021-01-07
rustup component add rust-src
sudo apt-get install gcc-avr
sudo apt-get install avr-libc
```

## Build

`cargo build`

or

`cargo build --release`

## Flash

- attiny85: `avrdude -p t85 -c avrispmkII -U flash:w:target/avr-attiny85/debug/avr-attiny85.elf`
- atmega168: `avrdude -p m168 -c avrispmkII -U flash:w:target/avr-atmega168/debug/avr-atmega168.elf` (ATmega 168-20PU)

You can use Chip Interrogation (see below 👇) to figure out the part number. If not foune, a list of _avrdude_ targets can be found [here](https://github.com/OnionIoT/avrdude-onion/blob/master/doc/avrdude.info).

## Configure Speed

atmega168 default clock speed:

> The device is shipped with internal RC oscillator at 8.0MHz and with the fuse CKDIV8 programmed, resulting in 1.0MHz system clock.

No matter which speed you select, remember to also adjust the timer to respect the setting:

```rust
let mut delay = Delay::<MHz8>::new();
```
## Chip Interrogation

`avrdude -p m168 -c avrispmkII -U lfuse:r:-:i`
