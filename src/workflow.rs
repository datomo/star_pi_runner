use core::time;
use std::{fmt, fs, thread};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::mpsc::{channel, Receiver, Sender};

use serde::{Deserialize, Serialize};
use serde::export::Formatter;
use serde_json::Value;

use crate::blocks::{ChannelAccess, Logic};
use crate::button::Button;
use crate::motor::Motor;
use crate::scale::Scale;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct BlueprintBlock {
    pub id: i32,
    pub name: String,
    pub pins: Vec<i32>,
    pub options: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct FlowCommand {
    pub id: i32,
    pub command: String,
}

impl BlueprintBlock {
    pub fn get_type(&self) -> &str {
        &self.options["type"]
    }

    pub fn get_module(&self) -> &str {
        &self.options["module"]
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct BluePrintLoop {
    repeat: i32,
    target: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct CommandLoop {
    repeat: i32,
    target: i32,
    next: Vec<i32>,
}


impl CommandLoop {
    pub(crate) fn decrease(&mut self) {
        self.repeat -= 1;
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Blueprint {
    pub root: Vec<i32>,
    pub blocks: HashMap<i32, BlueprintBlock>,
    pub flow_blocks: HashMap<i32, FlowCommand>,
    pub children: HashMap<i32, Vec<i32>>,
    pub loops: HashMap<i32, BluePrintLoop>,
    default: Value,
    options: Value,
    colors: Value,
    id: i32,
    flow_id: i32,
    file_name: String,
}

impl Blueprint {
    pub fn flow2block(&self, flow_id: i32) -> &i32 {
        &self.flow_blocks.get(&flow_id).unwrap().id
    }
    pub fn get_children(&self, flow_id: i32) -> Vec<i32> {
        self.children.get(&flow_id).unwrap().clone()
    }
}


pub fn load_config() -> Blueprint {
    const PATH: &str = "config.json";
    // const PATH: &str = "config\\config.json";

    let contents: String = fs::read_to_string(PATH).expect("Something went wrong reading the file!");

    let blueprint: Blueprint = serde_json::from_str(contents.as_str()).unwrap();

    blueprint
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CommandStatus {
    Done,
    Running,
    Initial,
    Error,
}

#[derive(Debug, Copy, Clone)]
pub enum CommandMessage {
    DoublePressed,
    Pressed,
    Over(i32),
    Under(i32),
    Between(i32, i32),
    Rotate { steps: i32, speed: i32 },
    None,
}

impl CommandMessage {
    pub fn from_string(msg: &String) -> CommandMessage {
        let split: Vec<&str> = msg.split("_").collect::<Vec<&str>>();
        match split[0] {
            "pressed" => CommandMessage::Pressed,
            "doublePressed" => CommandMessage::DoublePressed,
            "rotate" => CommandMessage::Rotate { steps: split[1].parse().unwrap(), speed: 60 },
            "over" => CommandMessage::Over(split[1].parse().unwrap()),
            "under" => CommandMessage::Under(split[1].parse().unwrap()),
            "between" => CommandMessage::Between(split[1].parse().unwrap(), split[2].parse().unwrap()),
            _ => CommandMessage::None
        }
    }
}


pub struct Command {
    pub(crate) flow_id: i32,
    pub(crate) block_id: i32,
    pub(crate) message: CommandMessage,
    pub(crate) next: Vec<i32>,
    pub(crate) status: CommandStatus,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Command {
            flow_id: self.flow_id,
            block_id: self.block_id,
            message: self.message.clone(),
            next: (self.next).clone(),
            status: CommandStatus::Done,
        }
    }
}

impl Command {
    pub fn new(id: i32, block_id: i32, message: String, next: Vec<i32>) -> Self {
        Command {
            flow_id: id,
            block_id,
            message: CommandMessage::from_string(&message),
            next,
            status: CommandStatus::Initial,
        }
    }

    pub fn set_status(&mut self, status: CommandStatus) {
        self.status = status;
    }

    pub fn from_flow_command(id: i32, flow_command: &FlowCommand, next: Vec<i32>) -> Self {
        Command {
            flow_id: id,
            block_id: flow_command.id,
            message: CommandMessage::from_string(&flow_command.command),
            next,
            status: CommandStatus::Initial,
        }
    }
}

pub(crate) struct Manager {
    root: Vec<i32>,
    senders: Arc<Mutex<HashMap<i32, Sender<Command>>>>,
    main_sender: Sender<Command>,
    main_receiver: Arc<Mutex<Receiver<Command>>>,
    commands: Arc<Mutex<HashMap<i32, Command>>>,
    endpoints: HashMap<i32, Box<ChannelAccess>>,
    loops: Arc<Mutex<HashMap<i32, CommandLoop>>>,
    gui_sender: Sender<SensorStatus>,
}

impl Manager {
    pub fn new(blueprint: Blueprint, gui_sender: Sender<SensorStatus>) -> Self {
        let (main_sender, main_receiver) = channel();
        let mut manager = Manager {
            root: blueprint.root.clone(),
            senders: Arc::new(Mutex::new(Default::default())),
            main_sender,
            main_receiver: Arc::new(Mutex::new(main_receiver)),
            commands: Arc::new(Mutex::new(Default::default())),
            endpoints: Default::default(),
            loops: Default::default(),
            gui_sender,
        };
        manager.init_commands(blueprint.clone());
        manager.init_blocks(blueprint.clone());
        manager.init_loops(blueprint.clone());
        manager
    }

    /// manager waits for responses for its initialized components
    pub fn start(&self) {
        let local_receiver = self.main_receiver.clone();
        let local_senders = self.senders.clone();
        let local_commands = self.commands.clone();
        let local_loops = self.loops.clone();

        &self.initial_send();

        let running = thread::spawn(move || loop {
            //println!("i am waiting");
            let msg = local_receiver.lock().unwrap().recv().unwrap();
            let mut loops = local_loops.lock().unwrap();
            let mut commands = local_commands.lock().unwrap();
            let mut senders = local_senders.lock().unwrap();

            let mut send_process = |id: &i32| {
                if commands.contains_key(&id) {
                    let command = commands.get_mut(&id).unwrap();
                    command.set_status(CommandStatus::Running);
                    println!("Manager: sending now to block_id: {}", command.block_id);
                    //commands.insert(command.block_id, command.clone());

                    senders.get(&command.block_id).unwrap().send(command.clone());
                }
            };

            //thread::sleep(time::Duration::from_millis(1000));
            println!("Manager: received msg from {}", msg.block_id);
            if msg.status == CommandStatus::Done {
                println!("next: {:?}, flow_id: {}, block_id: {}", msg.next, msg.flow_id, msg.block_id);
                for id in msg.next {
                    // check if next block is start of loop and replace with correct id
                    if loops.contains_key(&id) {
                        let block = loops.get_mut(&id).unwrap();

                        println!("{}", block.repeat);
                        match block.repeat > 0 || block.repeat == -1 {
                            true => { // send one more
                                block.decrease();

                                send_process(&block.target);
                            }
                            false => { // loop is empty send to next
                                for id in block.next.clone() {
                                    send_process(&id);
                                }
                            }
                        }
                    } else {
                        send_process(&id);
                    }
                }
            }
        });
        running.join();
    }




    /// manager sends first message to all blocks which appear in the root
    pub fn initial_send(&self) {
        let commands = self.commands.lock().unwrap();
        for id in self.root.iter() {
            if commands.contains_key(id) {
                let command: Command = commands.get(&id).unwrap().clone();
                println!("sending initially to {}", command.block_id);
                self.senders.lock().unwrap().get(&command.block_id).unwrap().send(command);
                println!("finished sending");
            }
        }
    }

    pub fn get_sender(&self) -> Sender<Command> {
        self.main_sender.clone()
    }

    /// initializes all available blocks and opens a channel to each one
    pub fn init_blocks(&mut self, blueprint: Blueprint) {
        for (id, block) in blueprint.blocks {
            let block: Box<dyn Logic> = match block.get_module() {
                "button" => Box::new(Button::new(block)),
                "motor" => Box::new(Motor::new(block)),
                "scale" => Box::new(Scale::new(block, self.gui_sender.clone())),
                _ => Box::new(Button::new(block))
            };

            let endpoint = ChannelAccess::new(self.get_sender(), block);

            &self.senders.lock().unwrap().insert(id, endpoint.get_sender());
            &self.endpoints.insert(id, Box::new(endpoint));
        }
    }

    fn init_loops(&mut self, blueprint: Blueprint) {
        for (id, block) in blueprint.loops {
            let next = match blueprint.children.contains_key(&id) {
                true => blueprint.children.get(&id).unwrap().clone(),
                false => Vec::new()
            };

            let loop_block = CommandLoop {
                repeat: block.repeat,
                target: block.target,
                next,
            };
            self.loops.lock().unwrap().insert(id, loop_block);
        }
    }

    /// parses the commands and inserts them into a workflow
    fn init_commands(&mut self, blueprint: Blueprint) {
        &self.parse_commands(blueprint.root.clone(), blueprint.clone());
    }

    /// handles one level of children and parses them
    fn parse_commands(&mut self, next: Vec<i32>, blueprint: Blueprint) {
        for id in next {
            if blueprint.loops.contains_key(&id) {
                &self.parse_commands(blueprint.get_children(id), blueprint.clone());
                return;
            }
            let block: &FlowCommand = blueprint.flow_blocks.get(&id).unwrap();
            let command: Command = Command::from_flow_command(id, block, blueprint.get_children(id));
            &self.commands.lock().unwrap().insert(id, command);

            &self.parse_commands(blueprint.children
                                     .get(&id)
                                     .unwrap_or(&(vec![]))
                                     .clone(), blueprint.clone());
        }
    }
}


pub enum SensorStatus {
    Scale(f32),
}

impl fmt::Display for SensorStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            SensorStatus::Scale(amount) => write!(f, "Scale: {}g", amount)
        }
    }
}

