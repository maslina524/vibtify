/// A structure that stores a snapshot of the D-pad state.
#[derive(Debug)]
pub enum DPadState {
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

impl DPadState {
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