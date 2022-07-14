use crate::cpu::Cpu;
use crate::cpu::Register;

pub trait IO {
    fn input(&mut self, cpu: &mut Register, byte: u8);

    fn output(&mut self, cpu: &mut Cpu, byte: u8);
}
