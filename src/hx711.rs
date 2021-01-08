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
    reference: f32,
    offset: f32,
    digits_after: u8
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
            reference: 1.0,
            offset: 0.0,
            digits_after: 0
        }
    }

    pub(crate) fn read(&mut self) -> i32 {
        &self.wait_ready();

        let mut value: i32 = 0;
        let mut data: Vec<i32> = vec![0b0000_0000, 0b0000_0000, 0b0000_0000];


        data[0] = self.read_next_byte();
        data[1] = self.read_next_byte();
        data[2] = self.read_next_byte();

        // HX711 Channel and gain factor are set by number of bits read
        // after 24 data bits.
        for _ in 0..self.gain {
            self.pd_sck.set_high();
            self.pd_sck.set_low();
        }

        value = ((data[0]) << 16
            | (data[1]) << 8
            | (data[2]));

        -(value & 0x800000) + (value & 0x7fffff)
    }

    fn wait_ready(&mut self) {
        while *self.d_out.get_state() == State::High {
            thread::sleep(time::Duration::from_millis(1));
        };
    }

    fn read_next_byte(&mut self) -> i32 {
        let mut value: i32 = 0;

        for _ in 0..8 {
            value <<= 1;
            value |= self.read_next_bit() as i32;
        }
        value
    }

    fn read_next_bit(&mut self) -> u8 {
        self.pd_sck.set_high();
        self.pd_sck.set_low();
        self.d_out.get_value()
    }

    pub(crate) fn get_units(&mut self, times: i32) -> f32 {
        let factor = 10_i32.pow(self.digits_after as u32) as f32;
        ((self.get_value(times) as f32 / self.reference) * factor).round() / factor
    }

    pub(crate) fn get_value(&mut self, times: i32) -> f32 {
        self.read_average(times) - self.offset
    }

    fn read_average(&mut self, times: i32) -> f32 {
        if times == 1 {
            return self.read() as f32;
        }

        let times = times as usize;

        if times < 5 {
            return self.read_median(times);
        }


        let mut sum: Vec<i32> = vec![0; times];
        for i in 0..times {
            sum[i] += self.read();
        };
        sum.sort();

        // just remove the worst outliers
        sum[1..sum.len() - 1].iter().sum::<i32>() as f32 / (times as usize - 2) as f32
    }

    fn read_median(&mut self, times: usize) -> f32 {
        let mut sum: Vec<i32> = Vec::with_capacity(times);
        for i in 0..times {
            sum[i] += self.read();
        };
        sum.sort();
        sum[sum.len() / 2] as f32
    }

    pub(crate) fn set_reference(&mut self, reference: f32) {
        if reference != 0 as f32 {
            self.reference = reference;
        }
    }

    pub(crate) fn set_offset(&mut self, offset: f32) {
        self.offset = offset;
    }

    pub(crate) fn tare(&mut self, times: i32) {
        let backup_reference: f32 = self.reference;
        self.set_reference(1.0);

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


