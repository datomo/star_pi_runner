use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use iced::{Align, Application, button, Button, Column, Command, Element, executor, ProgressBar, Sandbox, Settings, Text};

use crate::workflow::SensorStatus;

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 10;
            }
            Message::DecrementPressed => {
                self.value -= 10;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let progress_bar = ProgressBar::new(0.0..=100.0, self.value as f32);
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(progress_bar)
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}

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
        if has_gui {
            Counter::run(Settings::default());
        }
        gui
    }

    pub(crate) fn get_sender(&self) -> Sender<SensorStatus> {
        self.sender.clone()
    }

    fn loop_debug(&self) {
        thread::spawn(move || loop {
            let msg = *self.receiver.lock().unwrap().recv().unwrap();
            print!("{}", msg);
        });
    }
}

