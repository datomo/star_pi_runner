use core::time;
use std::thread;

use gpio::{GpioIn, GpioOut, GpioValue};
use gpio::sysfs::{SysFsGpioInput, SysFsGpioOutput};

use crate::gpio::{Pin, State, Direction, GPIOPin};
use std::time::Duration;

pub(crate) struct Hx711 {
    pd_sck: GPIOPin,
    d_out: GPIOPin,
    gain: i32,
    reference: i32,
    offset: i32,
}

impl Hx711 {
    pub(crate) fn new(sck_pin: i32, dout_pin: i32, gain: i32) -> Self {
        let pd_sck = GPIOPin::new(sck_pin as u8, Direction::Out).unwrap();
        let d_out = GPIOPin::new(dout_pin as u8, Direction::In).unwrap();

        Hx711 {
            pd_sck,
            d_out,
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
        let mut data: Vec<i32> = vec![0b0000_0000, 0b0000_0000, 0b0000_0000];


        data[0] = self.read_next_byte() as i32;
        data[1] = self.read_next_byte() as i32;
        data[2] = self.read_next_byte() as i32;

        // HX711 Channel and gain factor are set by number of bits read
        // after 24 data bits.
        for _ in 0..self.gain {
            self.pd_sck.set_high();
            self.pd_sck.set_low();
        }

        value = ((data[0]) << 16
            | (data[1]) << 8
            | (data[2])) as i32;

        -(value & 0x800000) + (value & 0x7fffff)
    }

    fn wait_ready(&mut self) {
        while *self.d_out.get_state() == State::High {
            thread::sleep(time::Duration::from_millis(1));
        };
    }

    fn read_next_byte(&mut self) -> u8 {
        let mut value: u8 = 0;

        for _ in 0..8 {
            value <<= 1;
            value |= self.read_next_bit();
        }
        value
    }

    fn read_next_bit(&mut self) -> u8 {
        self.pd_sck.set_high();
        self.pd_sck.set_low();
        self.d_out.get_value()
    }

    pub(crate) fn get_units(&mut self, times: i32) -> f32 {
        (&self.get_value(times) / &self.reference) as f32
    }

    pub(crate) fn get_value(&mut self, times: i32) -> i32 {
        &self.read_average(times) - &self.offset
    }

    fn read_average(&mut self, times: i32) -> i32 {
        if times == 1 {
            return self.read() as i32;
        }

        let mut sum:i32 = 0;
        for _ in 0..times {
            sum += self.read();
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
        self.pd_sck.set_low();
        self.pd_sck.set_high();

        thread::sleep(time::Duration::from_nanos(100))
    }

    fn power_up(&mut self) {
        self.pd_sck.set_low();

        thread::sleep(time::Duration::from_nanos(100))
    }


    pub(crate) fn reset(&mut self) {
        &self.power_down();
        &self.power_up();
    }
}


