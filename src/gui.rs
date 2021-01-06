use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::workflow::SensorStatus;

/// struct should hold all available ?sensors? and expose them to the GUI
pub struct GuiManager {
    sender: Sender<SensorStatus>,
    receiver: Arc<Mutex<Receiver<SensorStatus>>>,
}

impl GuiManager {
    pub(crate) fn new(has_gui: bool) -> Self {
        let (sender, receiver) = channel::<SensorStatus>();
        let gui = GuiManager { sender, receiver: Arc::new(Mutex::new(receiver)) };
        gui.loop_debug();
        if has_gui {}
        println!("starting gui");
        gui
    }

    pub(crate) fn get_sender(&self) -> Sender<SensorStatus> {
        self.sender.clone()
    }

    fn loop_debug(&self) {
        let local_sender = self.receiver.clone();
        thread::spawn(move || loop {
            let msg = local_sender.lock().unwrap().recv().unwrap();
            println!("{}", msg);
        });
    }
}

