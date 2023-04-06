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
	/// 
	/// `data` here should be a label, however assemblers should calculate it while assembling, 
	/// meaning the argument in the ROM is most likely the correct relative address
	pub fn exe_per(&mut self, data: u16) {
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
		self.push_byte_stack(self.status.bits() as u8)
	}
	
	/// Push Index Register X (Stack (Push))
	pub fn exe_phx(&mut self, data: u16) {
		match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => self.push_byte_stack(self.x as u8),
			false => self.push_long_stack(self.x),
		}
	}
	
	/// Push Index Register Y (Stack (Push))
	pub fn exe_phy(&mut self, data: u16) {
		match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => self.push_byte_stack(self.y as u8),
			false => self.push_long_stack(self.y),
		}
	}
	
	/// Pull Accumulator (Stack (Pull))
	pub fn exe_pla(&mut self, data: u16) {
		self.acc = match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => self.pull_byte_stack() as u16,
			false => self.pull_long_stack(), 
		};
		
		// Check flags
		if self.acc == 0 {
			self.status.set_flag(ProcessorStatusFlags::Zero)
		}
		
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => self.status.set(ProcessorStatusFlags::Accumulator8bit, (self.acc as i8) < 0),
			false => self.status.set(ProcessorStatusFlags::Accumulator8bit, (self.acc as i16) < 1),
		}
		
	}
	
	/// Pull Data Bank Register (Stack (Pull))
	pub fn exe_plb(&mut self, data: u16) {
		self.dbr = self.pull_byte_stack();
		
		// Check flags
		if self.dbr == 0 {
			self.status.set_flag(ProcessorStatusFlags::Zero);
		}
		
		if (self.dbr as i8) < 0 {
			self.status.set_flag(ProcessorStatusFlags::Negative);
		}
	}
	
	/// Pull Direct Page Register (Stack (Pull))
	pub fn exe_pld(&mut self, data: u16) {
		self.dp = self.pull_long_stack();
		
		// Check flags
		if self.dp == 0 {
			self.status.set_flag(ProcessorStatusFlags::Zero);
		}
		
		if (self.dp as i16) < 0 {
			self.status.set_flag(ProcessorStatusFlags::Negative);
		}
	}
	
	/// Pull Processor Status Register (Stack (Pull))
	pub fn exe_plp(&mut self, data: u16) {
		let flag_bits = self.pull_byte_stack();
		self.status.set_bits(flag_bits);
		
		if flag_bits == 0 {
			self.status.set_flag(ProcessorStatusFlags::Zero);
		}

		if (flag_bits as i8) < 0 {
			self.status.set_flag(ProcessorStatusFlags::Negative);
		}
	}
	
	/// Pull Index Register X (Stack (Pull))
	pub fn exe_plx(&mut self, data: u16) {
		self.x = match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => self.pull_byte_stack() as u16,
			false => self.pull_long_stack(), 
		};
		
		// Check flags
		if self.x == 0 {
			self.status.set_flag(ProcessorStatusFlags::Zero)
		}
		
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => self.status.set(ProcessorStatusFlags::Accumulator8bit, (self.x as i8) < 0),
			false => self.status.set(ProcessorStatusFlags::Accumulator8bit, (self.x as i16) < 1),
		}
	}
	
	/// Pull Index Register Y (Stack (Pull))
	pub fn exe_ply(&mut self, data: u16) {
		self.y = match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => self.pull_byte_stack() as u16,
			false => self.pull_long_stack(), 
		};
		
		// Check flags
		if self.y == 0 {
			self.status.set_flag(ProcessorStatusFlags::Zero)
		}
		
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => self.status.set(ProcessorStatusFlags::Accumulator8bit, (self.y as i8) < 0),
			false => self.status.set(ProcessorStatusFlags::Accumulator8bit, (self.y as i16) < 1),
		}
	}
	
	
}