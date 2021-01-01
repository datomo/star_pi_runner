use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::motor::Motor;
use crate::button::Button;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlueprintBlock {
    pub id: i32,
    pub name: String,
    pub pins: Vec<i32>,
    pub options: HashMap<String, String>,
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
pub struct Blueprint {
    pub root: Vec<i32>,
    pub blocks: HashMap<i32, BlueprintBlock>,
    pub flow_blocks: HashMap<i32, i32>,
    pub children: HashMap<i32, Vec<i32>>,
    default: Value,
    options: Value,
    colors: Value,
    id: i32,
    flow_id: i32,
    file_name: String,
}

pub struct Workflow {
    pub blocks: HashMap<i32, BlockWrapper>
}

impl Workflow {
    pub fn new() -> Self {
        Workflow { blocks: Default::default() }
    }

    pub fn init_blocks(&mut self, _blueprint: Blueprint) {}
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
