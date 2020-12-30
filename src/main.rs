use crate::blocks::Button;

mod workflow;
mod blocks;

fn main() {
    let blueprint:workflow::Blueprint = workflow::load_config();

    for (id, _) in &blueprint.blocks {
        println!("With text:{}", id);
    }


    blueprint.build();

    let mut btn = Button::new(32);
    btn.start();
}
