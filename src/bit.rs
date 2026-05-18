pub(crate) fn get_bit(value: u8, n: u8) -> u8 {
    (value >> n) & 1
}