use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

use crate::blocks::Logic;
use crate::hx711::Hx711;
use crate::workflow::{Command, CommandMessage, SensorStatus, CommandStatus};

struct Scale {
    main_sender: Sender<Command>,
    sender: Sender<Command>,
    receiver: Receiver<Command>,
    sck_pin: i32,
    d_out_pin: i32,
    last_weight: Arc<Mutex<f32>>,
    gui_sender: Sender<SensorStatus>,
}

impl Scale {
    pub(crate) fn new(pins: Vec<i32>, main_sender: Sender<Command>, gui_sender: Sender<SensorStatus>) -> Scale {
        let (sender, receiver) = channel();
        let mut scale = Scale { main_sender, sender, receiver, sck_pin: pins[0], d_out_pin: pins[1], last_weight: Arc::new(Mutex::new(0.0)), gui_sender };
        scale.scale_loop();
        scale
    }
    /// reads weight every x millis and writes it into the buffer
    fn scale_loop(&mut self) {
        thread::spawn(move || {
            let mut hx711 = Hx711::new(self.sck_pin, self.d_out_pin, 128);

            loop {
                let weight = *hx711.get_units(10);
                match self.last_weight.try_lock() {
                    Ok(ref mut mutex) => {
                        **mutex = weight;
                        self.gui_sender(SensorStatus::Scale(weight));
                    },
                    Err(_) => continue
                };


                thread::sleep(Duration::from_millis(100));
            }
        });
    }
}

impl Logic for Scale{

    /// function checks new command and waits till it is fulfilled
    fn eval_command(&mut self, cmd: &mut Command) {
        let condition = match cmd.message {
            CommandMessage::Over(amount) => { |x| x > amount }
            CommandMessage::Under(amount) => { |x| x < amount }
            CommandMessage::Between(under, over) => { |x| x < over && x > under }
            _ => { |x| true }
        };

        let unlocked = self.last_weight.lock().unwrap();
        let mut value = *unlocked;
        drop(unlocked);

        while !condition(value) {
            let unlocked = self.last_weight.lock().unwrap();
            value = *unlocked;
            drop(unlocked);

            thread::sleep(Duration::from_millis(100));
        }
        cmd.status = CommandStatus::Done;
        self.sender.send(cmd.clone());
    }
}