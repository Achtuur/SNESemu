#![allow(dead_code)]

pub mod ppu;
pub mod cpu;
pub mod apu;
pub mod bit_macros;
pub mod addr_macros;
pub mod snes;

use std::sync::Mutex;


use ppu::memory::PpuMemory;

use crate::{cpu::SCpu, ppu::SPpu, apu::SApu, snes::Snes};


#[macro_export]
/// `arc_mut!(x) = Arc::new(Mutex::new(x))`
macro_rules! arc_mut {
    ($obj:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($obj))
    };
}
use lazy_static::lazy_static;
lazy_static! {
    #[derive(Debug)]
    static ref TESTMUT: Mutex::<u8> = Mutex::new(5);
}

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // let rom = include_bytes!("../resources/rom/Legend of Zelda, The - A Link to the Past.smc");
    let rom = include_bytes!("../resources/rom/Super Mario World.smc");
    

    let mut snes = Snes::new();
    
    let r = snes.insert_cartridge(rom);
    match r {
        Ok(_) => {
            let e = snes.run();
            println!("Snes run stopped: {:?}", e);
        },
        Err(e) => println!("Failed inserting cartridge: {:?}", e)
    }
}
