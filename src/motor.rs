use core::time;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

use crate::blocks::Command;
use crate::workflow::BlueprintBlock;

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
            println!("pin checking");
            let msg = match local_self.lock().unwrap().receiver.try_recv() {
                () => {},
                _ => {}
            };
            println!("Message received: {}", msg.message);


            thread::sleep(time::Duration::from_millis(4000));
        });
        running.join();
    }

    pub fn get_sender(&self) -> Sender<Command> {
        self.inner.lock().unwrap().sender.clone()
    }
}