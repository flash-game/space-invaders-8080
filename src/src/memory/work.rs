use crate::memory::Memory;

/// 工作内存
pub struct Work {
    /// 读取的数据
    data: Box<[u8; 1024]>,
    /// 地址偏移量
    ofs: u16,
}

impl Memory for Work {
    fn get(&self, addr: u16) -> u8 {
        self.data[(addr - self.ofs) as usize]
    }

    fn set(&mut self, addr: u16, val: u8) {
        self.data[(addr - self.ofs) as usize] = val
    }
}

impl Work {
    pub fn init(ofs: u16, data: Box<[u8; 1024]>) -> Work {
        Work {
            data,
            ofs,
        }
    }
}