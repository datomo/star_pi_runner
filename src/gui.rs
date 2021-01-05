use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text, Application, executor, Command, ProgressBar};
use crate::workflow::SensorStatus;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

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
    pub(crate) fn new() -> Self {
        let (sender, receiver) = channel::<SensorStatus>();
        GuiManager { sender, receiver: Arc::new(Mutex::new(receiver)) }
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

