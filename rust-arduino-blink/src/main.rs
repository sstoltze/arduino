// https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0

#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;
mod morse;

const MORSE_TIME_SCALE: u16 = 300;

fn morse_delay(time: u16) {
    arduino_uno::delay_ms(time * MORSE_TIME_SCALE);
}

fn morse_sequence(led: &mut PB5<Output>, seq: morse::MorseSequence) {
    seq.iter().for_each(|m| morse_toggle(led, m));
}

fn morse_toggle(led: &mut PB5<Output>, m: &morse::Morse) {
    if !m.pause() {
        led.toggle().void_unwrap();
    }
    morse_delay(m.timing());
    if !m.pause() {
        led.toggle().void_unwrap();
    }
    morse_delay(1);
}

fn morse_blink(led: &mut PB5<Output>, sentence: &str) {
    sentence
        .chars()
        .for_each(|c| morse_sequence(led, morse::char_to_morse(c)))
}

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        morse_blink(&mut led, "sarah");
    }
}
