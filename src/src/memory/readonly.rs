use crate::memory::{Addressing, Memory, Work};

/// 只读内存
pub struct ReadOnly {
    /// 读取的数据
    data: Box<[u8; 2048]>,
    /// 地址偏移量
    ofs: u16,
}

impl Memory for ReadOnly {
    fn get(&self, addr: u16) -> u8 {
        self.data[(addr - self.ofs) as usize]
    }

    /// ReadOnly 所以不允许修改
    fn set(&mut self, addr: u16, val: u8) {}
}

impl ReadOnly {
    pub fn init(ofs: u16, data: Box<[u8; 2048]>) -> ReadOnly {
        ReadOnly {
            data,
            ofs,
        }
    }
}


