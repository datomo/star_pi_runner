use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;

use crate::blocks::{ChannelAccess, Block};
use crate::workflow::{BlueprintBlock, Command};
use gpio::{GpioIn, GpioValue};
use core::time;
use std::error::Error;

/// saves needed information to read the button state
struct ButtonInner {
    id: i32,
    pin: i32,
    is_fired: bool,
    //gpio: Box<dyn GpioIn<Error = std::io::Error> + Send>,
    gpio: Box<dyn GpioIn<Error = ()> + Send>
}

pub struct Button { inner: Arc<Mutex<ButtonInner>> }

impl Button {
    pub(crate) fn new(block: BlueprintBlock) -> Self {
        let mut btn = Button {
            inner: Arc::new(Mutex::new(ButtonInner {
                id: block.id,
                pin: block.pins[0],
                is_fired: false,
                //gpio: Box::new(gpio::sysfs::SysFsGpioInput::open(25).unwrap()),
                gpio : Box::new(gpio::dummy::DummyGpioIn::new(||true))
            }))
        };
        btn
    }
}

impl Block for Button{
    fn eval_command(&mut self, command: &Command) {
        if command.message == "pressed1" {
            //let mut gpio25 = gpio::dummy::DummyGpioIn::new(|| true);

            while GpioValue::High != self.inner.lock().unwrap().gpio.read_value().unwrap() {
                thread::sleep(time::Duration::from_millis(100));
            }
        }
    }
}