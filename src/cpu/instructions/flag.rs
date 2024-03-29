use crate::cpu::{Cpu, processorstatusflag::ProcessorStatusFlags};

impl Cpu {
	
	/// Clear Carry (Implied)
	pub fn exe_clc(&mut self) {
		self.status.clear_flag(ProcessorStatusFlags::Carry);
	}
	
	/// Clear Decimal Mode Flag (Implied)
	pub fn exe_cld(&mut self) {
		self.status.clear_flag(ProcessorStatusFlags::Decimal);
	}
	
	/// Clear Interrupt Disable Flag (Implied)
	pub fn exe_cli(&mut self) {
		self.status.clear_flag(ProcessorStatusFlags::IRQdisable);
	}
	
	/// Clear Overflow Flag (Implied)
	pub fn exe_clv(&mut self) {
		self.status.clear_flag(ProcessorStatusFlags::Overflow);
	}
	
	/// Reset Processor Status Bits (Immediate)
	pub fn exe_rep(&mut self, data: u8) {
		self.status.clear_bits(data as u8);
	}
	
	/// Set Carry Flag (Implied)
	pub fn exe_sec(&mut self) {
		self.status.set_flag(ProcessorStatusFlags::Carry);
	}
	
	/// Set Decimal Flag (Implied)
	pub fn exe_sed(&mut self) {
		self.status.set_flag(ProcessorStatusFlags::Decimal);
	}
	
	/// Set Interrupt Disable Flag (Implied)
	pub fn exe_sei(&mut self) {
		self.status.set_flag(ProcessorStatusFlags::IRQdisable);
	}
	
	/// Set Processor Status Bits (Immediate)
	pub fn exe_sep(&mut self, data: u8) {
		self.status.set_bits(data)
	}
	
	/// Exchange Carry and Emulation Flags (Implied)
	pub fn exe_xce(&mut self, data: u16) {
		let carry = self.status.contains(ProcessorStatusFlags::Carry);
		let emulation = self.status.contains(ProcessorStatusFlags::Emulation);
		self.status.set(ProcessorStatusFlags::Carry, emulation);
		self.status.set(ProcessorStatusFlags::Emulation, carry);
	}
	
	
}