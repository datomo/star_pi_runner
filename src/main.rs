use crate::workflow::{Manager};

mod workflow;
mod blocks;
mod event;
mod button;
mod motor;

fn main() {
    let blueprint: workflow::Blueprint = workflow::load_config();

    for (id, _) in &blueprint.blocks {
        println!("With text: {}", id);
    }


    let manager: Manager = Manager::new(blueprint);
    manager.start();
}
