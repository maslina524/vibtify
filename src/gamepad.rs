use rusb::*;

pub fn get_gamepads() {
    let context = Context::new().expect("Failed to create libusb context");
    
    let devices = context.devices().expect("Failed to enumerate devices");
    
    for device in devices.iter() {
        let device_desc = device.device_descriptor().unwrap();
        println!("Device: {:04x}:{:04x}", 
                device_desc.vendor_id(), 
                device_desc.product_id());
    }
}