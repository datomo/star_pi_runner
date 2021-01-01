use core::time;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::blocks::{Block, Trigger};
use crate::workflow::BlueprintBlock;
use std::sync::mpsc::Sender;

struct ButtonInner {
    pin: i32,
    is_fired: bool,
    senders: Vec<Sender<T>>
}

pub struct Button { inner: Arc<Mutex<ButtonInner>> }

impl Trigger for Button {
    fn check_status(&self) -> bool {
        unimplemented!()
    }

    fn event_loop(&mut self) {
        println!("running");
        let local_self = self.inner.clone();
        let running = thread::spawn(move || loop {
            // if pin is true
            println!("pin checking");

            let mut local = local_self.lock().unwrap();
            local.is_fired = !local.is_fired;
            drop(local);


            thread::sleep(time::Duration::from_millis(4000));
        });
        running.join().unwrap();
    }
}

impl Button {
    pub(crate) fn new(block: BlueprintBlock) -> Self {
        let mut btn = Button { inner: Arc::new(Mutex::new(ButtonInner { pin: block.pins[0], is_fired: false })) };
        btn.event_loop();
        btn
    }

    pub(crate) fn add_sender(sender: Sender<>){

    }
}

impl Block for Button {
    fn new(block: BlueprintBlock) -> Self {
        Button { inner: Arc::new(Mutex::new(ButtonInner { pin: block.pins[0], is_fired: false })) }
    }
}