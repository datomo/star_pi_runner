use core::time;
use std::sync::{Arc, Mutex};
use std::thread;

use serde_json::Value;

use crate::workflow::BlueprintBlock;

pub(crate) trait Block {
    fn new(block: BlueprintBlock) -> Self;
}


trait Action {
    fn run();
}


trait Trigger {
    /// returns
    fn check_status(&self) -> bool;

    fn event_loop(&mut self);
}


struct ButtonInner {
    pin: i32,
    is_fired: bool,
}

pub struct Button { inner: Arc<Mutex<ButtonInner>> }

impl Trigger for Button {
    fn check_status(&self) -> bool {
        unimplemented!()
    }

    fn event_loop(&mut self) {
        let local_self = self.inner.clone();
        thread::spawn(move || loop {
            // if pin is true

            let mut local = local_self.lock().unwrap();
            local.is_fired = !local.is_fired;
            drop(local);


            thread::sleep(time::Duration::from_millis(4000));
        });
    }
}

impl Button {
    pub(crate) fn new(block: BlueprintBlock) -> Self {
        Button { inner: Arc::new(Mutex::new(ButtonInner { pin: block.pins[0], is_fired: false })) }
    }

    pub(crate) fn start(&mut self) {
        let local_self = self.inner.clone();
        &self.event_loop();
        let running = thread::spawn(move || loop {
            // if pin is true
            println!("{}", local_self.lock().unwrap().is_fired);

            thread::sleep(time::Duration::from_millis(1000));
        });

        running.join();
    }
}

impl Block for Button {
    fn new(block: BlueprintBlock) -> Self {
        Button { inner: Arc::new(Mutex::new(ButtonInner { pin: block.pins[0], is_fired: false })) }
    }
}

pub(crate) struct Motor {
    pin: Vec<i32>,
}

impl Block for Motor {
    fn new(block: BlueprintBlock) -> Self {
        Motor { pin: block.pins }
    }
}