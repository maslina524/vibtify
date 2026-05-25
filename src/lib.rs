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
pub mod dualsense;
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
        let gamepad = &mut get_gamepads().unwrap()[0];
        for x in 0..1000 {
            let value = (((x as f64 / 10.).sin() + 1.) * 127.5) as u8;
            println!("{value}");
            if let Err(e) = gamepad.set_lightbar(value, 0, 0) {
                panic!("{e:?}");
            };
            thread::sleep(time::Duration::from_millis(10));
        }
    }

    fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
        let h = h % 360.0;
        
        let c = v * s; // Chroma
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        
        let (r1, g1, b1) = match h {
            0.0..=60.0 => (c, x, 0.0),
            60.0..=120.0 => (x, c, 0.0),
            120.0..=180.0 => (0.0, c, x),
            180.0..=240.0 => (0.0, x, c),
            240.0..=300.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        
        let r = ((r1 + m) * 255.0).round() as u8;
        let g = ((g1 + m) * 255.0).round() as u8;
        let b = ((b1 + m) * 255.0).round() as u8;
        
        (r, g, b)
    }

    #[test]
    fn rainbow_lightbar_test() {
        let gamepad = &mut get_gamepads().unwrap()[0];
        for x in 0..1000 {
            let (r, g, b) = hsv_to_rgb((x % 360) as f64, 1., 1.);
            println!("{r}-{g}-{b}");
            if let Err(e) = gamepad.set_lightbar(r, g, b) {
                panic!("{e:?}");
            };
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
