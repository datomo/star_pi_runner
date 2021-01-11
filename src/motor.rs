use std::sync::{Arc, Mutex};

use crate::blocks::Logic;
use crate::stepper::Stepper;
use crate::workflow::{BlueprintBlock, Command, CommandMessage};

/// every action has a single receiver and should
/// have the ability to hand out sender when requested
struct MotorInner {
    pub id: i32,
    stepper: Stepper,
}

pub(crate) struct Motor {
    inner: Arc<Mutex<MotorInner>>,
}

impl Motor {
    pub(crate) fn new(block: BlueprintBlock) -> Self {
        let stepper = Stepper::new(200, block.pins);
        Motor { inner: Arc::new(Mutex::new(MotorInner { id: block.id, stepper })) }
    }
}

impl Logic for Motor {
    fn eval_command(&mut self, cmd: &Command) {
        match cmd.message {
            CommandMessage::Rotate(steps, rpm:i32) => {
                let mut stepper = self.inner.lock().unwrap().stepper;
                stepper.set_speed(rpm);
                stepper.step(steps);
            }
            _ => {}
        }
    }
}