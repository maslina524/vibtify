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