use std::time::Duration;

use hidapi::{HidApi, HidDevice, HidError};

pub fn get_gamepads() -> Result<Vec<Gamepad>, HidError> {
    let api = HidApi::new()?;
    
    let mut ret: Vec<Gamepad> = Vec::new();

    for device in api.device_list() {
        let vid = device.vendor_id();
        let pid = device.product_id();

        let typ = match (vid, pid) {
            (0x054C, 0x05C4) | (0x054C, 0x09CC) => GamepadType::Dualshock4,
            _ => continue // not a gamepad
        };

        let device = api.open(vid, pid)?;

        ret.push(Gamepad { typ, vid, pid, device });
    }

    Ok(ret)
}

#[derive(Debug)]
pub enum GamepadType {
    Dualshock4
}

#[derive(Debug)]
pub struct Gamepad {
    typ: GamepadType,
    vid: u16,
    pid: u16,
    device: HidDevice
}