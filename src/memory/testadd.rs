use std::cell::RefCell;
use std::rc::Rc;

use crate::memory::{AddressBus};

pub struct TestAddressing {
    rom: Rc<RefCell<Vec<u8>>>,
}

impl AddressBus for TestAddressing {
    fn get_mem(&self, addr: u16) -> u8 {
        self.rom.borrow()[addr as usize]
    }

    fn set_mem(&mut self, addr: u16, val: u8) {
        self.rom.borrow_mut()[addr as usize] = val
    }
}

impl TestAddressing {
    pub fn new(rom: Rc<RefCell<Vec<u8>>>) -> Self {
        Self {
            rom
        }
    }
}