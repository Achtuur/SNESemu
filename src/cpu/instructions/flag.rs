use crate::cpu::Cpu;

impl Cpu {
	
	/// Clear Carry (Implied)
	pub fn exe_clc(&mut self, data: u16) {
		todo!()
	}
	
	/// Clear Decimal Mode Flag (Implied)
	pub fn exe_cld(&mut self, data: u16) {
		todo!()
	}
	
	/// Clear Interrupt Disable Flag (Implied)
	pub fn exe_cli(&mut self, data: u16) {
		todo!()
	}
	
	/// Clear Overflow Flag (Implied)
	pub fn exe_clv(&mut self, data: u16) {
		todo!()
	}
	
	/// Reset Processor Status Bits (Immediate)
	pub fn exe_rep(&mut self, data: u16) {
		todo!()
	}
	
	/// Set Carry Flag (Implied)
	pub fn exe_sec(&mut self, data: u16) {
		todo!()
	}
	
	/// Set Decimal Flag (Implied)
	pub fn exe_sed(&mut self, data: u16) {
		todo!()
	}
	
	/// Set Interrupt Disable Flag (Implied)
	pub fn exe_sei(&mut self, data: u16) {
		todo!()
	}
	
	/// Set Processor Status Bits (Immediate)
	pub fn exe_sep(&mut self, data: u16) {
		todo!()
	}
	
	/// Test and Reset Memory Bits Against Accumulator (Direct Page)
	pub fn exe_trb(&mut self, data: u16) {
		todo!()
	}
	
	/// Test and Set Memory Bits Against Accumulator (Direct Page)
	pub fn exe_tsb(&mut self, data: u16) {
		todo!()
	}
	
	
}