use std::sync::mpsc::{Sender, Receiver, channel};
use crate::workflow::Command;
use crate::workflow::CommandStatus::Done;
use std::thread;
use std::sync::{Arc, Mutex};

trait Action {
    fn run();
}


pub trait Block:Send {
    fn eval_command(&mut self, cmd: &Command);
}

pub struct ChannelAccessInner {
    sender: Sender<Command>,
    pub(crate) receiver: Receiver<Command>,
    main_sender: Sender<Command>,
    block: Box<dyn Block>,
}

pub struct ChannelAccess {
    inner: Arc<Mutex<ChannelAccessInner>>
}

impl ChannelAccess {
    pub fn new(main_sender: Sender<Command>, block: Box<dyn Block>) -> Self {
        let (sender, receiver) = channel();
        ChannelAccess { inner: Arc::new(Mutex::new(ChannelAccessInner { sender, receiver, main_sender, block })) }
    }

    pub(crate) fn recv_loop(&self, eval_func: fn(Command)) {
        let local_self = self.inner.clone();
        thread::spawn(move || loop {
            let mut unlocked = local_self.lock().unwrap();
            let command = unlocked.receive();

            unlocked.block.eval_command(&command);

            unlocked.send_done(command);
        });
    }

    pub fn get_sender(&self) -> Sender<Command> {
        self.inner.lock().unwrap().get_sender()
    }
}

impl ChannelAccessInner {
    pub fn get_sender(&self) -> Sender<Command> {
        self.sender.clone()
    }

    pub(crate) fn send(&self, cmd: Command) {
        self.main_sender.send(cmd).unwrap()
    }

    pub(crate) fn send_done(&self, mut cmd: Command) {
        cmd.set_status(Done);
        &self.send(cmd);
    }

    pub(crate) fn receive(&self) -> Command {
        let cmd = self.receiver.recv().unwrap();
        println!("Block {}: received msg: {}", cmd.block_id, cmd.message);
        cmd
    }
}


