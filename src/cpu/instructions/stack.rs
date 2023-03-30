use crate::cpu::{Cpu, processorstatusflag::ProcessorStatusFlags};

impl Cpu {
	
	/// Push Effective Absolute Address (Stack (Absolute))
	pub fn exe_pea(&mut self, data: u16) {
		self.push_long_stack(data);
	}
	
	/// Push Effective Indirect Address (Stack (DP Indirect))
	/// 
	/// `data` here contains the address that value inside the dp indirect address
	/// 
	/// meaning memory should be read outside this function
	pub fn exe_pei(&mut self, data: u16) {
		self.push_long_stack(data);
	}
	
	/// Push Effective PC Relative Indirect Address (Stack (PC Relative Long))
	pub fn exe_per(&mut self, data: u16) {
		let data = data.wrapping_add(self.pc);
		self.push_long_stack(data);
	}
	
	/// Push Accumulator (Stack (Push))
	pub fn exe_pha(&mut self, data: u16) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => self.push_byte_stack(self.acc as u8),
			false => self.push_long_stack(self.acc),
		}
	}
	
	/// Push Data Bank Register (Stack (Push))
	pub fn exe_phb(&mut self, data: u16) {
		self.push_byte_stack(self.dbr);
	}
	
	/// Push Direct Page Register (Stack (Push))
	pub fn exe_phd(&mut self, data: u16) {
		self.push_long_stack(self.dp);
	}
	
	/// Push Program Bank Register (Stack (Push))
	pub fn exe_phk(&mut self, data: u16) {
		self.push_byte_stack(self.pbr);
	}
	
	/// Push Processor Status Register (Stack (Push))
	pub fn exe_php(&mut self, data: u16) {
		self.push_byte_stack(self.status.bits())
	}
	
	/// Push Index Register X (Stack (Push))
	pub fn exe_phx(&mut self, data: u16) {
		todo!()
	}
	
	/// Push Index Register Y (Stack (Push))
	pub fn exe_phy(&mut self, data: u16) {
		todo!()
	}
	
	/// Pull Accumulator (Stack (Pull))
	pub fn exe_pla(&mut self, data: u16) {
		todo!()
	}
	
	/// Pull Data Bank Register (Stack (Pull))
	pub fn exe_plb(&mut self, data: u16) {
		todo!()
	}
	
	/// Pull Direct Page Register (Stack (Pull))
	pub fn exe_pld(&mut self, data: u16) {
		todo!()
	}
	
	/// Pull Processor Status Register (Stack (Pull))
	pub fn exe_plp(&mut self, data: u16) {
		todo!()
	}
	
	/// Pull Index Register X (Stack (Pull))
	pub fn exe_plx(&mut self, data: u16) {
		todo!()
	}
	
	/// Pull Index Register Y (Stack (Pull))
	pub fn exe_ply(&mut self, data: u16) {
		todo!()
	}
	
	
}