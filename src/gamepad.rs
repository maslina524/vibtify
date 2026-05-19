use hidapi::{HidApi, HidResult};

use crate::dpad::DPadState;
use crate::motion::MotionState;
use crate::touch::TPadState;
use crate::dualshock4::Dualshock4;

#[derive(Debug)]
pub struct GamepadState {
    pub l_stick: (f32, f32),
    pub r_stick: (f32, f32),

    pub l1: bool,
    pub l2: bool,
    pub l3: bool,
    pub r1: bool,
    pub r2: bool,
    pub r3: bool,

    pub square: bool,
    pub triangle: bool,
    pub circle: bool,
    pub cross: bool,

    pub l2_force: f32,
    pub r2_force: f32,

    pub dpad: DPadState,
    pub tpad: TPadState,
    
    pub options: bool,
    pub share: bool,
    pub home: bool,
    
    pub motion: MotionState
}

pub trait Gamepad: std::fmt::Debug {
    fn get_raw(&self) -> HidResult<[u8; 64]>;
    fn get_state(&self) -> HidResult<GamepadState>;
    fn get_vid(&self) -> u16;
    fn get_pid(&self) -> u16;
    fn rumble(&self, l_motor: u8, r_motor: u8) -> HidResult<()>;
    fn set_lightbar(&self, r: u8, g: u8, b: u8) -> HidResult<()>;
}

pub enum GamepadType {
    Dualshock4
}

pub fn get_gamepads() -> HidResult<Vec<Box<dyn Gamepad>>> {
    let api = HidApi::new()?;
    let mut ret: Vec<Box<dyn Gamepad>> = Vec::new();

    for device in api.device_list() {
        let vid = device.vendor_id();
        let pid = device.product_id();

        let typ = match (vid, pid) {
            (0x054C, _) => { // SONY
                match pid {
                    0x05C4 | 0x09CC => GamepadType::Dualshock4,
                    _ => continue
                }
            },
            _ => continue
        };
        
        let device = api.open(vid, pid)?;
        
        ret.push(
            match typ {
                GamepadType::Dualshock4 => Box::new(Dualshock4 { vid, pid, device })
            }
        );
    }

    Ok(ret)
}