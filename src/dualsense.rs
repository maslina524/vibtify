use hidapi::{HidDevice, HidResult};

use crate::gamepad::{Gamepad, GamepadState};

#[derive(Debug)]
pub struct Dualsense {
    pub(crate) vid: u16,
    pub(crate) pid: u16,
    pub(crate) device: HidDevice,

    pub(crate) led_r: u8,
    pub(crate) led_g: u8,
    pub(crate) led_b: u8,
}

impl Gamepad for Dualsense {
    fn get_raw(&self) -> HidResult<[u8; 64]> {
        let mut buf = [0u8; 64];
        let bytes_read = self.device.read_timeout(&mut buf, 1000)?;

        Ok(buf[..bytes_read].try_into().unwrap())
    }

    fn get_vid(&self) -> u16 {
        self.vid
    }

    fn get_pid(&self) -> u16 {
        self.pid
    }

    fn get_state(&self) -> HidResult<GamepadState> {
        todo!();
    }

    fn set_rumble(&self, l_motor: u8, r_motor: u8) -> HidResult<()> {
        todo!();
    }

    fn get_lightbar(&self) -> HidResult<(u8, u8, u8)> {
        todo!();
    }

    fn set_lightbar(&mut self, r: u8, g: u8, b: u8) -> HidResult<()> {
        todo!();
    }
}