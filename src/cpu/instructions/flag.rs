use crate::cpu::{SCpu, statusflag::StatusFlags};

impl SCpu {
	
	/// Clear Carry (Implied)
	pub fn exe_clc(&mut self) {
		self.status.clear_flag(StatusFlags::Carry);
	}
	
	/// Clear Decimal Mode Flag (Implied)
	pub fn exe_cld(&mut self) {
		self.status.clear_flag(StatusFlags::Decimal);
	}
	
	/// Clear Interrupt Disable Flag (Implied)
	pub fn exe_cli(&mut self) {
		self.status.clear_flag(StatusFlags::IRQdisable);
	}
	
	/// Clear Overflow Flag (Implied)
	pub fn exe_clv(&mut self) {
		self.status.clear_flag(StatusFlags::Overflow);
	}
	
	/// Reset Processor Status Bits (Immediate)
	pub fn exe_rep(&mut self, data: u8) {
		self.status.clear_bits(data as u8);
	}
	
	/// Set Carry Flag (Implied)
	pub fn exe_sec(&mut self) {
		self.status.set_flag(StatusFlags::Carry);
	}
	
	/// Set Decimal Flag (Implied)
	pub fn exe_sed(&mut self) {
		self.status.set_flag(StatusFlags::Decimal);
	}
	
	/// Set Interrupt Disable Flag (Implied)
	pub fn exe_sei(&mut self) {
		self.status.set_flag(StatusFlags::IRQdisable);
	}
	
	/// Set Processor Status Bits (Immediate)
	pub fn exe_sep(&mut self, data: u8) {
		self.status.set_bits(data)
	}
	
	/// Exchange Carry and Emulation Flags (Implied)
	pub fn exe_xce(&mut self, data: u16) {
		let carry = self.status.contains(StatusFlags::Carry);
		let emulation = self.status.contains(StatusFlags::Emulation);
		self.status.set(StatusFlags::Carry, emulation);
		self.status.set(StatusFlags::Emulation, carry);
	}
	
	
}