mod cpu;

use crate::cpu::*;

fn main() {
    let mut flag = ProcessorStatusFlags::new();
    flag.set(ProcessorStatusFlags::Overflow, true);
    flag.set(ProcessorStatusFlags::Negative, true);
    println!("flag: {0:?}", flag);
    

    flag.clear_all();
    println!("flag: {0:?}", flag);

    // let _rom = include_bytes!("roms/xxx.snes");
    // run(_rom);
}
