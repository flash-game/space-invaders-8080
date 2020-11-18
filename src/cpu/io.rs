use crate::cpu::Cpu;

pub trait IO {
    fn input(&mut self, cpu: &mut Cpu, byte: u8);

    fn output(&mut self, cpu: &mut Cpu, byte: u8);
}