use crate::workflow::BlueprintBlock;
use crate::button::Button;

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

    let _btn = Button::new(BlueprintBlock {
        id: 0,
        name: "".to_string(),
        pins: vec![3],
        options: Default::default(),
    });
}
