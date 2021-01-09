use crate::gpio::{Pin, GPIOPin, Direction};
use std::time::Instant;

/// Implementation for 2, 4 and 5 pin stepper motors
/// Drives a unipolar, bipolar, or five phase stepper motor.
///
/// https://github.com/arduino-libraries/Stepper/blob/master/src/Stepper.cpp

pub(crate) struct Stepper {
    step_number: i32,
    direction: i32,
    last_step_time: Instant,
    number_of_steps: i32,
    pins: Vec<GPIOPin>,
    pin_count: u8,
    step_delay: i64,
}

impl Stepper {
    pub(crate) fn new(number_of_steps: i32, pin_numbers: Vec<i32>) -> Self {
        let pin_count = pin_numbers.len() as u8;
        let mut pins: Vec<GPIOPin> = Vec::new();
        for number in pin_numbers {
            pins.push(GPIOPin::new(number as u8, Direction::Out).unwrap())
        }

        Stepper {
            step_number: 0,
            direction: 0,
            last_step_time: Instant::now(),
            number_of_steps,
            pins,
            pin_count,
            step_delay: 10000,
        }
    }

    pub(crate) fn set_speed(&mut self, speed: i64) {
        self.step_delay = (60_i64 * 1000_i64 * 1000_i64 / self.number_of_steps as i64 / speed);
    }

    /// Moves the motor steps_to_move steps.  If the number is negative,
    /// the motor moves in the reverse direction.
    pub(crate) fn step(&mut self, steps_to_move: i32) {
        let mut steps_left = steps_to_move.abs();

        self.direction = match steps_to_move > 0 {
            true => 1,
            false => 0
        };

        while steps_left > 0 {
            let now = Instant::now();

            if now.duration_since(self.last_step_time).as_micros() >= self.step_delay as u128 {
                self.last_step_time = now;

                match self.direction == 1 {
                    true => { // clockwise
                        self.step_number += 1;
                        if self.step_number == self.number_of_steps {
                            self.step_number = 0;
                        }
                    }
                    false => { // counter-clockwise
                        if self.step_number == 0 {
                            self.step_number = self.number_of_steps;
                        }
                        self.step_number -= 1;
                    }
                }

                steps_left -= 1;

                match self.pin_count == 5 {
                    true => self.step_motor(self.step_number % 10),
                    false => self.step_motor(self.step_number % 4)
                }
            }
        }
    }

    /// delegates the specific step to the sub-functions
    fn step_motor(&mut self, this_step: i32) {
        match self.pin_count {
            2 => self.step_2_pin(this_step),
            4 => self.step_4_pin(this_step),
            _ => self.step_5_pin(this_step)
        }
    }

    /// sets the pins to the two needed configurations
    /// 01 -> 11 -> 10 -> 00
    fn step_2_pin(&mut self, this_step: i32) {
        match this_step {
            0 => {
                self.pins[0].set_low();
                self.pins[1].set_high();
            }
            1 => {
                self.pins[0].set_high();
                self.pins[1].set_high();
            }
            2 => {
                self.pins[0].set_high();
                self.pins[1].set_low();
            }
            _ => {
                self.pins[0].set_low();
                self.pins[1].set_low();
            }
        }
    }

    /// sets the pins to the two needed configurations
    /// 1010 -> 0110 -> 0101 -> 1001
    fn step_4_pin(&mut self, this_step: i32 ) {
        match this_step {
            0 => {
                self.pins[0].set_high();
                self.pins[1].set_low();
                self.pins[2].set_high();
                self.pins[3].set_low();
            }
            1 => {
                self.pins[0].set_low();
                self.pins[1].set_high();
                self.pins[2].set_high();
                self.pins[3].set_low();
            }
            2 => {
                self.pins[0].set_low();
                self.pins[1].set_high();
                self.pins[2].set_low();
                self.pins[3].set_high();
            }
            _ => {
                self.pins[0].set_high();
                self.pins[1].set_low();
                self.pins[2].set_low();
                self.pins[3].set_high();
            }
        }
    }

    fn step_5_pin(&mut self, this_step:i32) {
        match this_step {
            0 => { // 01101
                self.pins[0].set_low();
                self.pins[1].set_high();
                self.pins[2].set_high();
                self.pins[3].set_low();
                self.pins[4].set_high();
            }
            1 => { // 01001
                self.pins[0].set_low();
                self.pins[1].set_high();
                self.pins[2].set_low();
                self.pins[3].set_low();
                self.pins[4].set_high();
            }
            2 => { // 01011
                self.pins[0].set_low();
                self.pins[1].set_high();
                self.pins[2].set_low();
                self.pins[3].set_high();
                self.pins[4].set_high();
            }
            3 => { // 01010
                self.pins[0].set_low();
                self.pins[1].set_high();
                self.pins[2].set_low();
                self.pins[3].set_high();
                self.pins[4].set_low();
            }
            4 => { // 11010
                self.pins[0].set_high();
                self.pins[1].set_high();
                self.pins[2].set_low();
                self.pins[3].set_high();
                self.pins[4].set_low();
            }
            5 => { // 10010
                self.pins[0].set_high();
                self.pins[1].set_low();
                self.pins[2].set_low();
                self.pins[3].set_high();
                self.pins[4].set_low();
            }
            6 => { // 10110
                self.pins[0].set_high();
                self.pins[1].set_low();
                self.pins[2].set_high();
                self.pins[3].set_high();
                self.pins[4].set_low();
            }
            7 => { // 10100
                self.pins[0].set_high();
                self.pins[1].set_low();
                self.pins[2].set_high();
                self.pins[3].set_low();
                self.pins[4].set_low();
            }
            8 => { // 10101
                self.pins[0].set_high();
                self.pins[1].set_low();
                self.pins[2].set_high();
                self.pins[3].set_low();
                self.pins[4].set_high();
            }
            _ => { // 00101
                self.pins[0].set_low();
                self.pins[1].set_low();
                self.pins[2].set_high();
                self.pins[3].set_low();
                self.pins[4].set_high();
            }
        }
    }
}