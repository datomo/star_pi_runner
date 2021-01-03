use crate::workflow::{Manager};
use crate::gpio::loop_gpio;
use crate::hx711::Hx711;

mod workflow;
mod blocks;
mod button;
mod motor;
mod gpio;
mod hx711;

fn main() {
    let blueprint: workflow::Blueprint = workflow::load_config();

    let manager: Manager = Manager::new(blueprint);
    //manager.start();
    //loop_gpio();
    let mut hx711 = Hx711::new(20, 21, 128);
    loop {
        println!("weight: {}", hx711.get_units(10));
    }

}
