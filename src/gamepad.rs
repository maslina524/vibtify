use rusb::*;

pub fn get_gamepads() -> Vec<Gamepad> {
    let context = Context::new().expect("Failed to create libusb context");
    let devices = context.devices().expect("Failed to enumerate devices");
    
    let mut ret: Vec<Gamepad> = Vec::new();

    for device in devices.iter() {
        let device_desc = device.device_descriptor().unwrap();
        let vendor = device_desc.vendor_id();
        let product = device_desc.product_id();

        let typ = match (vendor, product) {
            (0x054C, 0x05C4) | (0x054C, 0x09CC) => GamepadType::Dualshock4,
            _ => continue // not a gamepad
        };

        ret.push(Gamepad { typ, vendor, product });
    }
    ret
}

#[derive(Debug)]
pub enum GamepadType {
    Dualshock4
}

#[derive(Debug)]
pub struct Gamepad {
    typ: GamepadType,
    vendor: u16,
    product: u16
}