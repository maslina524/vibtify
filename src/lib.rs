mod gamepad;

#[cfg(test)]
mod tests {
    use crate::gamepad::get_gamepads;

    #[test]
    fn get_gamepads_test() {
        get_gamepads();
    }
}
