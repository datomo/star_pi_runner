use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

use crate::blocks::Logic;
use crate::hx711::Hx711;
use crate::workflow::{BlueprintBlock, Command, CommandMessage, CommandStatus, SensorStatus};

pub(crate) struct Scale {
    id: i32,
    sck_pin: i32,
    d_out_pin: i32,
    last_weight: Arc<Mutex<f32>>,
    gui_sender: Arc<Mutex<Sender<SensorStatus>>>,
}

impl Scale {
    pub(crate) fn new(block: BlueprintBlock, gui_sender: Sender<SensorStatus>) -> Scale {
        let mut scale = Scale { id: block.id, sck_pin: block.pins[0], d_out_pin: block.pins[1], last_weight: Arc::new(Mutex::new(0.0)), gui_sender: Arc::new(Mutex::new(gui_sender)) };
        scale.scale_loop();
        scale
    }
    /// reads weight every x millis and writes it into the buffer
    fn scale_loop(&mut self) {
        let last_weight = self.last_weight.clone();
        let gui_sender = self.gui_sender.clone();
        let id = self.id.clone();
        let sck = self.sck_pin;
        let d_out = self.d_out_pin;
        thread::spawn(move || {
            let mut hx711 = Hx711::new(sck, d_out, 128);
            hx711.tare(20);
            hx711.set_reference(85_500.0 / 264.0);

            loop {
                let weight = hx711.get_units(10);
                match last_weight.try_lock() {
                    Ok(ref mut mutex) => {

                        **mutex = weight;
                        gui_sender.lock().unwrap().send(SensorStatus::Scale{ id, value: weight as i32, max: 200 });
                    }
                    Err(_) => continue
                };


                thread::sleep(Duration::from_millis(100));
            }
        });
    }
}

impl Logic for Scale {
    // function checks new command and waits till it is fulfilled
    // closure have to be boxed as no 2 closures are the same for the compiler
    // https://github.com/rust-lang/rust/issues/24036
    fn eval_command(&mut self, cmd: &Command) {
        let condition: Box<dyn Fn(f32) -> bool> = match cmd.message {
            CommandMessage::Over(amount) => Box::new(move |x| x > amount as f32),
            CommandMessage::Under(amount) => Box::new(move |x| x < amount as f32),
            CommandMessage::Between(under, over) => Box::new(move |x| { x < over as f32 && x > under as f32 }),
            _ => Box::new(|x| true)
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
    }
}