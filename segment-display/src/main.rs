#![no_std]
#![no_main]

use arduino_hal::{port::{Pin, mode::Output}, hal::port::*};
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::PinState;
use panic_halt as _;

struct Program {
    a: Pin<Output, PD1>,
    b: Pin<Output, PD2>,
    c: Pin<Output, PD3>,
    d: Pin<Output, PD4>,
    e: Pin<Output, PD5>,
    f: Pin<Output, PD6>,
    g: Pin<Output, PD7>
}

const SEGMENTS: [u8; 16] = [
    0b1111110, // 0
    0b0110000, // 1
    0b1101101, // 2
    0b1111001, // 3
    0b0110011, // 4
    0b1011011, // 5
    0b1011111, // 6
    0b1110000, // 7
    0b1111111, // 8
    0b1111011, // 9
    0b1110111, // A
    0b0011111, // b
    0b1001110, // C
    0b0111101, // d
    0b1001111, // E
    0b1000111  // F
];

impl Program {
    fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        Self {
            a: pins.d1.into_output(),
            b: pins.d2.into_output(),
            c: pins.d3.into_output(),
            d: pins.d4.into_output(),
            e: pins.d5.into_output(),
            f: pins.d6.into_output(),
            g: pins.d7.into_output(),
        }
    }

    fn run(&mut self) -> ! {
        loop {
            for i in 0..=15 {
                self.show_num(i);
                arduino_hal::delay_ms(250);
            }
        }
    }

    fn show_num(&mut self, num: u8) {
        match num {
            0..=16 => {
                let digit: u8 = SEGMENTS[num as usize];
                self.a.set_state(PinState::from((digit & 0b1000000) != 0)).ok();
                self.b.set_state(PinState::from((digit & 0b0100000) != 0)).ok();
                self.c.set_state(PinState::from((digit & 0b0010000) != 0)).ok();
                self.d.set_state(PinState::from((digit & 0b0001000) != 0)).ok();
                self.e.set_state(PinState::from((digit & 0b0000100) != 0)).ok();
                self.f.set_state(PinState::from((digit & 0b0000010) != 0)).ok();
                self.g.set_state(PinState::from((digit & 0b0000001) != 0)).ok();
            }
            _ => {}
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let mut program = Program::new();
    program.run();
}
