
# blink on attiny

## Build

`cargo build`

## Flash

- attiny85: `avrdude -p t85 -c avrispmkII -U flash:w:target/avr-attiny85/debug/avr-attiny85.elf`
- atmega168: `avrdude -p m168 -c avrispmkII -U flash:w:target/avr-atmega168/debug/avr-atmega168.elf` (ATmega 168-20PU)

List of _avrdude_ targets can be found [here](https://github.com/OnionIoT/avrdude-onion/blob/master/doc/avrdude.info)

## Configure Speed

No matter which speed you select, remember to also adjust the timer to respect the setting:

```rust
let mut delay = Delay::<MHz8>::new();
```

**Factory default 1MHz prescaler**

`avrdude -p t85 -c avrispmkII -U lfuse:w:0x62:m`

**8 MHz**

`avrdude -p t85 -c avrispmkII -U lfuse:w:0xe2:m`

## Chip Interrogation

`-U lfuse:r:-:i`
