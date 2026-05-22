vibtify
===========================

[<img alt="github" src="https://img.shields.io/badge/github-maslina524/vibtify-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/maslina524/vibtify)
[<img alt="crates.io" src="https://img.shields.io/crates/v/vibtify.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/vibtify)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-vibtify-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/vibtify)

**vibtify** is a Rust crate for low-latency gamepad interaction – both receiving input data (buttons, axes, triggers) and controlling output features such as vibration (rumble) and LEDs. It provides a clean, cross-platform API for working with gamepads in games.

## Supported Gamepads
- **Dualshock4**

## Example
```rust
let gamepad = &get_gamepads().unwrap()[0];
loop {
    let state = gamepad.get_state().unwrap();
    if state.square {
        println!("The square button is pressed!");
        gamepad.set_rumble(125, 125);
    } else {
        gamepad.set_rumble(0, 0);
    }
}
```
