use arduino_hal::{
    port::{
        Pin, 
        mode::{
            Input,  
            Floating
        }
    },
    hal::port::*
};

pub struct DHT11 {
    pin: Option<Pin<Input<Floating>, PD2>>,
}

pub struct DHT11Status {
    pub relative_humidity: u8,
    pub relative_humidity_integral: u8,
    pub temperature: u8,
    pub temperature_integral: u8,
    pub checksum: u8,
}

impl DHT11 {
    pub fn new(pin: Pin<Input<Floating>, PD2>) -> Self {
        Self {
            pin: Some(pin)
        }
    }

    pub fn get_status(&mut self) -> DHT11Status {
        let mut results = [0u8; 5];
            
        let pin = self.pin.take().unwrap();
        
        let mut pin_out = pin.into_output_high();
        arduino_hal::delay_ms(18);
        pin_out.set_low();
        arduino_hal::delay_ms(18); // Pull down for at least 18ms
        pin_out.set_high();

        self.pin = Some(pin_out.into_floating_input());
        arduino_hal::delay_us(220);
        
        
        
        for result_byte in &mut results {
            for _bit_i in 0..8 {
                let mut highs_in_a_row = 0u8;
                loop {
                    if self.pin.as_ref().unwrap().is_high() {
                        highs_in_a_row += 1;
                    }
                    else if highs_in_a_row > 6 {
                        // Decoded 1
                        *result_byte = (*result_byte << 1) | 1;
                        break;
                    }
                    else if highs_in_a_row >= 1 {
                        // Decoded 0
                        *result_byte <<= 1;
                        break;
                    }
                    
                    arduino_hal::delay_us(8);
                }
            }
        }
        arduino_hal::delay_us(80);
        DHT11Status {
            relative_humidity: results[0],
            relative_humidity_integral: results[1],
            temperature: results[2],
            temperature_integral: results[3],
            checksum: results[4]
        }

    }
}