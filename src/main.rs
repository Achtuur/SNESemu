
mod memory;
mod ppu;
mod cpu;

use memory::Memory;

use crate::cpu::processorstatusflag::ProcessorStatusFlags;


fn main() {
    let mut flag = ProcessorStatusFlags::new();
    flag.set(ProcessorStatusFlags::Overflow, true);
    flag.set(ProcessorStatusFlags::Negative, true);
    println!("flag: {0:?}", flag);
    

    flag.clear_all();
    println!("flag: {0:?}", flag);
    // let rom = include_bytes!("../resources/rom/Legend of Zelda, The - A Link to the Past.smc");
    let rom = include_bytes!("../resources/rom/Super Mario World.smc");
    let mut m: Memory = Memory::new();

    m.insert_cartridge(rom);

    println!("m.cartridge_metadata: {0:#?}", m.cartridge_metadata);
}
