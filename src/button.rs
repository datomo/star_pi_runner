use core::time;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;

use gpio::{GpioIn, GpioValue};

use crate::blocks::Logic;
use crate::workflow::{BlueprintBlock, Command, CommandMessage};

/// saves needed information to read the button state
struct ButtonInner {
    id: i32,
    pin: i32,
    gpio: Box<dyn GpioIn<Error=std::io::Error> + Send>,
    //gpio: Box<dyn GpioIn<Error=()> + Send>,
}

pub struct Button { inner: Arc<Mutex<ButtonInner>> }

impl Button {
    pub(crate) fn new(block: BlueprintBlock) -> Self {
        let mut btn = Button {
            inner: Arc::new(Mutex::new(ButtonInner {
                id: block.id,
                pin: block.pins[0],
                gpio: Box::new(gpio::sysfs::SysFsGpioInput::open(block.pins[0] as u16).unwrap()),
                // gpio: Box::new(gpio::dummy::DummyGpioIn::new(|| true)),
            }))
        };
        btn
    }

    fn check_pressed(&mut self) {
        while GpioValue::High != self.inner.lock().unwrap().gpio.read_value().unwrap() {
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