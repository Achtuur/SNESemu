use crate::cpu::Cpu;

impl Cpu {
    
/// No Operation (Implied)
pub fn exe_nop(&mut self, data: u32) {
	todo!()
}

/// Stop Processor (Implied)
pub fn exe_stp(&mut self, data: u32) {
	todo!()
}

/// <em>Reserved for Future Expansion</em> ()
pub fn exe_wdm(&mut self, data: u32) {
	todo!()
}

/// Exchange B and A 8-bit Accumulators (Implied)
pub fn exe_xba(&mut self, data: u32) {
	todo!()
}

/// Exchange Carry and Emulation Flags (Implied)
pub fn exe_xce(&mut self, data: u32) {
	todo!()
}


}