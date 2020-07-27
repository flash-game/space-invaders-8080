use crate::memory::Memory;

// Video RAM
pub struct Video {
    data: Box<[u8; 7168]>,
    /// 地址偏移量
    ofs: u16,
}

impl Memory for Video {
    fn get(&self, addr: u16) -> u8 {
        self.data[(addr - self.ofs) as usize]
    }

    /// ReadOnly 所以不允许修改
    fn set(&mut self, addr: u16, val: u8) {
        self.data[(addr - self.ofs) as usize] = val
    }
}

impl Video {
    pub fn init(ofs: u16, data: Box<[u8; 7168]>) -> Video {
        Video {
            data,
            ofs,
        }
    }
}