use core::time;
use std::thread;

use gpio::{GpioIn, GpioOut, GpioValue};
use gpio::sysfs::{SysFsGpioInput, SysFsGpioOutput};
use sysfs_gpio::{Direction, Pin, Error};

pub(crate) struct Hx711 {
    pd_sck: Pin,
    dout: Pin,
    gain: i32,
    reference: i32,
    offset: i32,
}

impl Hx711 {
    pub(crate) fn new(sck_pin: i32, dout_pin: i32, gain: i32) -> Self {
        let sck = Pin::new(sck_pin as u64);
        let d_out = Pin::new(dout_pin as u64);
        sck.set_direction(Direction::Out);
        d_out.set_direction(Direction::In);
        Hx711 {
            pd_sck: sck,
            dout: d_out,
            gain: match gain {
                128 => 1,
                64 => 3,
                32 => 2,
                _ => 1
            },
            reference: 1,
            offset: 38_200,
        }
    }

    pub(crate) fn read(&mut self) -> i32 {
        &self.wait_ready();

        let mut value: i32 = 0;
        let mut data: Vec<u8> = vec![0b0000_0000, 0b0000_0000, 0b0000_0000];


        data[0] = self.read_next_byte();
        data[1] = self.read_next_byte();
        data[2] = self.read_next_byte();

        // HX711 Channel and gain factor are set by number of bits read
        // after 24 data bits.
        for i in 0..self.gain {
            self.pd_sck.set_value(1).unwrap();
            self.pd_sck.set_value(0).unwrap();
        }

        value = ((data[0] as i32) << 16
            | (data[1] as i32) << 8
            | (data[2] as i32)) as i32;

        -(value & 0x800000) + (value & 0x7fffff)
    }

    fn wait_ready(&mut self) {
        while match self.dout.get_value() {
            Ok(val) => val != 0,
            Err(err) => {
                println!("{}",err);
                false
            }
        }  {
            thread::sleep(time::Duration::from_millis(1));
        };
    }

    fn read_next_byte(&mut self) -> u8 {
        let mut value:u8 = 0;

        for _ in 0..8 {
            value <<= 1;
            value |= self.read_next_bit();
        }
        value
    }

    fn read_next_bit(&mut self) -> u8 {
        self.pd_sck.set_value(1).unwrap();
        self.pd_sck.set_value(0).unwrap();
        self.dout.get_value().unwrap()
    }

    pub(crate) fn get_units(&mut self, times: i32) -> f32 {
        (&self.get_value(times) / &self.reference) as f32
    }

    pub(crate) fn get_value(&mut self, times: i32) -> i32 {
        &self.read_average(times) - &self.offset
    }

    fn read_average(&mut self, times: i32) -> i32 {
        if times == 1 {
            return self.read();
        }

        let mut sum = 0;
        for i in 0..times {
            sum += *&self.read();
        };

        (sum / times) as i32
    }

    pub(crate) fn set_reference(&mut self, reference: i32) {
        if reference != 0 {
            self.reference = reference;
        }
    }

    pub(crate) fn set_offset(&mut self, offset: i32) {
        self.offset = offset;
    }

    pub(crate) fn tare(&mut self, times: i32) {
        let backup_reference: i32 = self.reference;
        self.set_reference(1);

        let value = self.read_average(times);

        self.set_offset(value);

        self.set_reference(backup_reference);
    }

    fn power_down(&mut self) {
        self.pd_sck.set_value(0).unwrap();
        self.pd_sck.set_value(1).unwrap();

        thread::sleep(time::Duration::from_nanos(100))
    }

    fn power_up(&mut self) {
        self.pd_sck.set_value(0);

        thread::sleep(time::Duration::from_nanos(100))
    }


    pub(crate) fn reset(&mut self) {
        &self.power_down();
        &self.power_up();
    }
}


