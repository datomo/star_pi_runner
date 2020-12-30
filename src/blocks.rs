use std::thread;
use core::time;
use std::sync::{Arc, Mutex};

struct Block {
    pin: Vec<i32>,
    //actions: Vec<dyn Action>,
    //triggers: Vec<dyn Trigger>,
}


impl Block {
    fn loop_block(&self) {
        thread::spawn(move || loop {
            /*for trigger in &self.triggers {
                if trigger.get_status() {
                    trigger.fire()
                }
            }
            thread::sleep(time::Duration::from_millis(1000));*/
        });
    }
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
            if true {
                local_self.lock().unwrap().is_fired = true;
            }

            thread::sleep(time::Duration::from_millis(1000));
        });
    }
}

impl Button {
    pub(crate) fn new(pin: i32) -> Self {
        Button { inner: Arc::new(Mutex::new(ButtonInner { pin, is_fired: false })) }
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


struct Motor {
    pin: Vec<i32>,
}