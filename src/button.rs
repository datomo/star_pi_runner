use core::time;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;

use gpio::{GpioIn, GpioValue};

use crate::blocks::Logic;
use crate::gpio::{Direction, GPIOPin, Pin, EventType};
use crate::workflow::{BlueprintBlock, Command, CommandMessage};

/// saves needed information to read the button state
struct ButtonInner {
    id: i32,
    pin: i32,
    gpio_pin: GPIOPin,
}

pub struct Button { inner: Arc<Mutex<ButtonInner>> }

impl Button {
    pub(crate) fn new(block: BlueprintBlock) -> Button {
        Button {
            inner: Arc::new(Mutex::new(ButtonInner {
                id: block.id,
                pin: block.pins[0],
                gpio_pin: GPIOPin::new(block.pins[0] as u8, Direction::In).unwrap(),
            }))
        }
    }

    pub(crate) fn dbgnew(pin: i32) -> Button {
        Button {
            inner: Arc::new(Mutex::new(ButtonInner {
                id: 0,
                pin,
                gpio_pin: GPIOPin::new(pin as u8, Direction::In).unwrap(),
            }))
        }
    }

    fn check_pressed(&mut self) {
        self.inner.lock().unwrap().gpio_pin.wait(EventType::Rising);
    }

    fn check_pressed_multiple(&mut self, amount:i32) {
        self.inner.lock().unwrap().gpio_pin.wait_flip_multiple(0, 2*amount - 1)
    }



}

impl Logic for Button {
    fn eval_command(&mut self, command: &Command) {
        match command.message {
            CommandMessage::DoublePress => self.check_pressed_multiple(2),
            CommandMessage::Press => self.check_pressed(),
            _ => {}
        }
    }
}