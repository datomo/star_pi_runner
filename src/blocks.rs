use std::sync::mpsc::{Sender, Receiver, channel};
use crate::workflow::Command;
use crate::workflow::CommandStatus::Done;

trait Action {
    fn run();
}


pub(crate) trait Trigger {
    /// returns
    fn check_status(&self) -> bool;

    fn event_loop(&mut self);
}


pub struct ChannelAccess {
    sender: Sender<Command>,
    pub(crate) receiver: Receiver<Command>,
    main_sender: Sender<Command>,
}

impl ChannelAccess {
    pub fn new(main_sender: Sender<Command>) -> Self {
        let (sender, receiver) = channel();
        ChannelAccess { sender, receiver, main_sender }
    }

    pub fn get_sender(&self) -> Sender<Command> {
        self.sender.clone()
    }

    pub(crate) fn send(&self, cmd: Command) {
        self.main_sender.send(cmd).unwrap()
    }

    pub(crate) fn send_done(&self, mut cmd: Command){
        cmd.set_status(Done);
        &self.send(cmd);
    }

    pub(crate) fn receive(&self) -> Command {
        let cmd = self.receiver.recv().unwrap();
        println!("Block {}: received msg: {}", cmd.block_id, cmd.message);
        cmd
    }
}


