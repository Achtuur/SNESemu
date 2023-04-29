use crate::cpu::{SCpu, processorstatusflag::ProcessorStatusFlags};

impl SCpu {

	fn branch(&mut self, target_addr: u32) {
		self.pc = target_addr as u16;
	}
	
	/// Branch if Carry Clear (Program Counter Relative)
	pub fn exe_bcc(&mut self, target_addr: u32) {
		if !self.status.contains(ProcessorStatusFlags::Carry) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Carry Set (Program Counter Relative)
	pub fn exe_bcs(&mut self, target_addr: u32) {
		if self.status.contains(ProcessorStatusFlags::Carry) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Equal (Program Counter Relative)
	pub fn exe_beq(&mut self, target_addr: u32) {
		if self.status.contains(ProcessorStatusFlags::Zero) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Minus (Program Counter Relative)
	pub fn exe_bmi(&mut self, target_addr: u32) {
		if self.status.contains(ProcessorStatusFlags::Negative) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Not Equal (Program Counter Relative)
	pub fn exe_bne(&mut self, target_addr: u32) {
		if !self.status.contains(ProcessorStatusFlags::Zero) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Plus (Program Counter Relative)
	pub fn exe_bpl(&mut self, target_addr: u32) {
		if !self.status.contains(ProcessorStatusFlags::Negative) {
			self.branch(target_addr);
		}
	}
	
	/// Branch Always (Program Counter Relative)
	pub fn exe_bra(&mut self, target_addr: u32) {
		self.branch(target_addr);
	}
	
	/// Branch Long Always (Program Counter Relative Long)
	/// 
	/// The 'long' part here means that the `target_addr` range is larger compared to non long branch instructions.
	/// For the execution here this doesnt matter, as the `target_addr` is calculated beforehand
	pub fn exe_brl(&mut self, target_addr: u32) {
		self.branch(target_addr);
	}
	
	/// Branch if Overflow Clear (Program Counter Relative)
	pub fn exe_bvc(&mut self, target_addr: u32) {
		if !self.status.contains(ProcessorStatusFlags::Overflow) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Overflow Set (Program Counter Relative)
	pub fn exe_bvs(&mut self, target_addr: u32) {
		if self.status.contains(ProcessorStatusFlags::Overflow) {
			self.branch(target_addr);
		}
	}
	
	
}