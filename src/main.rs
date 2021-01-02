use crate::workflow::{Manager};

mod workflow;
mod blocks;
mod button;
mod motor;

fn main() {
    let blueprint: workflow::Blueprint = workflow::load_config();

    let manager: Manager = Manager::new(blueprint);
    manager.start();
}
