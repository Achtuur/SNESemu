use crate::{apu::{SApu, statusword::StatusWord}, bit_set};



impl SApu {

	fn branch(&mut self, target_addr: u16) {
		self.pc = target_addr;
		// Correct pc for instruction length when branch is taken
		self.pc = self.pc.wrapping_sub(2);
        self.cycle_time += 2;
	}
	
	/// Branch if Carry Clear (Program Counter Relative)
	pub fn exe_bcc(&mut self, target_addr: u16) {
		if !self.status.contains(StatusWord::Carry) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Carry Set (Program Counter Relative)
	pub fn exe_bcs(&mut self, target_addr: u16) {
		if self.status.contains(StatusWord::Carry) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Equal (Program Counter Relative)
	pub fn exe_beq(&mut self, target_addr: u16) {
		if self.status.contains(StatusWord::Zero) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Minus (Program Counter Relative)
	pub fn exe_bmi(&mut self, target_addr: u16) {
		if self.status.contains(StatusWord::Negative) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Not Equal (Program Counter Relative)
	pub fn exe_bne(&mut self, target_addr: u16) {
		if !self.status.contains(StatusWord::Zero) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Plus (Program Counter Relative)
	pub fn exe_bpl(&mut self, target_addr: u16) {
		if !self.status.contains(StatusWord::Negative) {
			self.branch(target_addr);
		}
	}
	
	/// Branch Always (Program Counter Relative)
	pub fn exe_bra(&mut self, target_addr: u16) {
		self.branch(target_addr);
	}
	
	/// Branch if Overflow Clear (Program Counter Relative)
	pub fn exe_bvc(&mut self, target_addr: u16) {
		if !self.status.contains(StatusWord::Overflow) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Overflow Set (Program Counter Relative)
	pub fn exe_bvs(&mut self, target_addr: u16) {
		if self.status.contains(StatusWord::Overflow) {
			self.branch(target_addr);
		}
	}

    /// Branch if memory bit set
    pub fn exe_bbs(&mut self, data: u8, bit: u8, target_addr: u16) {
        if bit_set!(data, bit) {
            self.branch(target_addr);
        }
    }

    /// Branch if memory bit cleared
    pub fn exe_bbc(&mut self, data: u8, bit: u8, target_addr: u16) {
        if !bit_set!(data, bit) {
            self.branch(target_addr);
        }
    }

    /// Branch if memory bit set
    pub fn exe_cbne(&mut self, data_addr: u16, target_addr: u16) {
        let data = self.mem_read(data_addr);
        if self.acc != data {
            self.branch(target_addr);
        }
    }
    
    /// Subtract 1 from data inside `data_addr` and branch if not zero
    pub fn exe_dbnz(&mut self, data_addr: u16, target_addr: u16) {
        let data = self.mem_read(data_addr).wrapping_add(1);
        self.mem_write(data_addr, data);
        if data != 0 {
            self.branch(target_addr);
        }
    }

    /// Subtract 1 from Y register and subtract if not zero
    pub fn exe_dbnz_y(&mut self, target_addr: u16) {
        self.y = self.y.wrapping_sub(1);
        if self.y != 0 {
            self.branch(target_addr);
        }
    }
}