use crate::game::{InvadersLaunch, Launch};

mod util;

mod cpu;
mod game;
mod memory;

fn main() {
    let launch = InvadersLaunch::new();
    launch.start();
}
