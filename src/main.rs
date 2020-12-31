use crate::blocks::Button;
use crate::workflow::Workflow;

mod workflow;
mod blocks;
mod event;

fn main() {
    let blueprint:workflow::Blueprint = workflow::load_config();

    for (id, _) in &blueprint.blocks {
        println!("With text:{}", id);
    }


    let mut workflow = Workflow::new();
    workflow.init_blocks(blueprint);

    /*let mut btn = Button::new(32);
    btn.start();*/
}
