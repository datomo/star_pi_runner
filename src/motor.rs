use core::time;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::workflow::{BlueprintBlock, Command};

/// every action has a single receiver and should
/// have the ability to hand out sender when requested
struct MotorInner {
    pub id: i32,
    pin: Vec<i32>,
    receiver: Receiver<Command>,
    sender: Sender<Command>,
}

pub(crate) struct Motor {
    inner: Arc<Mutex<MotorInner>>
}

impl Motor {
    pub(crate) fn new(block: BlueprintBlock) -> Self {
        let (sender, receiver) = channel();

        let motor = Motor { inner: Arc::new(Mutex::new(MotorInner { id: block.id, pin: block.pins, receiver, sender })) };
        motor.receiver_loop();
        motor
    }

    fn receiver_loop(&self) {
        let local_self = self.inner.clone();
        let running = thread::spawn(move || loop {
            // if pin is true
            let msg = local_self.lock().unwrap().receiver.recv();
            println!("Message received: {}", msg.unwrap().message);
        });
    }

    pub fn get_sender(&self) -> Sender<Command> {
        self.inner.lock().unwrap().sender.clone()
    }
}