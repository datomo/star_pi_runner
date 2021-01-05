use core::time;
use std::thread;

use crate::gpio::loop_gpio;
use crate::gui::GuiManager;
use crate::hx711::Hx711;
use crate::workflow::Manager;

mod workflow;
mod blocks;
mod button;
mod motor;
mod gpio;
mod hx711;
mod gui;
mod scale;

fn main() {
    let blueprint: workflow::Blueprint = workflow::load_config();

    let gui_manager: GuiManager = GuiManager::new(false);
    let manager: Manager = Manager::new(blueprint, gui_manager.get_sender());
    manager.start();
    loop_gpio();

    //gui::main().unwrap();
}

fn hx_tests() {
    let mut hx711 = Hx711::new(20, 21, 128);

    let value = hx711.get_value(5);
    // value it shows / value it should be = reference
    hx711.set_reference(85_750 / 264);
    hx711.reset();

    /*println!("waiting");
    thread::sleep(time::Duration::from_secs(3));

    hx711.tare(15);*/

    loop {
        println!("weight: {}", hx711.get_value(5));
        thread::sleep(time::Duration::from_millis(100));
    }
}
