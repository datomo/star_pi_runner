use std::{fs, thread};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::button::Button;
use crate::motor::Motor;
use std::borrow::Borrow;

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
pub struct Blueprint {
    pub root: Vec<i32>,
    pub blocks: HashMap<i32, BlueprintBlock>,
    pub flow_blocks: HashMap<i32, FlowCommand>,
    pub children: HashMap<i32, Vec<i32>>,
    default: Value,
    options: Value,
    colors: Value,
    id: i32,
    flow_id: i32,
    file_name: String,
}

impl Blueprint {
    pub fn flow2bloc(&self, flow_id: i32) -> &i32 {
        &self.flow_blocks.get(&flow_id).unwrap().id
    }
    pub fn get_children(&self, flow_id: i32) -> Vec<i32> {
        self.children.get(&flow_id).unwrap().clone()
    }
}


pub fn load_config() -> Blueprint {
    const PATH: &str = "config\\config.json";

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

pub struct Command {
    pub(crate) flow_id: i32,
    pub(crate) block_id: i32,
    pub(crate) message: String,
    pub(crate) next: Vec<i32>,
    pub(crate) status: CommandStatus,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Command {
            flow_id: *&self.flow_id,
            block_id: *&self.block_id,
            message: String::from(&self.message),
            next: (*&self.next).clone(),
            status: CommandStatus::Done
        }
    }
}

impl Command {
    pub fn new(id: i32, block_id: i32, message: String, next: Vec<i32>) -> Self {
        Command {
            flow_id: id,
            block_id,
            message,
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
            message: flow_command.command.clone(),
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
    buttons: HashMap<i32, Button>,
    motors: HashMap<i32, Motor>,
}

impl Manager {
    pub fn new(blueprint: Blueprint) -> Self {
        let (main_sender, main_receiver) = channel();
        let mut manager = Manager {
            root: blueprint.root.clone(),
            senders: Arc::new(Mutex::new(Default::default())),
            main_sender,
            main_receiver: Arc::new(Mutex::new(main_receiver)),
            commands: Arc::new(Mutex::new(Default::default())),
            buttons: Default::default(),
            motors: Default::default(),
        };
        manager.init_commands(blueprint.clone());
        manager.init_blocks(blueprint.clone());
        manager
    }

    pub fn start(&self) {
        let local_receiver = self.main_receiver.clone();
        let local_senders = self.senders.clone();
        let local_commands = self.commands.clone();

        &self.initial_send();

        let running = thread::spawn(move || loop {
            //println!("i am waiting");
            let msg = local_receiver.lock().unwrap().recv().unwrap();
            println!("received msg from {}", msg.block_id);
            if msg.status == CommandStatus::Running {
                let senders = local_senders.lock().unwrap();
                let commands = local_commands.lock().unwrap();
                for id in msg.next {
                    if commands.contains_key(&id) {
                        let command = commands.get(&id).unwrap();
                        println!("sending now to block_id: {}", command.block_id);
                        senders.get(&command.block_id).unwrap().send((*command).clone());
                    }
                }
            }
        });
        running.join();
    }

    pub fn initial_send(&self) {
        let senders = self.senders.lock().unwrap();
        let commands = self.commands.lock().unwrap();

        for id in self.root.iter() {
            if commands.contains_key(id) {
                println!("sending initialy to {}", id);
                senders.get(&id).unwrap().send((*commands.get(&id).unwrap()).clone());
            }
        }

        drop(senders);
        drop(commands);
    }

    pub fn get_sender(&self) -> Sender<Command> {
        self.main_sender.clone()
    }

    /// initializes all available blocks and opens a channel to each one
    pub fn init_blocks(&mut self, blueprint: Blueprint) {
        for (id, block) in blueprint.blocks {
            if block.get_module() == "button" {
                let btn = Button::new(block, self.get_sender());
                &self.senders.lock().unwrap().insert(id, btn.get_sender());
                &self.buttons.insert(id, btn);
            } else if block.get_module() == "motor" {
                let motor = Motor::new(block, self.get_sender());
                &self.senders.lock().unwrap().insert(id, motor.get_sender());
                &self.motors.insert(id, motor);
            }
        }

        println!("end of init: {}", self.senders.lock().unwrap().len())
    }

    /// parses the commands and inserts them into a workflow
    fn init_commands(&mut self, blueprint: Blueprint) {
        &self.parse_commands(blueprint.root.clone(), blueprint.clone());
    }

    fn parse_commands(&mut self, next: Vec<i32>, blueprint: Blueprint) {
        for id in next {
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


