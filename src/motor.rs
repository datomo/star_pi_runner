use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;

use crate::blocks::ChannelAccess;
use crate::workflow::{BlueprintBlock, Command};

/// every action has a single receiver and should
/// have the ability to hand out sender when requested
struct MotorInner {
    pub id: i32,
    pin: Vec<i32>,
    access: ChannelAccess,
}

pub(crate) struct Motor {
    inner: Arc<Mutex<MotorInner>>
}

impl Motor {
    pub(crate) fn new(block: BlueprintBlock, main_sender: Sender<Command>) -> Self {
        let access: ChannelAccess = ChannelAccess::new(main_sender);

        let motor = Motor { inner: Arc::new(Mutex::new(MotorInner { id: block.id, pin: block.pins, access })) };
        motor.receiver_loop();
        motor
    }

    fn receiver_loop(&self) {
        let local_self = self.inner.clone();
        thread::spawn(move || loop {
            // if pin is true
            let unlocked = local_self.lock().unwrap();

            let command = unlocked.access.receive();
            unlocked.access.send_done(command);
        });
    }

    pub fn get_sender(&self) -> Sender<Command> {
        self.inner.lock().unwrap().access.get_sender()
    }
}