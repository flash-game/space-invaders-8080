use crate::cpu::register::Register;
use crate::motherboard::MotherBoard;
use std::borrow::Borrow;

/// Abstraction of Intel 8080
struct Cpu {
    register: Register,
    mb: MotherBoard,
}

impl Cpu {
    fn get_byte(&mut self) -> u8 {
        let op_addr = self.register.pc;
        let byte = self.mb.get_mem(op_addr);
        self.register.pc += 1;
        byte
    }

    /// 下一步指令
    pub fn next(&mut self) {
        let op_code = self.get_byte();
    }
}
