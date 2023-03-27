use crate::cpu::Cpu;

impl Cpu {
    
/// Transfer Accumulator to Index Register X (Implied)
pub fn exe_tax(&mut self, data: u32) {
	todo!()
}

/// Transfer Accumulator to Index Register Y (Implied)
pub fn exe_tay(&mut self, data: u32) {
	todo!()
}

/// Transfer 16-bit Accumulator to Direct Page Register (Implied)
pub fn exe_tcd(&mut self, data: u32) {
	todo!()
}

/// Transfer 16-bit Accumulator to Stack Pointer (Implied)
pub fn exe_tcs(&mut self, data: u32) {
	todo!()
}

/// Transfer Direct Page Register to 16-bit Accumulator (Implied)
pub fn exe_tdc(&mut self, data: u32) {
	todo!()
}

/// Transfer Stack Pointer to 16-bit Accumulator (Implied)
pub fn exe_tsc(&mut self, data: u32) {
	todo!()
}

/// Transfer Stack Pointer to Index Register X (Implied)
pub fn exe_tsx(&mut self, data: u32) {
	todo!()
}

/// Transfer Index Register X to Accumulator (Implied)
pub fn exe_txa(&mut self, data: u32) {
	todo!()
}

/// Transfer Index Register X to Stack Pointer (Implied)
pub fn exe_txs(&mut self, data: u32) {
	todo!()
}

/// Transfer Index Register X to Index Register Y (Implied)
pub fn exe_txy(&mut self, data: u32) {
	todo!()
}

/// Transfer Index Register Y to Accumulator (Implied)
pub fn exe_tya(&mut self, data: u32) {
	todo!()
}

/// Transfer Index Register Y to Index Register X (Implied)
pub fn exe_tyx(&mut self, data: u32) {
	todo!()
}


}