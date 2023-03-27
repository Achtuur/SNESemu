use crate::cpu::Cpu;

impl Cpu {
    
/// Push Effective Absolute Address (Stack (Absolute))
pub fn exe_pea(&mut self, data: u32) {
	todo!()
}

/// Push Effective Indirect Address (Stack (DP Indirect))
pub fn exe_pei(&mut self, data: u32) {
	todo!()
}

/// Push Effective PC Relative Indirect Address (Stack (PC Relative Long))
pub fn exe_per(&mut self, data: u32) {
	todo!()
}

/// Push Accumulator (Stack (Push))
pub fn exe_pha(&mut self, data: u32) {
	todo!()
}

/// Push Data Bank Register (Stack (Push))
pub fn exe_phb(&mut self, data: u32) {
	todo!()
}

/// Push Direct Page Register (Stack (Push))
pub fn exe_phd(&mut self, data: u32) {
	todo!()
}

/// Push Program Bank Register (Stack (Push))
pub fn exe_phk(&mut self, data: u32) {
	todo!()
}

/// Push Processor Status Register (Stack (Push))
pub fn exe_php(&mut self, data: u32) {
	todo!()
}

/// Push Index Register X (Stack (Push))
pub fn exe_phx(&mut self, data: u32) {
	todo!()
}

/// Push Index Register Y (Stack (Push))
pub fn exe_phy(&mut self, data: u32) {
	todo!()
}

/// Pull Accumulator (Stack (Pull))
pub fn exe_pla(&mut self, data: u32) {
	todo!()
}

/// Pull Data Bank Register (Stack (Pull))
pub fn exe_plb(&mut self, data: u32) {
	todo!()
}

/// Pull Direct Page Register (Stack (Pull))
pub fn exe_pld(&mut self, data: u32) {
	todo!()
}

/// Pull Processor Status Register (Stack (Pull))
pub fn exe_plp(&mut self, data: u32) {
	todo!()
}

/// Pull Index Register X (Stack (Pull))
pub fn exe_plx(&mut self, data: u32) {
	todo!()
}

/// Pull Index Register Y (Stack (Pull))
pub fn exe_ply(&mut self, data: u32) {
	todo!()
}


}