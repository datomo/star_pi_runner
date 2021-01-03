use std::{thread, time};
use std::io::Error;

use gpio::{GpioIn, GpioOut};
use gpio::dummy::DummyGpioIn;
use gpio::sysfs::SysFsGpioInput;

fn example() {

// Let's open GPIO23 and -24, e.g. on a Raspberry Pi 2.
    let mut gpio23 = gpio::sysfs::SysFsGpioInput::open(23).unwrap();
    let mut gpio24 = gpio::sysfs::SysFsGpioOutput::open(24).unwrap();

// GPIO24 will be toggled every second in the background by a different thread
    let mut value = false;
    thread::spawn(move || loop {
        gpio24.set_value(value).expect("could not set gpio24");
        thread::sleep(time::Duration::from_millis(1000));
        value = !value;
    });

// The main thread will simply display the current value of GPIO23 every 100ms.
    loop {
        println!("GPIO23: {:?}", gpio23.read_value().unwrap());
        thread::sleep(time::Duration::from_millis(100));
    }
}



pub fn loop_gpio() {
    if false {
        let mut gpio25 = gpio::sysfs::SysFsGpioInput::open(25).unwrap();
        loop {
            println!("GPIO25: {:?}", gpio25.read_value().unwrap());
            thread::sleep(time::Duration::from_millis(100));
        }
    } else {
        let mut gpio25 = gpio::dummy::DummyGpioIn::new(|| true);
        loop {
            println!("GPIO25: {:?}", gpio25.read_value().unwrap());
            thread::sleep(time::Duration::from_millis(100));
        }
    }

}
