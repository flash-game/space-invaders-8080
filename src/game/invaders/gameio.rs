use minifb::Key;

use crate::cpu::{Cpu, IO};

pub struct InvadersIO {
    input_temp: Option<Key>,
}

impl InvadersIO {
    pub fn new() -> Self {
        Self { input_temp: None }
    }

    pub fn clean_temp(&mut self) {
        self.input_temp = None;
    }

    pub fn set_input_temp(&mut self, key: Key) {
        self.input_temp = Some(key)
    }
}

impl IO for InvadersIO {
    fn input(&mut self, cpu: &mut Cpu, byte: u8) {
        let input_key = match self.input_temp {
            None => {
                //println!("执行input {:X}", byte);
                return;
            }
            Some(i) => i,
        };
        let reg_a = match byte {
            1 => match input_key {
                Key::C => Some(0b0000_0001),
                Key::Enter => Some(0b0000_0100),
                Key::Space => Some(0b0001_0000),
                Key::Left => Some(0b0010_0000),
                Key::Right => Some(0b0100_0000),
                _ => None,
            },
            2 => match input_key {
                // Key::C => { Some(0b0000_0001) }
                // Key::Enter => { Some(0b0000_0100) }
                // Key::Space => { Some(0b0001_0000) }
                // Key::Left => { Some(0b0010_0000) }
                // Key::Right => { Some(0b0100_0000) }
                _ => Some(0b1000_0000),
            },
            3 => None,
            _ => None,
        };
        match reg_a {
            Some(a) => cpu.register.a = a,
            _ => {}
        }
        self.input_temp = None;
        //println!("执行input {:X}", byte);
    }

    fn output(&mut self, _cpu: &mut Cpu, _byte: u8) {
        println!("执行output");
    }
}
