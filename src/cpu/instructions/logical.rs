use crate::cpu::{Cpu, processorstatusflag::ProcessorStatusFlags};

impl Cpu {
	
	/// AND Accumulator with Memory (DP Indexed Indirect,X)
	pub fn exe_and(&mut self, data: u16) {
		self.set_acc(self.get_acc() & data);
		self.set_acc_nz_flag();
	}
	
	/// Test Bits (AND data with accumulator and set some status flags)
	/// 
	/// For some godforsaken reason the BIT instruction with immediate addressing mode only affects the Z flag.
	pub fn exe_bit_imm(&mut self, data: u16) {
		self.status.set(ProcessorStatusFlags::Zero, (self.get_acc() & data) == 0);
	}

	/// Test Bits (Direct Page)
	pub fn exe_bit(&mut self, data: u16) {
		// Get amount of bits to shift right for negative and overflow flag based on bit mode
		let (n, v) = match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => (7, 6),
			false => (15, 14),
		};
		// These flags are set depending on data, NOT the result of the and operation
		self.status.set(ProcessorStatusFlags::Overflow, (data >> v) == 1);
		self.status.set(ProcessorStatusFlags::Negative, (data >> n) == 1);
		
		let and = self.get_acc() & data;
		self.status.set(ProcessorStatusFlags::Zero, and == 0);
	}
	
	/// Compare Accumulator with Memory (DP Indexed Indirect,X)
	pub fn exe_cmp(&mut self, data: u16) {
		let cmp = self.get_acc().wrapping_sub(data);
		self.status.set(ProcessorStatusFlags::Carry, cmp <= 0);
		self.status.set(ProcessorStatusFlags::Zero, cmp == 0);
		self.status.set(ProcessorStatusFlags::Negative, (cmp as i8) < 0);
	}
	
	/// Compare Index Register X with Memory (Immediate)
	pub fn exe_cpx(&mut self, data: u16) {
		let cmp = self.get_x().wrapping_sub(data);
		self.status.set(ProcessorStatusFlags::Carry, cmp <= 0);
		self.status.set(ProcessorStatusFlags::Zero, cmp == 0);
		self.status.set(ProcessorStatusFlags::Negative, (cmp as i8) < 0);
	}
	
	/// Compare Index Register Y with Memory (Immediate)
	pub fn exe_cpy(&mut self, data: u16) {
		let cmp = self.get_y().wrapping_sub(data);
		self.status.set(ProcessorStatusFlags::Carry, cmp <= 0);
		self.status.set(ProcessorStatusFlags::Zero, cmp == 0);
		self.status.set(ProcessorStatusFlags::Negative, (cmp as i8) < 0);
	}
	
	/// Exclusive-OR Accumulator with Memory (DP Indexed Indirect,X)
	pub fn exe_eor(&mut self, data: u16) {
		self.set_acc(self.get_acc() ^ data);
		self.set_acc_nz_flag();
	}
	
	/// OR Accumulator with Memory (DP Indexed Indirect,X)
	pub fn exe_ora(&mut self, data: u16) {
		self.set_acc(self.get_acc() | data);
		self.set_acc_nz_flag();
	}
	
	/// Rotate Memory or Accumulator Left (Direct Page)
	pub fn exe_rol(&mut self, addr: u16) {
		let mut val = self.mem_read(addr);
		let hb = val >> 7;
		let c = self.carry() as u8;
		val = (val << 1) | c;
		// set carry flag to old high bit
		self.status.set(ProcessorStatusFlags::Carry, hb == 1);
		self.status.set(ProcessorStatusFlags::Zero, val == 0);
		self.status.set(ProcessorStatusFlags::Negative, (val as i8) < 0);
	}
	
	/// Rotate Memory or Accumulator Right (Direct Page)
	pub fn exe_ror(&mut self, addr: u16) {
		let mut val = self.mem_read(addr);
		let lb = val & 0x01;
		let c = self.carry() as u8;
		val = (val >> 1) | (c << 7);
		// set carry flag to old low bit
		self.status.set(ProcessorStatusFlags::Carry, lb == 1);
		self.status.set(ProcessorStatusFlags::Zero, val == 0);
		self.status.set(ProcessorStatusFlags::Negative, (val as i8) < 0);
	}

	/// Test and Reset Memory Bits Against Accumulator (Direct Page)
	pub fn exe_trb(&mut self, addr: u16) {
		let mut val = self.mem_read(addr);
		let and = self.get_acc() as u8 & val;
		self.status.set(ProcessorStatusFlags::Zero, and == 0);

		// set bits to 0 in val that are 1 in and
		val &= !and;
		self.mem_write(addr, val);
	}
	
	/// Test and Set Memory Bits Against Accumulator (Direct Page)
	pub fn exe_tsb(&mut self, addr: u16) {
		let mut val = self.mem_read(addr);
		let and = self.get_acc() as u8 & val;
		self.status.set(ProcessorStatusFlags::Zero, and == 0);

		// set bits to 1 in val that are 1 in and
		val |= and;
		self.mem_write(addr, val);
	}
	
	
}