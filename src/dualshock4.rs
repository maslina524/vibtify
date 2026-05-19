use hidapi::{HidDevice, HidResult};

use crate::dpad::DPadState;
use crate::gamepad::{Gamepad, GamepadState};
use crate::motion::MotionState;
use crate::touch::TPadState;
use crate::bit::get_bit;

#[derive(Debug)]
pub struct Dualshock4 {
    pub(crate) vid: u16,
    pub(crate) pid: u16,
    pub(crate) device: HidDevice
}

impl Gamepad for Dualshock4 {
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

            square: get_bit(raw[5], 4) == 1,
            triangle: get_bit(raw[5], 7) == 1,
            circle: get_bit(raw[5], 6) == 1,
            cross: get_bit(raw[5], 5) == 1,

            l2_force: raw[8] as f32 / 255.,
            r2_force: raw[9] as f32 / 255.,

            dpad: DPadState::from_byte(raw[5]),
            tpad,

            options: get_bit(raw[6], 5) == 1,
            share: get_bit(raw[6], 4) == 1,
            home: get_bit(raw[0x07], 0) == 1,

            motion: MotionState::from_buf(&raw)
        };
        Ok(ret)
    }

    fn rumble(&self, l_motor: u8, r_motor: u8) -> HidResult<()> {
        let mut buf = vec![0u8; 16];
        buf[0] = 0x05;
        buf[1] = 0x01;

        buf[4] = l_motor; 
        buf[5] = r_motor;

        self.device.write(&buf)?;
        
        Ok(())
    }
}