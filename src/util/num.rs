pub struct U16Util {}

impl U16Util {
    pub fn from_le_bytes(f: u8, s: u8) -> u16 {
        (f as u16) | ((s as u16) << 8)
    }
}