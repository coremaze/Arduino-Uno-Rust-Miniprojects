#![no_std]
#![no_main]

use arduino_hal::{port::{Pin, mode::Output}, hal::port::PB5};
use panic_halt as _;


const DOT_MS: u16 = 200;
const DASH_MS: u16 = 400;
const WAIT_MS: u16 = 300;

struct Program {
    led: Pin<Output, PB5>
}

impl Program {
    fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        Self {
            led: pins.d13.into_output()
        }
    }

    fn run(&mut self) -> ! {
        loop {
            self.s();
            self.o();
            self.s();
            arduino_hal::delay_ms(1000);
        }
    }

    fn s(&mut self) {
        for _ in 0..3 {
            self.led.set_high();
            arduino_hal::delay_ms(DOT_MS);
            self.led.set_low();
            arduino_hal::delay_ms(WAIT_MS);
        }
    }

    fn o(&mut self) {
        for _ in 0..3 {
            self.led.set_high();
            arduino_hal::delay_ms(DASH_MS);
            self.led.set_low();
            arduino_hal::delay_ms(WAIT_MS);
        }
    }


}


#[arduino_hal::entry]
fn main() -> ! {
    let mut program = Program::new();
    program.run();
}
