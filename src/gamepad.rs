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

fn get_bit(value: u8, n: u8) -> u8 {
    (value >> n) & 1
}

#[derive(Debug)]
pub enum GamepadType {
    Dualshock4
}

#[derive(Debug)]
pub enum CrossState {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    None
}

impl CrossState {
    pub fn from_byte(byte: u8) -> Self {
        return match byte {
            0x00 => Self::Up,
            0x04 => Self::Down,
            0x06 => Self::Left,
            0x02 => Self::Right,
            0x07 => Self::UpLeft,
            0x01 => Self::UpRight,
            0x05 => Self::DownLeft,
            0x03 => Self::DownRight,
            _ => Self::None
        }
    }
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

    pub cross: CrossState,
    
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

        let ret = GamepadState {
            l_stick,
            r_stick,
            l1: get_bit(raw[6], 0) == 1,
            l2: get_bit(raw[6], 2) == 1,
            l3: get_bit(raw[6], 6) == 1,
            r1: get_bit(raw[6], 1) == 1,
            r2: get_bit(raw[6], 3) == 1,
            r3: get_bit(raw[6], 7) == 1,
            cross: CrossState::from_byte(raw[5]),
            options: get_bit(raw[6], 5) == 1,
            share: get_bit(raw[6], 4) == 1,
        };
        Ok(ret)
    }
}