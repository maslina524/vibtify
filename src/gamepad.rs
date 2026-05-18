use hidapi::{HidApi, HidDevice, HidError};

use crate::dpad::DPadState;
use crate::touch::TPadState;
use crate::bit::get_bit;

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
pub struct GamepadState {
    pub l_stick: (f32, f32),
    pub r_stick: (f32, f32),

    pub l1: bool,
    pub l2: bool,
    pub l3: bool,
    pub r1: bool,
    pub r2: bool,
    pub r3: bool,

    pub l2_force: f32,
    pub r2_force: f32,

    pub dpad: DPadState,
    pub tpad: TPadState,
    
    pub options: bool,
    pub share: bool
}

#[derive(Debug)]
pub struct Gamepad {
    typ: GamepadType,
    vid: u16,
    pid: u16,
    device: HidDevice
}

impl Gamepad {
    pub fn get_raw(&self) -> Result<[u8; 64], HidError> {
        let mut buf = [0u8; 64];
        let bytes_read = self.device.read_timeout(&mut buf, 1000)?;

        Ok(buf[..bytes_read].try_into().unwrap())
    }

    pub fn get_typ(&self) -> &GamepadType {
        &self.typ
    }

    pub fn get_vid(&self) -> u16 {
        self.vid
    }

    pub fn get_pid(&self) -> u16 {
        self.pid
    }

    pub fn get_state(&self) -> Result<GamepadState, HidError> {
        let raw = self.get_raw()?;
        
        let l_stick = (
            raw[1] as f32 / 127.5 - 1.,
            -(raw[2] as f32 / 127.5 - 1.)
        );
        let r_stick = (
            raw[3] as f32 / 127.5 - 1.,
            -(raw[4] as f32 / 127.5 - 1.)
        );

        let tpad = TPadState::from_buf(&raw);

        let ret = GamepadState {
            l_stick,
            r_stick,

            l1: get_bit(raw[6], 0) == 1,
            l2: get_bit(raw[6], 2) == 1,
            l3: get_bit(raw[6], 6) == 1,
            r1: get_bit(raw[6], 1) == 1,
            r2: get_bit(raw[6], 3) == 1,
            r3: get_bit(raw[6], 7) == 1,

            l2_force: raw[8] as f32 / 255.,
            r2_force: raw[9] as f32 / 255.,

            dpad: DPadState::from_byte(raw[5]),
            tpad,

            options: get_bit(raw[6], 5) == 1,
            share: get_bit(raw[6], 4) == 1,
        };
        Ok(ret)
    }
}