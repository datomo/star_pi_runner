use crate::button::Button;
use crate::motor::Motor;
use crate::workflow::{Manager, BlueprintBlock};
use crate::blocks::Trigger;
use std::sync::mpsc::channel;

mod workflow;
mod blocks;
mod event;
mod button;
mod motor;

fn main() {
    let blueprint: workflow::Blueprint = workflow::load_config();

    for (id, _) in &blueprint.blocks {
        println!("With text:{}", id);
    }


    /*let mut workflow = Workflow::new();
    workflow.init_blocks(blueprint);*/

    //btn.add_sender(motor.get_sender());
    //println!("here");
    //btn.event_loop();

    let manager:Manager = Manager::new(blueprint);
    manager.start();

}
