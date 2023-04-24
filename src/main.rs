#![allow(dead_code)]

mod ppu;
mod cpu;
mod apu;
pub mod bit_macros;
pub mod addr_macros;

use apu::memory::ApuMemory;

use ppu::memory::PpuMemory;

use crate::{cpu::Cpu, ppu::Ppu, apu::Apu};


#[macro_export]
/// `arc_mut!(x) = Arc::new(Mutex::new(x))`
macro_rules! arc_mut {
    ($obj:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($obj))
    };
}

fn main() {
    // let rom = include_bytes!("../resources/rom/Legend of Zelda, The - A Link to the Past.smc");
    let rom = include_bytes!("../resources/rom/Super Mario World.smc");

    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();
    let mut apu = Apu::new();

    let ppumem = arc_mut!(PpuMemory::new());
    cpu.memory.set_ppumemory_ref(ppumem.clone());
    ppu.set_ppumemory_ref(ppumem);

    let apumem = arc_mut!(ApuMemory::new());
    cpu.memory.set_apumemory_ref(apumem.clone());
    apu.set_apumemory_ref(apumem);

    let _ = cpu.memory.insert_cartridge(rom);
    println!("{:#?}", cpu.memory.cartridge_metadata);

    ppu.run();

}
