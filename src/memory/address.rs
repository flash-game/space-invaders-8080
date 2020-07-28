use std::cell::{RefCell, RefMut};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

use crate::memory::{Memory, ReadOnly, Video, Work};

pub trait Addressing {
    fn get_mem(&self, addr: u16) -> u8;

    fn set_mem(&mut self, addr: u16, val: u8);

    fn get_word(&self, addr: u16) -> u16 {
        u16::from(self.get_mem(addr)) | (u16::from(self.get_mem(addr + 1)) << 8)
    }

    fn set_word(&mut self, addr: u16, value: u16) {
        self.set_mem(addr, (value & 0xFF) as u8);
        self.set_mem(addr + 1, (value >> 8) as u8)
    }
}