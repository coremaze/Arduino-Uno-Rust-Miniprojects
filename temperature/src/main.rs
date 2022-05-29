#![no_std]
#![no_main]
#![feature(int_log)]

use arduino_hal::{
    Usart, 
    pac::USART0, 
    port::{
        Pin, 
        mode::{
            Input, 
            Output, 
        }
    },
    hal::port::*
};

mod hardware;
use hardware::DHT11;
use hardware::LCD1602;

use panic_halt as _;

struct Program {
    serial: Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>,
    dht11: DHT11,
    lcd: LCD1602,
}

impl Program {
    fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        let serial = arduino_hal::default_serial!(dp, pins, 57600);
        let temperature_pin = pins.d2;
        let d0 = pins.d3;
        let d1 = pins.d4;
        let d2 = pins.d5;
        let d3 = pins.d6;
        let d4 = pins.d7;
        let d5 = pins.d8;
        let d6 = pins.d9;
        let d7 = pins.d10;
        let rs = pins.d11;
        let rw = pins.d12;
        let e = pins.d13.into_output();
        
        Self {
            serial,
            dht11: DHT11::new(temperature_pin),
            lcd: LCD1602 {
                d0: Some(d0),
                d1: Some(d1),
                d2: Some(d2),
                d3: Some(d3),
                d4: Some(d4),
                d5: Some(d5),
                d6: Some(d6),
                d7: Some(d7),
                rs: Some(rs),
                rw: Some(rw),
                e: e,
            }
        }
    }

    fn run(&mut self) -> ! {       
        self.lcd.function_set(true, true, true);
        self.lcd.display_on(true, false, false);
        self.lcd.clear_display(); 
        self.lcd.return_home(); 

        self.lcd.print("    Welcome!    ");
        arduino_hal::delay_ms(2500);
        self.lcd.clear_display();
        let humidity_prefix = "Humidity: ";
        let temperature_prefix = "Temp: ";
        self.lcd.print_at(0, humidity_prefix);
        self.lcd.print_at(0x40, temperature_prefix);

        let humidity_pos = humidity_prefix.len() as u8;
        let temperature_pos = (temperature_prefix.len() + 0x40) as u8;

        loop {
            let status = self.dht11.get_status();
            let humidity = status.relative_humidity;
            let temp = status.temperature;
            let temp_f =((temp as f32 * (9.0/5.0)) + 32.5) as u8;

            self.lcd.set_position(humidity_pos);
            self.lcd.print_number(humidity);
            self.lcd.set_char('%');
            self.lcd.clear_rest_of_line();

            self.lcd.set_position(temperature_pos);
            self.lcd.print_number(temp_f);
            self.lcd.print("F / ");
            self.lcd.print_number(temp);
            self.lcd.print("C");
            self.lcd.clear_rest_of_line();

            ufmt::uwriteln!(&mut self.serial, "Hum: {}; Temp {}", status.relative_humidity, status.temperature).ok();
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let mut program = Program::new();
    program.run();
}
