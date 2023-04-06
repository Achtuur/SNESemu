use crate::cpu::{Cpu, processorstatusflag::ProcessorStatusFlags};

impl Cpu {
	
	/// Add With Carry (DP Indexed Indirect,X)
	pub fn exe_adc(&mut self, data: u16) {
		let sum = match self.status.contains(ProcessorStatusFlags::Decimal) {
			// BCD addition
			true => {
				self.exe_adc_bcd(data)
			},
			// Binary addition
			false => {
				self.exe_adc_bin(data)
			}
		};
		self.set_acc(sum);
		self.set_acc_nz_flag();
	}

	/// Add BCD numbers
	fn exe_adc_bcd(&mut self, data: u16) -> u16 {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			//8 bit addition
			true => {
				let data = data as u8;
				let acc = self.acc as u8;

				// source: https://forums.nesdev.org/viewtopic.php?t=3070
				let mut lower_nybble = (acc & 0x0F) + (data & 0x0F) + self.carry() as u8;
				let mut upper_nybble = (acc & 0xF0) + (data & 0xF0);

				if lower_nybble > 9 {
					lower_nybble = (lower_nybble + 6) & 0xF;
					upper_nybble += 1;
				}

				if upper_nybble > 9 {
					upper_nybble = (upper_nybble + 6) % 0xF;
					self.status.set_flag(ProcessorStatusFlags::Carry);
				} else {
					self.status.clear_flag(ProcessorStatusFlags::Carry);
				}

				let sum_bcd = (upper_nybble << 4) + lower_nybble;

				// Overflow is unreliable with bcd, so just always disable
				self.status.clear_flag(ProcessorStatusFlags::Overflow);

				sum_bcd as u16
			},
			// 16 bit addition
			false => {
				// This is same implementation as 8 bits, except with a for loop
				let mut nybbles = [0_u16; 4];
				
				let mut lower_nybble = (self.acc & 0x0F) + (data & 0x0F) + self.carry();
				let mut upper_nybble = 0;

				for i in 0..3 {
					let nyb_sel = 0xF0 << 4*i;
					upper_nybble = (self.acc & nyb_sel) + (data & nyb_sel);
					if lower_nybble > 9 {
						lower_nybble += 6;
						lower_nybble &= 0xF;
						upper_nybble += 1;
					}
					nybbles[i] = lower_nybble;
					lower_nybble = upper_nybble;
				}
				nybbles[3] = upper_nybble;

				if nybbles[3] > 9 {
					nybbles[3] += 6;
					nybbles[3] &= 0xF;
					self.status.set_flag(ProcessorStatusFlags::Carry);
				} else {
					self.status.clear_flag(ProcessorStatusFlags::Carry);
				}

				
				let sum_bcd = (nybbles[3] << 12) | (nybbles[2] << 8) | (nybbles[1] << 4) | nybbles[0];

				// Overflow is unreliable with bcd, so just always disable
				self.status.clear_flag(ProcessorStatusFlags::Overflow);

				sum_bcd as u16
			}
		}
	}

	/// Add binary numbers
	fn exe_adc_bin(&mut self, data: u16) -> u16 {
		let sum = self.get_acc().wrapping_add(data).wrapping_add(self.carry());
		
		// If acc > sum, then sum wraps around u8/u16 limit, meaning carry should be propogated
		self.status.set(ProcessorStatusFlags::Carry, self.get_acc() > sum);

		// amount of bits to shift left to find if number is negative
		let b = match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			//8 bit
			true => 7,
			//16 bit
			false => 15,
		};

		self.status.set(
			ProcessorStatusFlags::Overflow, 
			(self.get_acc() >> b) == 1 && (data >> b) == 1 && (sum >> b) == 0 || 
				(self.get_acc() >> b) == 0 && (data >> b) == 0 && (sum >> b) == 1
		);

		sum
	}
	
	/// Arithmetic Shift Left (Direct Page)
	pub fn exe_asl(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				let mut val = self.mem_read(low_addr);
				self.status.set(ProcessorStatusFlags::Carry, (val >> 7) == 1);
				val >>= 1;
				self.status.set(ProcessorStatusFlags::Zero, val == 0);
				self.status.set(ProcessorStatusFlags::Negative, (val >> 7) == 1);
				self.mem_write(low_addr, val);
			},
			false => {
				let mut val = self.mem_read_long(low_addr, high_addr);
				self.status.set(ProcessorStatusFlags::Carry, (val >> 15) == 1);
				val >>= 1;
				self.status.set(ProcessorStatusFlags::Zero, val == 0);
				self.status.set(ProcessorStatusFlags::Negative, (val >> 15) == 1);
				self.mem_write_long(low_addr, high_addr, val);
			}
		}
	}

	/// Arithmetic Shift Left (Accumulator)
	/// 
	/// the asla name was made up by me to differentiate it from asl
	pub fn exe_asla(&mut self) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => self.status.set(ProcessorStatusFlags::Carry, (self.acc >> 7) == 1),
			false => self.status.set(ProcessorStatusFlags::Carry, (self.acc >> 15) == 1)
		}
		self.set_acc(self.get_acc() << 1);
		self.set_acc_nz_flag();
	}
	
	/// Decrement (Accumulator)
	pub fn exe_dea(&mut self) {
		self.set_acc(self.get_acc().wrapping_sub(1));
		self.set_acc_nz_flag();
	}
	
	/// Decrement (Direct Page)
	pub fn exe_dec(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				let val = self.mem_read(low_addr).wrapping_sub(1);
				self.status.set(ProcessorStatusFlags::Zero, val == 0);
				self.status.set(ProcessorStatusFlags::Negative, (val >> 7) == 1);
				self.mem_write(low_addr, val);
			},
			false => {
				let val = self.mem_read_long(low_addr, high_addr).wrapping_sub(1);
				self.status.set(ProcessorStatusFlags::Zero, val == 0);
				self.status.set(ProcessorStatusFlags::Negative, (val >> 15) == 1);
				self.mem_write_long(low_addr, high_addr, val);
			}
		}
	}
	
	/// Decrement Index Register X (Implied)
	pub fn exe_dex(&mut self) {
		self.set_x(self.get_x().wrapping_sub(1));
		self.set_x_nz_flag();
	}
	
	/// Decrement Index Register Y (Implied)
	pub fn exe_dey(&mut self) {
		self.set_y(self.get_y().wrapping_sub(1));
		self.set_y_nz_flag();
	}
	
	/// Increment (Accumulator)
	pub fn exe_ina(&mut self) {
		self.set_acc(self.get_acc().wrapping_add(1));
		self.set_acc_nz_flag()
	}
	
	/// Increment (Direct Page)
	pub fn exe_inc(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				let val = self.mem_read(low_addr).wrapping_sub(1);
				self.status.set(ProcessorStatusFlags::Zero, val == 0);
				self.status.set(ProcessorStatusFlags::Negative, (val >> 7) == 1);
				self.mem_write(low_addr, val);
			},
			false => {
				let val = self.mem_read_long(low_addr, high_addr).wrapping_sub(1);
				self.status.set(ProcessorStatusFlags::Zero, val == 0);
				self.status.set(ProcessorStatusFlags::Negative, (val >> 15) == 1);
				self.mem_write_long(low_addr, high_addr, val);
			}
		}
	}
	
	/// Increment Index Register X (Implied)
	pub fn exe_inx(&mut self) {
		self.set_x(self.get_x().wrapping_add(1));
		self.set_x_nz_flag();
	}
	
	/// Increment Index Register Y (Implied)
	pub fn exe_iny(&mut self) {
		self.set_y(self.get_y().wrapping_add(1));
		self.set_y_nz_flag();
	}

	/// Logical Shift Memory or Accumulator Right (Direct Page)
	pub fn exe_lsr(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				let mut val = self.mem_read(low_addr);
				self.status.set(ProcessorStatusFlags::Carry, (val & 0x01) == 1);
				val <<= 1;
				self.status.set(ProcessorStatusFlags::Zero, val == 0);
				self.status.set(ProcessorStatusFlags::Negative, (val >> 7) == 1);
				self.mem_write(low_addr, val);
			},
			false => {
				let mut val = self.mem_read_long(low_addr, high_addr);
				self.status.set(ProcessorStatusFlags::Carry, (val & 0x01) == 1);
				val <<= 1;
				self.status.set(ProcessorStatusFlags::Zero, val == 0);
				self.status.set(ProcessorStatusFlags::Negative, (val >> 15) == 1);
				self.mem_write_long(low_addr, high_addr, val);
			}
		}
	}

	pub fn exe_lsra(&mut self) {
		self.status.set(ProcessorStatusFlags::Carry, (self.acc & 0x1) == 1);
		self.set_acc(self.get_acc() >> 1);
		self.set_acc_nz_flag();
	}
	
	/// Subtract with Borrow from Accumulator (DP Indexed Indirect,X)
	pub fn exe_sbc(&mut self, data: u16) {
		let sum = match self.status.contains(ProcessorStatusFlags::Decimal) {
			// BCD addition
			true => {
				self.exe_sbc_bcd(data)
			},
			// Binary addition
			false => {
				self.exe_sbc_bin(data)
			}
		};
		self.set_acc(sum);
	}

	fn exe_sbc_bcd(&mut self, data: u16) -> u16 {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			//8 bit addition
			true => {				
				let data = data as u8;
				let acc = self.acc as u8;
				
				// source: https://forums.nesdev.org/viewtopic.php?t=3070
				let mut lower_nybble: u8 = (acc & 0x0F).wrapping_sub(data & 0x0F).wrapping_sub(1 + self.carry() as u8);
				let mut upper_nybble: u8 = (acc & 0xF0).wrapping_sub(data & 0xF0);
				
				

				if lower_nybble > 9 {
					lower_nybble -= 6;
					lower_nybble &= 0xF;
					upper_nybble -= 1;
				}

				if upper_nybble > 9 {
					upper_nybble -= 6;
					upper_nybble &= 0xF;
					self.status.clear_flag(ProcessorStatusFlags::Carry);
				} else {
					self.status.set_flag(ProcessorStatusFlags::Carry);
				}

				let sum_bcd = (upper_nybble << 4) + lower_nybble;

				self.status.clear_flag(
					ProcessorStatusFlags::Overflow |
					ProcessorStatusFlags::Negative |
					ProcessorStatusFlags::Zero
				);

				if sum_bcd == 0 {
					self.status.set_flag(ProcessorStatusFlags::Zero);
				}

				// If first bit is set
				if (sum_bcd as i8) < 0 {
					self.status.set_flag(ProcessorStatusFlags::Negative);
				}

				sum_bcd as u16
			},
			// 16 bit addition
			false => {
				

				// This is same implementation as 8 bits, except with a for loop
				let mut nybbles = [0_u16; 4];
				
				let mut lower_nybble = (self.acc & 0x0F) + (data & 0x0F);
				let mut upper_nybble = 0;
				for i in 0..3 {
					let nyb_sel = 0xF0 << 4*i;
					upper_nybble = (self.acc & nyb_sel) + (data & nyb_sel);
					if lower_nybble > 9 {
						lower_nybble -= 6;
						lower_nybble &= 0xF;
						upper_nybble -= 1;
					}
					nybbles[i] = lower_nybble;
					lower_nybble = upper_nybble;
				}
				nybbles[3] = upper_nybble;

				if nybbles[3] > 9 {
					nybbles[3] -= 6;
					nybbles[3] &= 0xF;
					self.status.clear_flag(ProcessorStatusFlags::Carry);
				} else {
					self.status.set_flag(ProcessorStatusFlags::Carry);
				}

				let sum_bcd = (nybbles[3] << 12) | (nybbles[2] << 8) | (nybbles[1] << 4) | nybbles[0];

				self.status.clear_flag(
					ProcessorStatusFlags::Overflow |
					ProcessorStatusFlags::Negative |
					ProcessorStatusFlags::Zero
				);

				if sum_bcd == 0 {
					self.status.set_flag(ProcessorStatusFlags::Zero);
				}

				// If first bit is set
				if (sum_bcd as i16) < 0 {
					self.status.set_flag(ProcessorStatusFlags::Negative);
				}

				sum_bcd as u16
			}
		}
	}
	
	fn exe_sbc_bin(&mut self, data: u16) -> u16 {

		let sum = self.get_acc().wrapping_sub(data).wrapping_sub(1).wrapping_add(self.carry());
		
		// If acc < sum, then sum wraps around 0, meaning carry should be propogated
		self.status.set(ProcessorStatusFlags::Carry, self.get_acc() < sum);

		// amount of bits to shift left to find if number is negative
		let b = match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			//8 bit
			true => 7,
			//16 bit
			false => 15,
		};

		self.status.set(
			ProcessorStatusFlags::Overflow, 
			(self.get_acc() >> b) == 1 && (data >> b) == 0 && (sum >> b) == 0 || 
				(self.get_acc() >> b) == 0 && (data >> b) == 1 && (sum >> b) == 1
		);

		sum
	}
	
}