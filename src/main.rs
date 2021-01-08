use core::time;
use std::thread;
use std::time::Duration;

use crate::gpio::{Direction, GPIOPin, Pin};
use crate::gui::GuiManager;
use crate::hx711::Hx711;
use crate::workflow::{Manager, BlueprintBlock, Command, CommandMessage, CommandStatus};
use crate::button::Button;
use crate::blocks::Logic;
use gpio_cdev::Chip;

mod workflow;
mod blocks;
mod button;
mod motor;
mod gpio;
mod hx711;
mod gui;
mod scale;

fn main() {
    // hx_tests();

    let blueprint: workflow::Blueprint = workflow::load_config();

    let gui_manager: GuiManager = GuiManager::new(true);
    let manager: Manager = Manager::new(blueprint, gui_manager.get_sender());
    //test_pin();
    manager.start();
    //loop_gpio();

    //gui::main().unwrap();
}

fn test_pin() {
    let mut button = Button::dbgnew(25);


    let test = thread::spawn(move || loop {
        button.eval_command(&Command {
            flow_id: 0,
            block_id: 0,
            message: CommandMessage::Pressed,
            next: vec![],
            status: CommandStatus::Running,
        });
        println!("finished")
    });
    test.join().unwrap();
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
