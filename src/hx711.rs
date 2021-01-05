use gpio::{GpioValue, GpioIn, GpioOut};
use std::thread;
use core::time;
use gpio::sysfs::{SysFsGpioInput, SysFsGpioOutput};

pub(crate) struct Hx711 {
    pd_sck: gpio::sysfs::SysFsGpioOutput,
    dout: gpio::sysfs::SysFsGpioInput,
    gain: i32,
    reference: i32,
    offset: i32,
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
            reference: 1,
            offset: 38_200,
        }
    }

    fn is_ready(mut self) -> bool {
        self.dout.read_value().unwrap() == GpioValue::Low
    }

    pub(crate) fn read(&mut self) -> i32 {
        &self.wait_ready();

        let mut value: i32 = 0;
        let mut data: Vec<u8> = vec![0b0000_0000, 0b0000_0000, 0b0000_0000];


        data[0] = self.read_next_byte();
        data[1] = self.read_next_byte();
        data[2] = self.read_next_byte();

        /// HX711 Channel and gain factor are set by number of bits read
        /// after 24 data bits.
        for i in 0..self.gain {
            self.pd_sck.set_high().unwrap();
            self.pd_sck.set_low().unwrap();
        }

        value = ((data[0] as i32) << 16
            | (data[1] as i32) << 8
            | (data[2] as i32)) as i32;

        -(value & 0x800000) + (value & 0x7fffff)
    }

    fn wait_ready(&mut self) {
        while self.dout.read_value().unwrap() != GpioValue::Low {
            thread::sleep(time::Duration::from_millis(1));
        }
    }

    fn read_next_byte(&mut self) -> u8 {
        let mut value = 0;

        for i in 0..8 {
            value <<= 1;
            value |= self.read_next_bit() as u8;
        }
        value
    }

    /// LSBFIRST 0
    /// MSBFIRST 1
    fn read_next_bit(&mut self) -> i32 {
        self.pd_sck.set_high().unwrap();
        self.pd_sck.set_low().unwrap();
        match self.dout.read_value().unwrap() == GpioValue::High {
            true => 1,
            false => 0
        }
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

        let mut sum: i32 = 0;
        for _ in 0..times {
            sum += &self.read();
        };
        (sum / times) as i32
    }

    pub(crate) fn set_reference(&mut self, reference: i32) {
        if reference != 0 {
            self.reference = reference;
        }
    }

    pub(crate) fn set_offset(&mut self, offset:i32) {
        self.offset = offset;
    }

    pub(crate) fn tare(&mut self, times:i32) {
        let backup_reference: i32 = self.reference;
        self.set_reference(1);

        let value = self.read_average(times);

        self.set_offset(value);

        self.set_reference(backup_reference);
    }

    fn power_down(&mut self) {
        self.pd_sck.set_low().unwrap();
        self.pd_sck.set_high().unwrap();

        thread::sleep(time::Duration::from_nanos(100))
    }

    fn power_up(&mut self) {
        self.pd_sck.set_low().unwrap();

        thread::sleep(time::Duration::from_nanos(100))
    }


    pub(crate) fn reset(&mut self) {
        &self.power_down();
        &self.power_up();
    }
}


