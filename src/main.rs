mod ppu;
mod cpu;
mod apu;

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
    
    // println!("Cartridge parsed succesfully!: {0:#?}", m.lock().unwrap().cartridge_metadata);

    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();
    let mut apu = Apu::new();

    let _ = cpu.memory.insert_cartridge(rom);
    println!("{:#?}", cpu.memory.cartridge_metadata);
}
