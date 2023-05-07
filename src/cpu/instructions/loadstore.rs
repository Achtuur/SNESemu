use crate::{cpu::{SCpu, processorstatusflag::ProcessorStatusFlags}, bit_set};

impl SCpu {
	
	/// Load Accumulator from Memory (DP Indexed Indirect,X)
	/// 
	/// `low_addr` is the address for the low byte, `high_addr` is the address for the high byte
	pub fn exe_lda(&mut self, data: u16) {
		self.set_acc(data);
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => self.status.set(ProcessorStatusFlags::Negative, bit_set!(self.get_acc(), 7)),
			false => self.status.set(ProcessorStatusFlags::Negative, bit_set!(self.get_acc(), 15)),
		}
		self.status.set(ProcessorStatusFlags::Zero, self.get_acc() == 0);
	}

	
	/// Load Index Register X from Memory (Immediate)
	pub fn exe_ldx(&mut self, data: u16) {
		self.set_x(data);
		match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => self.status.set(ProcessorStatusFlags::Negative, bit_set!(self.get_x(), 7)),
			false => self.status.set(ProcessorStatusFlags::Negative, bit_set!(self.get_x(), 15)),
		}
		self.status.set(ProcessorStatusFlags::Zero, self.get_x() == 0);
	}
	
	/// Load Index Register Y from Memory (Immediate)
	pub fn exe_ldy(&mut self, data: u16) {
		self.set_y(data);
		match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => self.status.set(ProcessorStatusFlags::Negative, bit_set!(self.get_y(), 7)),
			false => self.status.set(ProcessorStatusFlags::Negative, bit_set!(self.get_y(), 15)),
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

#[cfg(test)]
mod tests {
    use crate::{cpu::{processorstatusflag::ProcessorStatusFlags, SCpu}, arc_mut, apu::memory::ApuMemory, ppu::memory::PpuMemory};

	fn get_test_cpu() -> SCpu {
		let mut cpu = SCpu::new();
		let ppumem = arc_mut!(PpuMemory::new());
        cpu.memory.set_ppumemory_ref(ppumem.clone());
		let apumem = arc_mut!(ApuMemory::new());
        cpu.memory.set_apumemory_ref(apumem.clone());
		cpu
	}

	#[test]
	fn test_lda() {
		let mut cpu = get_test_cpu();
		cpu.status.set_flag(ProcessorStatusFlags::Accumulator8bit);

	}	
    
}
