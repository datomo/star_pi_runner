use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};

use web_view::*;

use crate::workflow::SensorStatus;

mod gui;

// holds all available ?sensors? and expose them to the GUI
pub struct GuiManager {
    sender: Sender<SensorStatus>,
    receiver: Arc<Mutex<Receiver<SensorStatus>>>,
    gui_sender: Arc<Mutex<Sender<Update>>>;
    gui: Gui,
}

impl GuiManager {
    pub(crate) fn new(has_gui: bool) -> Self {
        let (sender, receiver) = channel::<SensorStatus>();
        let gui = Gui::new();
        let manager = GuiManager { sender, receiver: Arc::new(Mutex::new(receiver)), gui, gui_sender: Arc::new(Mutex::new(gui.get_sender)) };

        manager.loop_debug();

        println!("starting gui");
        manager
    }


    fn loop_debug(&self) {
        let local_receiver = self.receiver.clone();
        let local_gui_sender = self.gui_sender.clone();
        thread::spawn(move || loop {
            let msg = local_receiver.lock().unwrap().recv().unwrap();
            local_gui_sender.lock().unwrap().send(msg.to_update()).unwrap()
        });
    }
}


