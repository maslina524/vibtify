const ACTIVE_BLOCK_1: usize = 0x23;
const DATA_BLOCK_A_1: usize = 0x24;
const DATA_BLOCK_B_1: usize = 0x25;
const DATA_BLOCK_C_1: usize = 0x26;

const ACTIVE_BLOCK_2: usize = 0x27;
const DATA_BLOCK_A_2: usize = 0x28;
const DATA_BLOCK_B_2: usize = 0x29;
const DATA_BLOCK_C_2: usize = 0x2a;

#[derive(Debug)]
pub struct TPadState {
    is_touched_1: bool,
    xy_1: Option<(u16, u16)>,

    is_touched_2: bool,
    xy_2: Option<(u16, u16)>,
}

impl TPadState {
    pub fn from_buf(buf: &[u8; 64]) -> Self {
        let active_1 = buf[ACTIVE_BLOCK_1] < 0x80;

        let xy_1 = if active_1 {
            Some((
                decode_touch_x(false, buf),
                decode_touch_y(false, buf)
            ))
        } else {
            None
        };

        let active_2 = buf[ACTIVE_BLOCK_2] < 0x80;

        let xy_2 = if active_2 {
            Some((
                decode_touch_x(true, buf),
                decode_touch_y(true, buf)
            ))
        } else {
            None
        };

        Self {
            is_touched_1: active_1,
            xy_1,

            is_touched_2: active_2,
            xy_2,
        }
    }
}

fn decode_touch_x(is_2: bool, buf: &[u8]) -> u16 {
    let a = if is_2 { DATA_BLOCK_A_2 } else { DATA_BLOCK_A_1 };
    let b = if is_2 { DATA_BLOCK_B_2 } else { DATA_BLOCK_B_1 };

    let block_a = u16::from(buf[a]);
    let block_b = u16::from(buf[b]);

    ((block_b & 0x0f) << 8) | block_a
}

fn decode_touch_y(is_2: bool, buf: &[u8]) -> u16 {
    let b = if is_2 { DATA_BLOCK_B_2 } else { DATA_BLOCK_B_1 };
    let c = if is_2 { DATA_BLOCK_C_2 } else { DATA_BLOCK_C_1 };

    let block_b = u16::from(buf[b]);
    let block_c = u16::from(buf[c]);

    (block_c << 4) | ((block_b & 0xf0) >> 4)
}