use std::{thread, time};
use std::io::Error;
use gpio_cdev::{Chip, LineRequestFlags, LineHandle};

pub(crate) enum Direction {
    In,
    Out,
}

#[derive(PartialEq, Eq)]
pub(crate) enum State {
    High,
    Low,
}

impl State {
    fn value(&self) -> u8 {
        match *self {
            State::High => 1,
            State::Low => 2,
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

struct DummyPin {
    pin: u8,
    direction: Direction,
    value: State,
}

impl DummyPin {
    fn new(pin_number: u8, direction: Direction) -> DummyPin {
        DummyPin { pin: pin_number, direction, value: State::High }
    }
}

impl Pin for DummyPin {

    fn set_high(&mut self) {
        eprintln!("setting high");
    }

    fn set_low(&mut self) {
        eprintln!("setting low");
    }

    fn get_state(&self) -> &State {
        &self.value
    }

    fn get_value(&self) -> u8 {
        State::High.value()
    }
}


pub(crate)struct GPIOPin {
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

