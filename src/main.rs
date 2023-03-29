
mod memory;
mod ppu;

use memory::Memory;


fn main() {

    // let rom = include_bytes!("../resources/rom/Legend of Zelda, The - A Link to the Past.smc");
    let rom = include_bytes!("../resources/rom/Super Mario World.smc");
    let mut m: Memory = Memory::new();

    m.insert_cartridge(rom);

    println!("m.cartridge_metadata: {0:#?}", m.cartridge_metadata);
}
