use serde_json::{Value};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blueprint {
    pub root: Vec<i32>,
    pub blocks: HashMap<i32, Value>,
    pub flow_blocks: HashMap<i32, i32>,
    pub children: HashMap<i32, Vec<i32>>,
    default: Value,
    options: Value,
    colors: Value,
    id: i32,
    flow_id: i32,
    file_name: String,
}

impl Blueprint {
    pub fn build(&self) {
        for id in &self.root {
            println!("{}", self.flow2block(id));
        }
    }

    fn flow2block(&self, id:&i32) -> &i32 {
        &self.flow_blocks[id]
    }
}

pub fn load_config() -> Blueprint {
    const PATH: &str = "config\\config.json";

    let contents: String = fs::read_to_string(PATH).expect("Something went wrong reading the file!");

    let blueprint: Blueprint = serde_json::from_str(contents.as_str()).unwrap();

    blueprint
}
