// #[macro_use]
extern crate serde_derive;
extern crate web_view;

use web_view::*;

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
        thread::spawn(move || {
            build();
        });
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


pub(crate) fn build() {
    let html_content = "<html><body><h1>Hello, World!</h1></body></html>";

    web_view::builder()
        .title("My Project")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}