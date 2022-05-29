// D3-D10 = D0-7 on lcd
// D11 = RS (register select)
// D12 = RW
// D13 = E
use arduino_hal::{
    port::{
        Pin, 
        mode::{
            Input, 
            Output, 
            Floating
        }
    }, 
    hal::port::*,
};
use embedded_hal::digital::v2::{OutputPin, PinState};


pub struct LCD1602 {
    pub d0: Option<Pin<Input<Floating>, PD3>>,
    pub d1: Option<Pin<Input<Floating>, PD4>>,
    pub d2: Option<Pin<Input<Floating>, PD5>>,
    pub d3: Option<Pin<Input<Floating>, PD6>>,
    pub d4: Option<Pin<Input<Floating>, PD7>>,
    pub d5: Option<Pin<Input<Floating>, PB0>>,
    pub d6: Option<Pin<Input<Floating>, PB1>>,
    pub d7: Option<Pin<Input<Floating>, PB2>>,
    pub rs: Option<Pin<Input<Floating>, PB3>>,
    pub rw: Option<Pin<Input<Floating>, PB4>>,
    pub e: Pin<Output, PB5>,
}

pub enum LCD1602Mode {
    Instruction,
    Data,
}

impl LCD1602 {
    pub fn write(&mut self, mode: LCD1602Mode, data: u8) {
        let mut rs = self.rs.take().unwrap().into_output();

        match mode {
            LCD1602Mode::Instruction => rs.set_low(),
            LCD1602Mode::Data => rs.set_high(),
        }

        let mut d0 = self.d0.take().unwrap().into_output();
        let mut d1 = self.d1.take().unwrap().into_output();
        let mut d2 = self.d2.take().unwrap().into_output();
        let mut d3 = self.d3.take().unwrap().into_output();
        let mut d4 = self.d4.take().unwrap().into_output();
        let mut d5 = self.d5.take().unwrap().into_output();
        let mut d6 = self.d6.take().unwrap().into_output();
        let mut d7 = self.d7.take().unwrap().into_output();
        

        d0.set_state(PinState::from((data & 0b00000001) != 0)).ok();
        d1.set_state(PinState::from((data & 0b00000010) != 0)).ok();
        d2.set_state(PinState::from((data & 0b00000100) != 0)).ok();
        d3.set_state(PinState::from((data & 0b00001000) != 0)).ok();
        d4.set_state(PinState::from((data & 0b00010000) != 0)).ok();
        d5.set_state(PinState::from((data & 0b00100000) != 0)).ok();
        d6.set_state(PinState::from((data & 0b01000000) != 0)).ok();
        d7.set_state(PinState::from((data & 0b10000000) != 0)).ok();
        
        let mut rw = self.rw.take().unwrap().into_output();
        rw.set_low(); // Low for "write"
        
        self.e.set_high(); // latch
        arduino_hal::delay_us(12);
        self.e.set_low();
        arduino_hal::delay_ms(3);

        self.rw = Some(rw.into_floating_input());
        self.d7 = Some(d7.into_floating_input());
        self.d6 = Some(d6.into_floating_input());
        self.d5 = Some(d5.into_floating_input());
        self.d4 = Some(d4.into_floating_input());
        self.d3 = Some(d3.into_floating_input());
        self.d2 = Some(d2.into_floating_input());
        self.d1 = Some(d1.into_floating_input());
        self.d0 = Some(d0.into_floating_input());
        self.rs = Some(rs.into_floating_input());
    }

    pub fn clear_display(&mut self) {
        self.write(LCD1602Mode::Instruction, 0b00000001);
    }

    pub fn return_home(&mut self) {
        self.write(LCD1602Mode::Instruction, 0b00000010);
    }

    pub fn display_on(&mut self, display: bool, cursor: bool, cursor_pos: bool) {
        let mut command: u8 = 0b1000;
        command |= (display as u8) << 2;
        command |= (cursor as u8) << 1;
        command |= (cursor_pos as u8) << 0;

        self.write(LCD1602Mode::Instruction, command);
    }

    pub fn function_set(&mut self, data_8bit: bool, double_line: bool, big_font: bool) {
        let mut command: u8 = 0b1000_00;
        command |= (data_8bit as u8) << 4;
        command |= (double_line as u8) << 3;
        command |= (big_font as u8) << 2;
        self.write(LCD1602Mode::Instruction, command); 
    }

    pub fn set_position(&mut self, position: u8) {
        self.write(LCD1602Mode::Instruction, 0b10000000 | position);
    }

    pub fn set_char_at(&mut self, position: u8, letter: char) {
        self.set_position(position);
        self.set_char(letter);
    }
    
    pub fn set_char(&mut self, letter: char) {
        self.write(LCD1602Mode::Data, letter as u8);
    }

    pub fn print(&mut self, text: &str) {
        for x in text.chars() {
            self.set_char(x);
        }
    }

    pub fn print_at(&mut self, position: u8, text: &str) {
        self.set_position(position);
        self.print(text);
    }

    pub fn print_number_at(&mut self, position: u8, number: u8) {
        self.set_position(position);
        self.print_number(number);
    }

    pub fn print_number(&mut self, number: u8) {
        // let mut digits = 0;
        if number == 0 {
            self.set_char('0');
            // digits += 1;
        }
        else {
            for digit in (0..=number.log10()).rev() {
                self.set_char(
                    get_digit(number, digit)
                );
                // digits += 1;
            }
        }

        // let padding_needed = 3 - digits;
        // for _ in 0..padding_needed {
        //     self.set_char(' ');
        // }
    }

    pub fn clear_rest_of_line(&mut self) {
        for _ in 0..16 {
            self.set_char(' ');
        }
    }
}

fn get_digit(number: u8, digit: u32) -> char {
    let mut val: u8 =  number / (10u8.pow(digit));
    val %= 10;
    return ('0' as u8 + val) as char;
}