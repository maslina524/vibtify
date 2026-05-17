mod gamepad;

#[cfg(test)]
mod tests {
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
            println!("{:#?}", gamepad.get_raw());
        }
    }
}
