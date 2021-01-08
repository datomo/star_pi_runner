use std::{thread, time};
use std::borrow::Borrow;
use std::io::Error;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use gpio_cdev::{Chip, EventRequestFlags, LineEvent, LineEventHandle, LineHandle, LineRequestFlags};

pub(crate) enum Direction {
    In,
    Out,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum State {
    High,
    Low,
}

pub enum EventType {
    Rising,
    Falling,
    Both,
}

impl State {
    fn value(&self) -> u8 {
        match *self {
            State::High => 1,
            State::Low => 0,
        }
    }
}


pub(crate) trait Pin {
    /// generates a Pin which maps to a generic GPIO Pin

    fn set_high(&mut self);

    fn set_low(&mut self);

    /// returns the state as State enum
    fn get_state(&self) -> &State;

    /// returns the state as numeric
    fn get_value(&self) -> u8;
}


pub(crate) struct GPIOPin {
    pin_number: u8,
    pin: LineHandle,
    direction: Direction,
}

impl GPIOPin {
    pub(crate) fn new(pin_number: u8, direction: Direction) -> Result<GPIOPin, gpio_cdev::errors::Error> {
        let line = Chip::new("/dev/gpiochip0").unwrap()
            .get_line(pin_number as u32).unwrap();

        let dir = match direction {
            Direction::In => LineRequestFlags::INPUT,
            Direction::Out => LineRequestFlags::OUTPUT,
        };
        let pin = line.request(dir, 0, format!("pin_{}", pin_number).as_str()).unwrap();

        Ok(GPIOPin { pin_number, pin, direction: Direction::In })
    }


    /// wait for a specific event to occur and block the execution
    pub(crate) fn wait(&mut self, event: EventType) {
        match event {
            EventType::Rising => {
                self.wait_until(State::Low);
                self.wait_flip(0)
            }
            EventType::Falling => {
                self.wait_until(State::High);
                self.wait_flip(1)
            }
            EventType::Both => {
                self.wait_flip(self.pin.get_value().unwrap())
            }
        }
    }

    /// wait until the value of the pin flips to the opposite of **val**
    fn wait_flip(&self, mut val: u8) {
        while self.pin.get_value().unwrap() == val {
            val = self.pin.get_value().unwrap();
            thread::sleep(Duration::from_nanos(10));
        }
    }


    pub(crate) fn wait_flip_multiple(&self, initial: u8, flips: i32) {
        let debounce = 1000;
        let mut val = initial;
        let mut counter = 0;

        while counter != flips {
            let time = Instant::now();
            self.wait_flip(val);
            val = match val {
                1 => 0,
                0 => 1,
                _ => 0
            };
            match time.elapsed().as_millis() < debounce {
                true => counter += 1,
                false => counter = 0,
            };
        }
    }

    fn wait_until(&self, state: State) {
        let val = state.value();
        while self.pin.get_value().unwrap() != val {
            println!("i am waiting: {}, is:{}", val, self.pin.get_value().unwrap());
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

impl Pin for GPIOPin {
    fn set_high(&mut self) {
        self.pin.set_value(1);
    }

    fn set_low(&mut self) {
        self.pin.set_value(0);
    }

    fn get_state(&self) -> &State {
        match self.pin.get_value().unwrap() {
            1 => &State::High,
            _ => &State::Low
        }
    }

    fn get_value(&self) -> u8 {
        self.pin.get_value().unwrap()
    }
}


