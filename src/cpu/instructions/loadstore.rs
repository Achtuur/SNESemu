use crate::cpu::{SCpu, processorstatusflag::ProcessorStatusFlags};

impl SCpu {
	
	/// Load Accumulator from Memory (DP Indexed Indirect,X)
	/// 
	/// `low_addr` is the address for the low byte, `high_addr` is the address for the high byte
	pub fn exe_lda(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				let val = self.mem_read(low_addr);
				self.set_acc(val as u16);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_acc() >> 7) == 1);
			},
			false => {
				let val = self.mem_read_long(low_addr, high_addr);
				self.set_acc(val);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_acc() >> 15) == 1);
			}
		}
		self.status.set(ProcessorStatusFlags::Zero, self.get_acc() == 0);
	}
	
	/// Load Index Register X from Memory (Immediate)
	pub fn exe_ldx(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => {
				let val = self.mem_read(low_addr);
				self.set_x(val as u16);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_x() >> 7) == 1);
			},
			false => {
				let val = self.mem_read_long(low_addr, high_addr);
				self.set_x(val);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_x() >> 15) == 1);
			}
		}
		self.status.set(ProcessorStatusFlags::Zero, self.get_x() == 0);
	}
	
	/// Load Index Register Y from Memory (Immediate)
	pub fn exe_ldy(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => {
				let val = self.mem_read(low_addr);
				self.set_y(val as u16);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_y() >> 7) == 1);
			},
			false => {
				let val = self.mem_read_long(low_addr, high_addr);
				self.set_y(val);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_y() >> 15) == 1);
			}
		}
		self.status.set(ProcessorStatusFlags::Zero, self.get_y() == 0);
	}
	
	/// Store Accumulator to Memory (DP Indexed Indirect,X)
	pub fn exe_sta(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				self.mem_write(low_addr, self.get_acc() as u8);
			},

			false => {
				self.mem_write_long(low_addr, high_addr, self.get_acc());
			}
		}
	}
	
	/// Store Index Register X to Memory (Direct Page)
	pub fn exe_stx(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				self.mem_write(low_addr, self.get_x() as u8);
			},

			false => {
				self.mem_write_long(low_addr, high_addr, self.get_x());
			}
		}
	}
	
	/// Store Index Register Y to Memory (Direct Page)
	pub fn exe_sty(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				self.mem_write(low_addr, self.get_y() as u8);
			},

			false => {
				self.mem_write_long(low_addr, high_addr, self.get_y());
			}
		}
	}
	
	/// Store Zero to Memory (Direct Page)
	pub fn exe_stz(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				self.mem_write(low_addr, 0);
			},

			false => {
				self.mem_write_long(low_addr, high_addr, 0);
			}
		}
	}
	
	
}