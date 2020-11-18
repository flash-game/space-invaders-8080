use std::sync::{Arc, RwLock};

use crate::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_SIZE: usize = 7168;

// Video RAM
pub struct Video {
    pub data: Rc<RefCell<Vec<u8>>>,
    /// 地址偏移量
    ofs: u16,
}

impl Memory for Video {
    fn get(&self, addr: u16) -> u8 {
        self.data.borrow()[(addr - self.ofs) as usize]
    }

    fn set(&mut self, addr: u16, val: u8) {
        self.data.borrow_mut()[(addr - self.ofs) as usize] = val;
    }
}

impl Video {
    pub fn init(ofs: u16, data: Rc<RefCell<Vec<u8>>>) -> Video {
        Video {
            data,
            ofs,
        }
    }
}