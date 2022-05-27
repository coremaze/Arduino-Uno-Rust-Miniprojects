#![no_std]
#![no_main]

use arduino_hal::{port::{Pin, mode::Output}, hal::port::*};
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::PinState;
use panic_halt as _;

struct Program {
    red: Pin<Output, PD6>,
    green: Pin<Output, PD5>,
    blue: Pin<Output, PD3>
}

impl Program {
    fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        Self { 
            red: pins.d6.into_output(),
            green: pins.d5.into_output(), 
            blue: pins.d3.into_output() 
        }
    }

    fn run(&mut self) -> ! {
        loop {
            for i in 1..2u16.pow(3) {
                self.red.set_state(PinState::from(i & 0b001 != 0)).ok();
                self.green.set_state(PinState::from(i & 0b010 != 0)).ok();
                self.blue.set_state(PinState::from(i & 0b100 != 0)).ok();
                arduino_hal::delay_ms(250);
            }
        }
    }
}


#[arduino_hal::entry]
fn main() -> ! {
    let mut program = Program::new();
    program.run();
}
