mod memory;
mod ppu;
mod cpu;
mod apu;

use memory::Memory;

use cpu::processorstatusflag::ProcessorStatusFlags;

use crate::{cpu::Cpu, ppu::Ppu, apu::Apu};


#[macro_export]
/// `arc_mut!(x) = Arc::new(Mutex::new(x))`
macro_rules! arc_mut {
    ($obj:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($obj))
    };
}


fn main() {
    let mut flag = ProcessorStatusFlags::new();
    flag.set(ProcessorStatusFlags::Overflow, true);
    flag.set(ProcessorStatusFlags::Negative, true);
    println!("flag: {0:?}", flag);
    
    // let rom = include_bytes!("../resources/rom/Legend of Zelda, The - A Link to the Past.smc");
    let rom = include_bytes!("../resources/rom/Super Mario World.smc");
    let m = arc_mut!(Memory::new());
    
    let _ = m.lock().unwrap().insert_cartridge(rom);

    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();
    let mut apu = Apu::new();

    cpu.set_memory(m.clone());
    ppu.set_memory(m.clone());
    apu.set_memory(m.clone());
    

}
