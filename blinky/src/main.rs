#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use {defmt_rtt as _, panic_probe as _};

mod modular;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Starting tasks on STM32H7!");

    let led_red = Output::new(p.PB14, Level::High, Speed::Low);
    let led_blue = Output::new(p.PE1, Level::High, Speed::Low);
    let led_green = Output::new(p.PB0, Level::High, Speed::Low);

      // Spawn the LED Redtask
    info!("Starting LED Red toggle task");
    unwrap!(spawner.spawn(modular::toogle_led_red(led_red)));

      // Spawn the LED Blue task
    info!("Starting LED Blue toggle task");
    unwrap!(spawner.spawn(modular::toogle_led_blue(led_blue)));

      // Spawn the LED Green task
    info!("Starting LED Green toggle task");
    unwrap!(spawner.spawn(modular::toogle_led_green(led_green)));
}
