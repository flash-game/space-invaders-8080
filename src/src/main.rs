use std::fs::File;
use std::io;
use std::io::Read;

mod util;

mod cpu;
mod memory;

fn main() -> io::Result<()> {
    let a: u8 = 0x70;
    let new_a: u8 = a.wrapping_sub(0x71);

    println!("result:{:X} 借位 {}", new_a, a < new_a);
    Ok(())
}
