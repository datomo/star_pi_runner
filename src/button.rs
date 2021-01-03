use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;

use crate::blocks::ChannelAccess;
use crate::workflow::{BlueprintBlock, Command};
use gpio::{GpioIn, GpioValue};
use core::time;

/// struct holds a reference to all used senders
struct ButtonInner {
    id: i32,
    pin: i32,
    is_fired: bool,
    access: ChannelAccess,
    gpio: gpio::sysfs::SysFsGpioInput,
}

impl ButtonInner {
    pub fn wait_for(&mut self, command_str: String) {
        if command_str == "pressed" {
            //let mut gpio25 = gpio::dummy::DummyGpioIn::new(|| true);

            while GpioValue::High != self.gpio.read_value().unwrap() {
                thread::sleep(time::Duration::from_millis(100));
            }
        }
    }
}

pub struct Button { inner: Arc<Mutex<ButtonInner>> }

impl Button {
    pub(crate) fn new(block: BlueprintBlock, main_sender: Sender<Command>) -> Self {
        let access: ChannelAccess = ChannelAccess::new(main_sender);
        let mut btn = Button { inner: Arc::new(Mutex::new(ButtonInner {
            id: block.id,
            pin: block.pins[0],
            is_fired: false,
            access,
            gpio: gpio::sysfs::SysFsGpioInput::open(25).unwrap() })) };
        btn.receiver_loop();
        btn
    }

    fn receiver_loop(&mut self) {
        let local_self = self.inner.clone();
        thread::spawn(move || loop {
            // if pin is true
            // println!("pin checking");

            let mut unlocked = local_self.lock().unwrap();

            let command = unlocked.access.receive();

            unlocked.wait_for(command.message.clone());

            unlocked.access.send_done(command);
        });
        //running.join().unwrap();
    }

    pub(crate) fn get_sender(&self) -> Sender<Command> {
        self.inner.lock().unwrap().access.get_sender().clone()
    }
}