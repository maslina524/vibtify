mod gamepad;

#[cfg(test)]
mod tests {
    use crate::gamepad::get_gamepads;
    use std::{process::Command, time::Duration};

    #[test]
    fn get_gamepads_test() {
        let gamepads = get_gamepads();
        println!("{gamepads:#?}")
    }

    #[test]
    fn get_raw_test() {
        let gamepad = &get_gamepads().unwrap()[0];
        loop {
            for byte in gamepad.get_raw().unwrap() {
                print!("{byte:02x} ")
            }
            println!("\n")
        }
    }
}
