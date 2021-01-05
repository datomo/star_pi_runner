use crate::hx711::Hx711;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::workflow::{Command, SensorStatus, CommandMessage};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::VecDeque;

struct Scale {
    main_sender: Sender<Command>,
    sender: Sender<Command>,
    receiver: Receiver<Command>,
    sck_pin: i32,
    d_out_pin: i32,
    last_weight: Arc<Mutex<f32>>,
    gui_sender: Sender<SensorStatus>
}

impl Scale {
    pub(crate) fn new(d_out_pin: i32, sck_pin: i32, main_sender: Sender<Command>, gui_sender: Sender<SensorStatus>) -> Scale {
        let (sender, receiver) = channel();
        let mut scale = Scale { main_sender, sender, receiver, sck_pin, d_out_pin, last_weight: Arc::new(Mutex::new(0.0)), gui_sender };
        scale.scale_loop();
        scale.receiver_loop();
        scale
    }

    fn scale_loop(&mut self) {
        thread::spawn(move || {
            let mut hx711 = Hx711::new(self.sck_pin, self.d_out_pin, 128);

            loop {
                let weight = *hx711.get_units(10);
                let mut buffer = self.last_weight.lock().unwrap();
                buffer = weight;
                drop(buffer);
                self.gui_sender(SensorStatus::Scale(weight));

                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    fn receiver_loop(&self) {
        thread::spawn(move || loop {
            let msg = &self.receiver.recv().unwrap();

            let condition = match msg.message {
                CommandMessage::Over(amount) => {|x| x > amount}
                CommandMessage::Under(amount) => {|x| x < amount}
                CommandMessage::Between(under, over) => {|x| x < over && x > under  }
                _ => {|x|true}
            };

            let unlocked = self.last_weight.lock().unwrap();
            let mut value = *unlocked;
            drop(unlocked);

            while !condition(value){
                let unlocked = self.last_weight.lock().unwrap();
                value = *unlocked;
                drop(unlocked);

                thread::sleep(Duration::from_millis(100));
            }

        });
    }
}