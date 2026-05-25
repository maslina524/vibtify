use hidapi::{HidDevice, HidResult};

use crate::dpad::DPadState;
use crate::gamepad::{Gamepad, GamepadState};
use crate::motion::MotionState;
use crate::touch::TPadState;
use crate::bit::get_bit;

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
}