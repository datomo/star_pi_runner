use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;

use crate::blocks::{Block, ChannelAccess};
use crate::workflow::{BlueprintBlock, Command};

/// every action has a single receiver and should
/// have the ability to hand out sender when requested
struct MotorInner {
    pub id: i32,
    pin: Vec<i32>,
}

pub(crate) struct Motor {
    inner: Arc<Mutex<MotorInner>>,
}

impl Motor {
    pub(crate) fn new(block: BlueprintBlock) -> Self {
        Motor { inner: Arc::new(Mutex::new(MotorInner { id: block.id, pin: block.pins })) }
    }
}

impl Block for Motor {
    fn eval_command(&mut self, cmd: &Command) {}
}