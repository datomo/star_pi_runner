use core::time;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;

use gpio::{GpioIn, GpioValue};

use crate::blocks::Logic;
use crate::workflow::{BlueprintBlock, Command, CommandMessage};
use crate::gpio::{Pin, GPIOPin, Direction};

/// saves needed information to read the button state
struct ButtonInner {
    id: i32,
    pin: i32,
    gpio_pin: GPIOPin,
    //gpio: Box<dyn GpioIn<Error=()> + Send>,
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

    pub(crate) fn dbgnew(pin:i32) -> Button{
        Button {
            inner: Arc::new(Mutex::new(ButtonInner {
                id: 0,
                pin,
                gpio_pin: GPIOPin::new(pin as u8, Direction::In).unwrap(),
            }))
        }
    }

    fn check_pressed(&mut self) {
        while 1 != self.inner.lock().unwrap().gpio_pin.get_value() {
            println!("waiting {}", self.inner.lock().unwrap().gpio_pin.get_value());
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}

impl Logic for Button {
    fn eval_command(&mut self, command: &Command) {
        match command.message {
            CommandMessage::DoublePressed => {}
            CommandMessage::Pressed => self.check_pressed(),
            _ => {}
        }
    }
}