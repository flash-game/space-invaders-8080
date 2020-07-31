use std::fs::File;
use std::io;
use std::io::Read;

use crate::cpu::{Cpu, Register};
use crate::memory::SpaceInvadersAddressing;

mod util;

mod cpu;
mod memory;

fn main() -> io::Result<()> {
    let mut arr_h = [0u8; 2048];
    let mut h = File::open("C:/Users/cao/Desktop/invaders/invaders.h")?;
    let h_size = h.read(&mut arr_h)?;

    let mut arr_g = [0u8; 2048];
    let mut g = File::open("C:/Users/cao/Desktop/invaders/invaders.g")?;
    let g_size = g.read(&mut arr_g)?;

    let mut arr_f = [0u8; 2048];
    let mut f = File::open("C:/Users/cao/Desktop/invaders/invaders.f")?;
    let f_size = f.read(&mut arr_f)?;

    let mut arr_e = [0u8; 2048];
    let mut e = File::open("C:/Users/cao/Desktop/invaders/invaders.e")?;
    let e_size = e.read(&mut arr_e)?;

    let addressing = SpaceInvadersAddressing::new(
        Box::new(arr_h), Box::new(arr_g), Box::new(arr_f), Box::new(arr_e));
    let mut cpu = Cpu::new(Box::new(addressing));
    println!("   no        op       af      bc      de      hl      pc      sp  ");
    let mut op_code = 0;
    let times = 0042435;
    // for i in 0..times {
    //     op_code = cpu.next();
    //     if times - i < 100 {
    //         println!("{:07}:    {:#04X}     {:04X}    {:04X}    {:04X}    {:04X}    {:04X}    {:04X}",
    //                  i + 1, op_code, cpu.register.get_af(), cpu.register.get_bc(), cpu.register.get_de(),
    //                  cpu.register.get_hl(), cpu.register.pc, cpu.register.sp);
    //     }
    // }
    loop {
        op_code = cpu.next();
    }
    Ok(())
}
