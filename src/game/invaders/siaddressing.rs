use std::cell::RefCell;
use std::rc::Rc;

use crate::memory::{AddressBus, Memory, ReadOnly, Video, Work};

pub struct InvadersAddressBus {
    read_only_h: ReadOnly,
    read_only_g: ReadOnly,
    read_only_f: ReadOnly,
    read_only_e: ReadOnly,
    work_ram: Work,
    pub video_ram: Video,
    work_ram2: Work,
}

impl AddressBus for InvadersAddressBus {
    fn get_mem(&self, addr: u16) -> u8 {
        let value = match addr {
            0x0000..=0x07ff => self.read_only_h.get(addr),
            0x0800..=0x0fff => self.read_only_g.get(addr),
            0x1000..=0x17ff => self.read_only_f.get(addr),
            0x1800..=0x1fff => self.read_only_e.get(addr),
            0x2000..=0x23ff => self.work_ram.get(addr),
            0x2400..=0x3fff => self.video_ram.get(addr),
            0x4000..=0xFFFF => self.work_ram2.get(addr),
            _n => panic!("address not support.")
        };
        value
    }

    fn set_mem(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x07ff => self.read_only_h.set(addr, val),
            0x0800..=0x0fff => self.read_only_g.set(addr, val),
            0x1000..=0x17ff => self.read_only_f.set(addr, val),
            0x1800..=0x1fff => self.read_only_e.set(addr, val),
            0x2000..=0x23ff => self.work_ram.set(addr, val),
            0x2400..=0x3fff => self.video_ram.set(addr, val),
            0x4000..=0xFFFF => self.work_ram2.set(addr, val),
            _n => println!("Unsupport Address {:X}", _n)
        }
    }
}

impl InvadersAddressBus {
    pub fn new(h_arr: Box<[u8; 2048]>,
               g_arr: Box<[u8; 2048]>,
               f_arr: Box<[u8; 2048]>,
               e_arr: Box<[u8; 2048]>,
               video_arr: Rc<RefCell<Vec<u8>>>,
    ) -> Self {
        Self {
            read_only_h: ReadOnly::init(0, h_arr),
            read_only_g: ReadOnly::init(0x0800, g_arr),
            read_only_f: ReadOnly::init(0x1000, f_arr),
            read_only_e: ReadOnly::init(0x1800, e_arr),
            work_ram: Work::init(0x2000, Box::new([0u8; 1024])),
            video_ram: Video::init(0x2400, video_arr),
            work_ram2: Work::init(0x4000, Box::new([0u8; 1024])),
        }
    }
}