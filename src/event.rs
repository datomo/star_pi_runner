use std::sync::{Mutex, Arc};

pub enum Event {}

trait Observer {
    fn on_notify(&mut self);
}

pub struct EventManager {
    wrapped_observer: Vec<Arc<Mutex<dyn Observer>>>
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager { wrapped_observer: vec![] }
    }

    pub fn notify(&self, event: Event) {
        for wrapped_observer in self.wrapped_observers.clone() {
            let mut observer = wrapped_observer.lock().unwrap();
            observer.on_notify(&event);
        }
    }

    pub fn add_observer(&mut self, observer: Arc<Mutex<dyn Observer>>) {
        self.wrapped_observers.push(observer);
    }
}