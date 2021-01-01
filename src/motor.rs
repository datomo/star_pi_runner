use crate::blocks::Block;
use crate::workflow::BlueprintBlock;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::collections::HashMap;

/// every action has a single receiver and should
/// have the ability to hand out sender when requested
pub(crate) struct Motor {
    pin: Vec<i32>,
    receiver: Receiver<Command>,
    sender: Sender<Command>,
}

impl Block for Motor {
    fn new(block: BlueprintBlock) -> Self {
        let (sender, receiver) = channel();
        Motor { pin: block.pins, receiver, sender }
    }
}

impl Motor {
    pub fn get_sender(&self) -> &Sender<Command> {
        &self.sender.clone()
    }
}