mod dualshock4;
mod dpad;
mod touch;
mod motion;
mod bit;

#[cfg(test)]
mod tests {
    use crate::dualshock4::get_gamepads;

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
            print!("{:08b}", raw[0x0c]);
            println!("\n")
        }
    }

    #[test]
    fn get_state_test() {
        let gamepad = &get_gamepads().unwrap()[0];
        loop {
            let state = gamepad.get_state().unwrap();
            print!("{:?}", state.motion);
            println!("\n")
        }
    }
}
