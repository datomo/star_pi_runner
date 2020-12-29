// mod gpio;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Blueprint {
    root: Vec<i32>,
    blocks: HashMap<String, Value>,
    flow_blocks: HashMap<String, i32>,
    children: HashMap<String, Vec<i32>>,
    default: Value,
    options: Value,
    colors: Value,
    id: i32,
    flow_id: i32,
    file_name: String
}

fn main() {
    const PATH: &str = "config\\config.json";

    let contents: String = fs::read_to_string(PATH).expect("Something went wrong reading the file!");

    let v: Blueprint = serde_json::from_str(contents.as_str()).unwrap();

    println!("With text:\n{}", v.root[1]);
}
