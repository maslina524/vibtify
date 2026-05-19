#![allow(dead_code)]

//! **vibtify** is a Rust crate for low-latency gamepad interaction – both receiving input data (buttons, axes, triggers) and controlling output features such as vibration (rumble) and LEDs. It provides a clean, cross-platform API for working with gamepads in games.
//!
//! ## Supported Gamepads
//! - **Dualshock4**
//!
//! ## Example
//! ```rust
//! let gamepad = &get_gamepads().unwrap()[0];
//! loop {
//!     let state = gamepad.get_state().unwrap();
//!     if state.square {
//!         println!("The square button is pressed!");
//!         gamepad.set_rumble(125, 125);
//!     } else {
//!         gamepad.set_rumble(0, 0);
//!     }
//! }
//! ```

pub mod gamepad;
pub mod dualshock4;
pub mod dpad;
pub mod touch;
pub mod motion;
mod bit;

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use crate::gamepad::get_gamepads;

    #[test]
    fn get_gamepads_test() {
        let gamepads = get_gamepads();
        println!("{gamepads:#?}")
    }

    #[test]
    fn get_raw_test() {
        let gamepad = &get_gamepads().unwrap()[0];
        loop {
            let raw = gamepad.get_raw().unwrap();
            // for byte in raw {
            //     print!("{byte:02x} ")
            // }
            print!("{:08b}", raw[0x05]);
            println!("\n")
        }
    }

    #[test]
    fn get_right() {
        let gamepad = &get_gamepads().unwrap()[0];
        loop {
            let state = gamepad.get_state().unwrap();
            print!(
                "triangle: {} | circle: {} | cross: {} | square: {}",
                state.triangle, state.circle, state.cross, state.square 
            );
            println!("\n")
        }
    }

    #[test]
    fn get_state_test() {
        let gamepad = &get_gamepads().unwrap()[0];
        loop {
            let state = gamepad.get_state().unwrap();
            print!("{:?}", state);
            println!("\n")
        }
    }

    #[test]
    fn rumble_test() {
        let gamepad = &get_gamepads().unwrap()[0];
        println!("{:?}", gamepad.set_rumble(255, 255));
    }

    #[test]
    fn lightbar_test() {
        let gamepad = &get_gamepads().unwrap()[0];
        for x in 0..1000 {
            let value = (((x as f64 / 10.).sin() + 1.) * 127.5) as u8;
            println!("{value}");
            if let Err(e) = gamepad.set_lightbar(value, 0, 0) {
                panic!("{e:?}");
            };
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
