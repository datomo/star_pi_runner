use std::{fs, thread};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::button::Button;
use crate::motor::Motor;

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

pub enum CommandStatus {
    Done,
    Pending,
    Error,
}

pub struct Command {
    pub(crate) flow_id: i32,
    pub(crate) block_id: i32,
    pub(crate) message: String,
    pub(crate) next: Vec<i32>,
    pub(crate) status: CommandStatus,
}

impl Command {
    pub fn new(id: i32, block_id: i32, message: String, next: Vec<i32>) -> Self {
        Command {
            flow_id: id,
            block_id,
            message,
            next,
            status: CommandStatus::Pending,
        }
    }
    pub fn from_flow_command(id: i32,flow_command: &FlowCommand, next: Vec<i32> ) -> Self {
        Command {
            flow_id: id,
            block_id: flow_command.id,
            message: flow_command.command.clone(),
            next,
            status: CommandStatus::Pending,
        }
    }
}


pub(crate) struct Manager {
    root: Vec<i32>,
    senders: Arc<Mutex<HashMap<i32, Sender<Command>>>>,
    main_sender: Sender<Command>,
    main_receiver: Arc<Mutex<Receiver<Command>>>,
    commands: HashMap<i32, Command>,
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
            commands:
            Default::default(),
            buttons: Default::default(),
            motors: Default::default(),
        };
        manager.init_commands(blueprint.clone());
        manager.init_blocks(blueprint.clone());
        manager
    }

    pub fn start(&self) {
        let receiver = self.main_receiver.clone();
        let senders = self.senders.clone();
        let running = thread::spawn(move || loop {
            //println!("i am waiting");
            let msg = receiver.lock().unwrap().recv().unwrap();
            // println!("he i work {}", msg.message);
            senders.lock().unwrap().get(&msg.flow_id).unwrap().send(msg);
        });
        running.join();
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
            let command: Command = Command::from_flow_command(id, block, blueprint.get_children(id) );
            &self.commands.insert(id, command);

            &self.parse_commands(blueprint.children
                                     .get(&id)
                                     .unwrap_or(&(vec![]))
                                     .clone(), blueprint.clone());
        }
    }
}


