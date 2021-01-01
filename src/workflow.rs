use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::button::Button;
use crate::motor::Motor;
use std::sync::mpsc::{channel, Sender, Receiver};
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

pub struct BlockWrapper {
    motors: HashMap<i32, Motor>,
    buttons: HashMap<i32, Button>,
}


pub fn load_config() -> Blueprint {
    const PATH: &str = "config\\config.json";

    let contents: String = fs::read_to_string(PATH).expect("Something went wrong reading the file!");

    let blueprint: Blueprint = serde_json::from_str(contents.as_str()).unwrap();

    blueprint
}


pub struct Command {
    pub(crate) id: i32,
    pub(crate) block_id: i32,
    pub(crate) message: String,
    pub(crate) next: Vec<i32>,
}

struct Workflow {
    command: Command,
    next: Vec<i32>,
}

pub(crate) struct Manager {
    root: Vec<i32>,
    sender: Sender<Command>,
    receiver: Receiver<Command>,
    commands: HashMap<i32, Command>,
}

impl Manager {
    pub fn new(blueprint: Blueprint) -> Self {
        let (sender, receiver) = channel();
        Manager { root: blueprint.root, sender, receiver, commands: Default::default() }
    }

    pub fn clone_sender(&self) -> Sender<Command> {
        self.sender.clone()
    }

    /// initializes all available blocks and opens a channel to each one
    pub fn init_blocks(blueprint: Blueprint) {}

    /// parses the commands and inserts them into a workflow
    fn init_commands(&mut self, blueprint: Blueprint) {
        &self.parse_commands(blueprint.root.clone(), blueprint.clone());
    }

    fn parse_commands(&mut self, next: Vec<i32>, blueprint: Blueprint) {
        for id in next {
            let command: Command = Command {
                id,
                block_id: *blueprint.flow2bloc(id),
                message: "".to_string(),
                next: blueprint.get_children(id),
            };
            &self.commands.insert(id, command);

            &self.parse_commands(blueprint.children
                                     .get(&id)
                                     .unwrap_or(&(vec![]))
                                     .clone(), blueprint.clone());
        }
    }
}


