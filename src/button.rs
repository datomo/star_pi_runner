use core::time;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;

use crate::blocks::{Trigger};
use crate::workflow::{BlueprintBlock, Command};
use std::borrow::Borrow;

/// struct holds a reference to all used senders
struct ButtonInner {
    id: i32,
    pin: i32,
    is_fired: bool,
    senders: Vec<Sender<Command>>,
}

pub struct Button { inner: Arc<Mutex<ButtonInner>> }

impl Trigger for Button {
    fn check_status(&self) -> bool {
        unimplemented!()
    }

    fn event_loop(&mut self) {
        let local_self = self.inner.clone();
        let running = thread::spawn(move || loop {
            // if pin is true
            // println!("pin checking");

            let unlocked = local_self.lock().unwrap();
            for sender in &unlocked.senders {
                sender.send(Command { id: unlocked.id, block_id: 32, message: "hello from here".to_string(), next: vec![] });
                println!("sending to {}", unlocked.id)
            }
            drop(unlocked);
            println!("successfully sent!");


            thread::sleep(time::Duration::from_millis(4000));
        });
        running.join().unwrap();
    }
}

impl Button {
    pub(crate) fn new(block: BlueprintBlock) -> Self {
        let mut btn = Button { inner: Arc::new(Mutex::new(ButtonInner { id: block.id, pin: block.pins[0], is_fired: false, senders: vec![] })) };
        //btn.event_loop();
        btn
    }

    pub(crate) fn add_sender(&mut self, sender: Sender<Command>) {
        &self.inner.lock().unwrap().senders.push(sender);
    }
}