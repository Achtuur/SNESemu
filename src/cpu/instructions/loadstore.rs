use crate::cpu::{Cpu, processorstatusflag::ProcessorStatusFlags};

impl Cpu {
	
	/// Load Accumulator from Memory (DP Indexed Indirect,X)
	/// 
	/// `addr_low` is the address for the low byte, `addr_high` is the address for the high byte
	pub fn exe_lda(&mut self, addr_low: u32, addr_high: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				self.mem_read_long(addr_low);
				self.set_acc(self.mdr as u16);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_acc() >> 7) == 1);
			},
			false => {
				self.mem_read_long(addr_low);
				let lower = self.mdr as u16;
				// Take next address as a u32 since addresses dont wrap around
				self.mem_read_long(addr_high);
				let upper = self.mdr as u16;
				self.set_acc((upper << 8) | lower);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_acc() >> 15) == 1);
			}
		}
		self.status.set(ProcessorStatusFlags::Zero, self.get_acc() == 0);
	}
	
	/// Load Index Register X from Memory (Immediate)
	pub fn exe_ldx(&mut self, addr_low: u32, addr_high: u32) {
		match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => {
				self.mem_read_long(addr_low);
				self.set_x(self.mdr as u16);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_x() >> 7) == 1);
			},
			false => {
				self.mem_read_long(addr_low);
				let lower = self.mdr as u16;
				self.mem_read_long(addr_high);
				let upper = self.mdr as u16;
				self.set_x((upper << 8) | lower);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_x() >> 15) == 1);
			}
		}
		self.status.set(ProcessorStatusFlags::Zero, self.get_x() == 0);
	}
	
	/// Load Index Register Y from Memory (Immediate)
	pub fn exe_ldy(&mut self, addr_low: u32, addr_high: u32) {
		match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => {
				self.mem_read_long(addr_low);
				self.set_y(self.mdr as u16);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_y() >> 7) == 1);
			},
			false => {
				self.mem_read_long(addr_low);
				let lower = self.mdr as u16;
				self.mem_read_long(addr_high);
				let upper = self.mdr as u16;
				self.set_y((upper << 8) | lower);
				self.status.set(ProcessorStatusFlags::Negative, (self.get_y() >> 15) == 1);
			}
		}
		self.status.set(ProcessorStatusFlags::Zero, self.get_y() == 0);
	}
	
	/// Store Accumulator to Memory (DP Indexed Indirect,X)
	pub fn exe_sta(&mut self, addr_low: u32, addr_high: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				self.mdr = self.get_acc() as u8;
				self.mem_write_long(addr_low);
			},

			false => {
				self.mdr = self.get_acc() as u8;
				self.mem_write_long(addr_low);
				self.mdr = (self.get_acc() >> 8) as u8;
				self.mem_write_long(addr_high)
			}
		}
	}
	
	/// Store Index Register X to Memory (Direct Page)
	pub fn exe_stx(&mut self, addr_low: u32, addr_high: u32) {
		match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => {
				self.mdr = self.get_x() as u8;
				self.mem_write_long(addr_low);
			},

			false => {
				self.mdr = self.get_x() as u8;
				self.mem_write_long(addr_low);
				self.mdr = (self.get_x() >> 8) as u8;
				self.mem_write_long(addr_high)
			}
		}
	}
	
	/// Store Index Register Y to Memory (Direct Page)
	pub fn exe_sty(&mut self, addr_low: u32, addr_high: u32) {
		match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => {
				self.mdr = self.get_y() as u8;
				self.mem_write_long(addr_low);
			},

			false => {
				self.mdr = self.get_y() as u8;
				self.mem_write_long(addr_low);
				self.mdr = (self.get_y() >> 8) as u8;
				self.mem_write_long(addr_high)
			}
		}
	}
	
	/// Store Zero to Memory (Direct Page)
	pub fn exe_stz(&mut self, addr_low: u32, addr_high: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				self.mdr = 0;
				self.mem_write_long(addr_low);
			},

			false => {
				self.mdr = 0;
				self.mem_write_long(addr_low);
				self.mem_write_long(addr_high)
			}
		}
	}
	
	
}