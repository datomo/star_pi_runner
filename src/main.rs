use crate::workflow::{Manager};
use crate::gpio::loop_gpio;

mod workflow;
mod blocks;
mod button;
mod motor;
mod gpio;

fn main() {
    let blueprint: workflow::Blueprint = workflow::load_config();

    let manager: Manager = Manager::new(blueprint);
    //manager.start();
    loop_gpio();
}
