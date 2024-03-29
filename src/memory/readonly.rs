use crate::memory::Memory;

/// 只读内存
pub struct ReadOnly {
    /// 读取的数据
    pub data: Box<[u8; 2048]>,
    /// 地址偏移量
    ofs: u16,
}

impl Memory for ReadOnly {
    fn get(&self, addr: u16) -> u8 {
        self.data[(addr - self.ofs) as usize]
    }

    /// ReadOnly 所以不允许修改
    fn set(&mut self, _addr: u16, _val: u8) {}
}

impl ReadOnly {
    pub fn init(ofs: u16, data: Box<[u8; 2048]>) -> ReadOnly {
        ReadOnly { data, ofs }
    }
}
