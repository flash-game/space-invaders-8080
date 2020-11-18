use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::io::Read;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use minifb::{Key, Window, WindowOptions};

use crate::cpu::Cpu;
use crate::game::{InvadersLaunch, Launch};
use crate::memory::{AddressBus, TestAddressing};

mod util;

mod cpu;
mod memory;
mod game;


fn main() {
    let launch = InvadersLaunch::new();
    launch.start();
}
