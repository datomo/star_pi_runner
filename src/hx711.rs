use gpio::{GpioValue, GpioIn, GpioOut};
use std::thread;
use core::time;
use gpio::sysfs::{SysFsGpioInput, SysFsGpioOutput};

pub(crate) struct Hx711 {
    pd_sck: gpio::sysfs::SysFsGpioOutput,
    dout: gpio::sysfs::SysFsGpioInput,
    gain: i32,
    SCALE: i32,
    OFFSET: i32,
}

impl Hx711 {
    pub(crate) fn new(sck_pin: i32, dout_pin: i32, gain: i32) -> Self {
        Hx711 {
            pd_sck: gpio::sysfs::SysFsGpioOutput::open(sck_pin as u16).unwrap(),
            dout: gpio::sysfs::SysFsGpioInput::open(dout_pin as u16).unwrap(),
            gain: match gain {
                128 => 1,
                64 => 3,
                32 => 2,
                _ => 1
            },
            SCALE: 5895655,
            OFFSET: 50682624,
        }
    }

    fn is_ready(mut self) -> bool {
        self.dout.read_value().unwrap() == GpioValue::Low
    }

    fn read(&mut self) -> i32 {
        &self.wait_ready();

        let mut value: i32 = 0;
        let mut data: Vec<u8> = vec![0b0000_0000, 0b0000_0000, 0b0000_0000];
        let mut filler = 0x00;

        /*data[2] = self.shift_in_slow(1);
        data[1] = self.shift_in_slow(1);
        data[0] = self.shift_in_slow(1);*/

        for j in 0..3 {
            for i in 0..8 {
                self.pd_sck.set_high();
                let mask = match self.dout.read_value().unwrap() == GpioValue::High {
                    true => 1 << i,
                    false => 0
                };
                data[j] |= mask;
                self.pd_sck.set_low();
            }
        };

        for i in 0..self.gain {
            self.pd_sck.set_high();
            self.pd_sck.set_low();
        }

        if data[2] & 0x80 == 1 {
            filler = 0xFF;
        }

        value = ((filler as i64) << 24
            | (data[2] as i64) << 16
            | (data[1] as i64) << 8
            | (data[0] as i64)) as i32;

        value
    }

    fn wait_ready(&mut self) {
        while self.dout.read_value().unwrap() == GpioValue::Low {
            thread::sleep(time::Duration::from_millis(1));
        }
    }

    /// LSBFIRST 0
    /// MSBFIRST 1
    fn shift_in_slow(&mut self, bit_order: i8) {

    }

    pub(crate) fn get_units(&mut self, times: i32) -> f32 {
        (&self.get_value(times) / &self.SCALE) as f32
    }

    fn get_value(&mut self, times: i32) -> i32 {
        &self.read_average(times) - &self.OFFSET
    }

    fn read_average(&mut self, times: i32) -> i32 {
        let mut sum: i32 = 0;
        for _ in 0..times {
            sum += &self.read();
        };
        (sum / times) as i32
    }
}


