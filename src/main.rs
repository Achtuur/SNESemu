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

#[macro_export]
/// Creates a word from two bytes, first pass the high byte, then the low byte
macro_rules! to_word {
    ($hh: expr, $ll: expr) => {
        (($hh as u16) << 8) | ($ll as u16)
    };
}

#[macro_export]
/// Get `nth` bit
macro_rules! nth_bit {
    ($num: expr, $n: literal) => {
        ($num >> $n) & 1
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
