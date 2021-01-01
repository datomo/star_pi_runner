use crate::button::Button;
use crate::motor::Motor;
use crate::workflow::BlueprintBlock;
use crate::blocks::Trigger;

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

    let mut btn = Button::new(BlueprintBlock {
        id: 0,
        name: "".to_string(),
        pins: vec![3],
        options: Default::default(),
    });

    let motor = Motor::new(BlueprintBlock {
        id: 2,
        name: "".to_string(),
        pins: vec![7, 10],
        options: Default::default(),
    });

    btn.add_sender(motor.get_sender());

    btn.event_loop();
}
