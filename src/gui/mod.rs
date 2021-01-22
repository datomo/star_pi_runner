use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};

use web_view::*;

use crate::workflow::SensorStatus;
use crate::gui::gui::Gui;

mod gui;

// holds all available ?sensors? and expose them to the GUI
pub struct GuiManager {
    sender: Sender<SensorStatus>,
    receiver: Arc<Mutex<Receiver<SensorStatus>>>,
    gui_sender: Arc<Mutex<Sender<Update>>>,
    gui: Gui,
}

impl GuiManager {
    pub(crate) fn new(has_gui: bool) -> Self {
        let (sender, receiver) = channel::<SensorStatus>();
        let gui = Gui::new();
        let gui_sender = gui.get_sender();
        let manager = GuiManager { sender, receiver: Arc::new(Mutex::new(receiver)), gui, gui_sender: Arc::new(Mutex::new(gui_sender)) };

        manager.loop_debug();

        println!("starting gui");
        manager
    }

    pub(crate) fn get_sender(&self) -> Sender<SensorStatus> {
        return self.sender.clone();
    }


    fn loop_debug(&self) {
        let local_receiver = self.receiver.clone();
        let local_gui_sender = self.gui_sender.clone();
        thread::spawn(move || loop {
            let msg = local_receiver.lock().unwrap().recv().unwrap();
            println!("{}g", msg);
            local_gui_sender.lock().unwrap().send(msg.to_update()).unwrap()
        });
    }
}

/// generic Update struct
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Update {
    pub(crate) id: i32,
    pub(crate) min: i32,
    pub(crate) max: i32,
    pub(crate) value: i32,
}

