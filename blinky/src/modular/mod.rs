// Master file for the modular project.
/*!
 * -----------------------------------------------------------------------------
 *  Project     : Master file for the modular project.
 *  File        : mod.rs
 *  Created by  : Everton Oriente
 *  Date        : 2025-07-22
 *  * -----------------------------------------------------------------------------
 *  Description :
 *      The master module is responsible about to add or remove modules from the project.
 *
 *  Target MCU  : Raspberry Pi Pico W (RP2040 and CYW43)
 *  Framework   : Embassy, no_std
 *
 */

// Import required crates and modules

mod blinky_green_led;
mod blinky_blue_led;
mod blinky_red_led;

pub(crate) use blinky_green_led::*;
pub(crate) use blinky_blue_led::*;
pub(crate) use blinky_red_led::*;
