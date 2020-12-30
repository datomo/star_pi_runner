mod workflow;
mod actions;
mod triggers;

fn main() {
    let blueprint:workflow::Blueprint = workflow::load_config();

    for (id, _) in &blueprint.blocks {
        println!("With text:{}", id);
    }


    blueprint.build()
}
