use scroll::{self, Pread};

/// A structure that stores a snapshot of the gyroscope and motion state of the gamepad.
#[derive(Debug)]
pub struct MotionState {
    motion_x: i16,
    motion_y: i16,
    motion_z: i16,

    gyro_x: i16,
    gyro_y: i16,
    gyro_z: i16,
}

impl MotionState {
    pub(crate) fn from_buf(buf: &[u8; 64]) -> Self {
        let motion_x = buf.pread_with::<i16>(0x0d, scroll::BE).unwrap();
        let motion_y = buf.pread_with::<i16>(0x0f, scroll::BE).unwrap();
        let motion_z = buf.pread_with::<i16>(0x11, scroll::BE).unwrap();
        let gyro_x = buf.pread_with::<i16>(0x13, scroll::BE).unwrap();
        let gyro_y = buf.pread_with::<i16>(0x15, scroll::BE).unwrap();
        let gyro_z = buf.pread_with::<i16>(0x17, scroll::BE).unwrap();

        Self {
            motion_x, motion_y, motion_z, 
            gyro_x, gyro_y, gyro_z
        }
    }
}