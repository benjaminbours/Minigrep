extern crate rppal;

use std::error::Error;
use std::process::Command;
// use std::thread;
// use std::time::Duration;

use rppal::gpio::{Gpio, Trigger};

// Gpio uses BCM pin numbering. BCM GPIO 3 is tied to physical pin 5.
const GPIO_BUTTON: u8 = 3;

fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let mut pin = Gpio::new()?.get(GPIO_BUTTON)?.into_input();
    pin.set_interrupt(Trigger::RisingEdge).unwrap();
    let level = pin.poll_interrupt(true, None).unwrap().expect("Error");

    Command::new("shutdown").args(&["-h", "now"]).spawn().expect("Can't shutdown the system");

    Ok(())
}