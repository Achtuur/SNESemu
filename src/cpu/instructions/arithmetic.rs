use crate::{cpu::{SCpu, statusflag::StatusFlags}, bit_set};

impl SCpu {
	
	/// Add With Carry (DP Indexed Indirect,X)
	pub fn exe_adc(&mut self, data: u16) {
		let sum = match self.status.contains(StatusFlags::Decimal) {
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
	/// 
	/// Currently not completely functional, however no game uses this flag I think(?)
	fn exe_adc_bcd(&mut self, data: u16) -> u16 {
		match self.status.contains(StatusFlags::Accumulator8bit) {
			//8 bit addition
			true => {
				let data = data as u8;
				let acc = self.get_acc() as u8;

				// source: https://forums.nesdev.org/viewtopic.php?t=3070
				let mut lower_nybble = (acc & 0x0F) + (data & 0x0F) + self.carry() as u8;
				let mut upper_nybble = (acc & 0xF0) + (data & 0xF0);

				if lower_nybble > 9 {
					lower_nybble = (lower_nybble + 6) & 0xF;
					upper_nybble += 1;
				}

				if upper_nybble > 9 {
					upper_nybble = (upper_nybble + 6) % 0xF;
					self.status.set_flag(StatusFlags::Carry);
				} else {
					self.status.clear_flag(StatusFlags::Carry);
				}

				let sum_bcd = (upper_nybble << 4) + lower_nybble;

				// Overflow is unreliable with bcd, so just always disable
				self.status.clear_flag(StatusFlags::Overflow);

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
					self.status.set_flag(StatusFlags::Carry);
				} else {
					self.status.clear_flag(StatusFlags::Carry);
				}

				
				let sum_bcd = (nybbles[3] << 12) | (nybbles[2] << 8) | (nybbles[1] << 4) | nybbles[0];

				// Overflow is unreliable with bcd, so just always disable
				self.status.clear_flag(StatusFlags::Overflow);

				sum_bcd as u16
			}
		}
	}

	/// Add binary numbers
	fn exe_adc_bin(&mut self, data: u16) -> u16 {
		let sum = self.get_acc().wrapping_add(data).wrapping_add(self.carry());
		
		// If acc > sum, then sum wraps around u8/u16 limit, meaning carry should be propogated
		match self.status.contains(StatusFlags::Accumulator8bit) {
			true => self.status.set(StatusFlags::Carry, sum > u8::MAX as u16),
			false => self.status.set(StatusFlags::Carry, self.get_acc() > sum),
		}

		// number of bits to shift left to find if number is negative
		let b = match self.status.contains(StatusFlags::Accumulator8bit) {
			// 8 bit
			true => 7,
			// 16 bit
			false => 15,
		};

		self.status.set(
			StatusFlags::Overflow, 
			bit_set!(self.get_acc(), b) && bit_set!(data, b) && !bit_set!(sum, b) ||
			!bit_set!(self.get_acc(), b) && !bit_set!(data, b) && bit_set!(sum, b)
		);

		sum
	}
	
	/// Arithmetic Shift Left (Direct Page)
	pub fn exe_asl(&mut self, low_addr: u32, high_addr: u32) {
		match self.status.contains(StatusFlags::Accumulator8bit) {
			true => {
				let mut val = self.mem_read(low_addr);
				self.status.set(StatusFlags::Carry, bit_set!(val, 7));
				val <<= 1;
				self.status.set(StatusFlags::Zero, val == 0);
				self.status.set(StatusFlags::Negative, bit_set!(val, 7));
				self.mem_write(low_addr, val);
			},
			false => {
				let mut val = self.mem_read_long(low_addr, high_addr);
				self.status.set(StatusFlags::Carry, bit_set!(val, 15));
				val <<= 1;
				self.status.set(StatusFlags::Zero, val == 0);
				self.status.set(StatusFlags::Negative, bit_set!(val, 15));
				self.mem_write_long(low_addr, high_addr, val);
			}
		}
	}

	/// Arithmetic Shift Left (Accumulator)
	/// 
	/// the asla name was made up by me to differentiate it from asl
	pub fn exe_asla(&mut self) {
		match self.status.contains(StatusFlags::Accumulator8bit) {
			true => self.status.set(StatusFlags::Carry, bit_set!(self.get_acc(), 7)),
			false => self.status.set(StatusFlags::Carry, bit_set!(self.get_acc(), 15)),
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
		match self.status.contains(StatusFlags::Accumulator8bit) {
			true => {
				let val = self.mem_read(low_addr).wrapping_sub(1);
				self.status.set(StatusFlags::Zero, val == 0);
				self.status.set(StatusFlags::Negative, bit_set!(val, 7));
				self.mem_write(low_addr, val);
			},
			false => {
				let val = self.mem_read_long(low_addr, high_addr).wrapping_sub(1);
				self.status.set(StatusFlags::Zero, val == 0);
				self.status.set(StatusFlags::Negative, bit_set!(val, 15));
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
		match self.status.contains(StatusFlags::Accumulator8bit) {
			true => {
				let val = self.mem_read(low_addr).wrapping_sub(1);
				self.status.set(StatusFlags::Zero, val == 0);
				self.status.set(StatusFlags::Negative, bit_set!(val, 7));
				self.mem_write(low_addr, val);
			},
			false => {
				let val = self.mem_read_long(low_addr, high_addr).wrapping_sub(1);
				self.status.set(StatusFlags::Zero, val == 0);
				self.status.set(StatusFlags::Negative, bit_set!(val, 15));
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
		match self.status.contains(StatusFlags::Accumulator8bit) {
			true => {
				let mut val = self.mem_read(low_addr);
				self.status.set(StatusFlags::Carry, bit_set!(val, 1));
				val >>= 1;
				self.status.set(StatusFlags::Zero, val == 0);
				self.status.set(StatusFlags::Negative, bit_set!(val, 7));
				self.mem_write(low_addr, val);
			},
			false => {
				let mut val = self.mem_read_long(low_addr, high_addr);
				self.status.set(StatusFlags::Carry, bit_set!(val, 1));
				val >>= 1;
				self.status.set(StatusFlags::Zero, val == 0);
				self.status.set(StatusFlags::Negative, bit_set!(val, 15));
				self.mem_write_long(low_addr, high_addr, val);
			}
		}
	}

	pub fn exe_lsra(&mut self) {
		self.status.set(StatusFlags::Carry, bit_set!(self.get_acc(), 1));
		self.set_acc(self.get_acc() >> 1);
		self.set_acc_nz_flag();
	}
	
	/// Subtract with Borrow from Accumulator (DP Indexed Indirect,X)
	pub fn exe_sbc(&mut self, data: u16) {
		let sum = match self.status.contains(StatusFlags::Decimal) {
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
		match self.status.contains(StatusFlags::Accumulator8bit) {
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
					self.status.clear_flag(StatusFlags::Carry);
				} else {
					self.status.set_flag(StatusFlags::Carry);
				}

				let sum_bcd = (upper_nybble << 4) + lower_nybble;

				self.status.clear_flag(
					StatusFlags::Overflow |
					StatusFlags::Negative |
					StatusFlags::Zero
				);

				if sum_bcd == 0 {
					self.status.set_flag(StatusFlags::Zero);
				}

				// If first bit is set
				if (sum_bcd as i8) < 0 {
					self.status.set_flag(StatusFlags::Negative);
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
					self.status.clear_flag(StatusFlags::Carry);
				} else {
					self.status.set_flag(StatusFlags::Carry);
				}

				let sum_bcd = (nybbles[3] << 12) | (nybbles[2] << 8) | (nybbles[1] << 4) | nybbles[0];

				self.status.clear_flag(
					StatusFlags::Overflow |
					StatusFlags::Negative |
					StatusFlags::Zero
				);

				if sum_bcd == 0 {
					self.status.set_flag(StatusFlags::Zero);
				}

				// If first bit is set
				if (sum_bcd as i16) < 0 {
					self.status.set_flag(StatusFlags::Negative);
				}

				sum_bcd as u16
			}
		}
	}
	
	fn exe_sbc_bin(&mut self, data: u16) -> u16 {

		let sum = self.get_acc().wrapping_sub(data).wrapping_sub(1).wrapping_add(self.carry());
		
		// If acc < sum, then sum wraps around 0, meaning carry should be propogated
		self.status.set(StatusFlags::Carry, self.get_acc() < sum);

		// amount of bits to shift left to find if number is negative
		let b = match self.status.contains(StatusFlags::Accumulator8bit) {
			//8 bit
			true => 7,
			//16 bit
			false => 15,
		};

		self.status.set(
			StatusFlags::Overflow, 
			(self.get_acc() >> b) == 1 && (data >> b) == 0 && (sum >> b) == 0 || 
				(self.get_acc() >> b) == 0 && (data >> b) == 1 && (sum >> b) == 1
		);

		sum
	}
	
}

#[cfg(test)]
mod tests {
    use crate::{cpu::{statusflag::StatusFlags, SCpu}, arc_mut, apu::memory::ApuMemory, ppu::memory::PpuMemory};

	fn get_test_cpu() -> SCpu {
		let mut cpu = SCpu::new();
		let ppumem = arc_mut!(PpuMemory::new());
        cpu.memory.set_ppumemory_ref(ppumem.clone());
		let apumem = arc_mut!(ApuMemory::new());
        cpu.memory.set_apumemory_ref(apumem.clone());
		cpu
	}

	#[test]
	fn test_adc_bit_8() {
		let mut cpu = get_test_cpu();
		cpu.status.set_flag(StatusFlags::Accumulator8bit);
		cpu.status.clear_flag(StatusFlags::Decimal);

		for a in 0..u8::MAX as u16 {
			for b in 0..u16::MAX {
				cpu.set_acc(a);
				cpu.status.clear_flag(StatusFlags::Carry);
				cpu.exe_adc(b);
				assert_eq!(a.wrapping_add(b) & 0xFF, cpu.get_acc());
			}
		}
	}
	
	#[test]
	fn test_adc_bit_16() {
		let mut cpu = get_test_cpu();
		cpu.status.clear_flag(StatusFlags::Accumulator8bit);
		cpu.status.clear_flag(StatusFlags::Decimal);

		for a in 0..u8::MAX as u16 {
			for b in 0..u16::MAX {
				cpu.set_acc(a);
				cpu.status.clear_flag(StatusFlags::Carry);
				cpu.exe_adc(b);
				assert_eq!(a.wrapping_add(b), cpu.get_acc());
			}
		}
	}
	
	#[test]
	fn test_adc_bcd_8() {
		let mut cpu = get_test_cpu();
		cpu.status.set_flag(StatusFlags::Accumulator8bit);
		cpu.status.set_flag(StatusFlags::Decimal);

		cpu.set_acc(0x03);
		cpu.exe_adc(0x09);
		assert_eq!(cpu.get_acc(), 0x0012);

		// cpu.set_acc(0b0101_0110); // 56
		// cpu.exe_adc(0b0001_1000); // 18
		// println!("cpu.get_acc(): 0x{0:04X}", cpu.get_acc());
		// assert_eq!(cpu.get_acc(), 74);
	}
	
	#[test]
	fn test_adc_bcd_16() {
		// todo!()
	}

	#[test]
	fn test_asl_8() {
		let mut cpu = get_test_cpu();
		cpu.status.set_flag(StatusFlags::Accumulator8bit);
		let low_addr = 0x7F00AB;
		let high_addr = 0;
		for a in 0..u8::MAX {
			cpu.status.clear_flag(StatusFlags::Carry);
			cpu.mem_write(low_addr, a);
			cpu.exe_asl(low_addr, high_addr);
			let v = cpu.mem_read(low_addr);
			assert_eq!(v, a << 1);
		}
	}

	#[test]
	fn test_asl_16() {
		let mut cpu = get_test_cpu();
		cpu.status.clear_flag(StatusFlags::Accumulator8bit);
		let low_addr = 0x7F00AB;
		let high_addr = 0x7F00AC;
		for a in 0..u16::MAX {
			cpu.status.clear_flag(StatusFlags::Carry);
			cpu.mem_write_long(low_addr, high_addr, a);
			cpu.exe_asl(low_addr, high_addr);
			let v = cpu.mem_read_long(low_addr, high_addr);
			assert_eq!(v, a << 1);
		}
	}

	#[test]
	fn test_asla() {
		let mut cpu = get_test_cpu();
		
		// 8 bit
		cpu.status.set_flag(StatusFlags::Accumulator8bit);
		for a in 0..u16::MAX {
			cpu.set_acc(a);
			cpu.exe_asla();
			assert_eq!(cpu.get_acc(), (a << 1) & 0xFF);
		}

		// 16 bit
		cpu.status.clear_flag(StatusFlags::Accumulator8bit);
		for a in 0..u16::MAX {
			cpu.set_acc(a);
			cpu.exe_asla();
			assert_eq!(cpu.get_acc(), a << 1);
		}
	}

	#[test]
	fn test_dea() {
		let mut cpu = get_test_cpu();

		// 8 bit
		cpu.status.set_flag(StatusFlags::Accumulator8bit);
		for a in 0..u16::MAX {
			cpu.set_acc(a);
			cpu.exe_dea();
			assert_eq!(cpu.get_acc(), a.wrapping_sub(1) & 0xFF);
		}

		// 16 bit
		cpu.status.clear_flag(StatusFlags::Accumulator8bit);
		for a in 0..u16::MAX {
			cpu.set_acc(a);
			cpu.exe_dea();
			assert_eq!(cpu.get_acc(), a.wrapping_sub(1));
		}
	}

	#[test]
	fn test_dex() {
		let mut cpu = get_test_cpu();

		// 8 bit
		cpu.status.set_flag(StatusFlags::XYreg8bit);
		for a in 0..u16::MAX {
			cpu.set_x(a);
			cpu.exe_dex();
			assert_eq!(cpu.get_x(), a.wrapping_sub(1) & 0xFF);
		}

		// 16 bit
		cpu.status.clear_flag(StatusFlags::XYreg8bit);
		for a in 0..u16::MAX {
			cpu.set_x(a);
			cpu.exe_dex();
			assert_eq!(cpu.get_x(), a.wrapping_sub(1));
		}
	}

	#[test]
	fn test_dey() {
		let mut cpu = get_test_cpu();

		// 8 bit
		cpu.status.set_flag(StatusFlags::XYreg8bit);
		for a in 0..u16::MAX {
			cpu.set_y(a);
			cpu.exe_dey();
			assert_eq!(cpu.get_y(), a.wrapping_sub(1) & 0xFF);
		}

		// 16 bit
		cpu.status.clear_flag(StatusFlags::XYreg8bit);
		for a in 0..u16::MAX {
			cpu.set_y(a);
			cpu.exe_dey();
			assert_eq!(cpu.get_y(), a.wrapping_sub(1));
		}
	}

	#[test]
	fn test_dec_8() {
		let mut cpu = get_test_cpu();
		cpu.status.set_flag(StatusFlags::Accumulator8bit);
		let low_addr = 0x7F00AB;
		let high_addr = 0x7F00AC;
		for a in 0..u8::MAX {
			cpu.status.clear_flag(StatusFlags::Carry);
			cpu.mem_write(low_addr, a);
			
			cpu.exe_dec(low_addr, high_addr);
			let v = cpu.mem_read(low_addr);
			assert_eq!(v, a.wrapping_sub(1));
		}
	}

	#[test]
	fn test_dec_16() {
		let mut cpu = get_test_cpu();
		cpu.status.clear_flag(StatusFlags::Accumulator8bit);
		let low_addr = 0x7F00AB;
		let high_addr = 0x7F00AC;
		for a in 0..u16::MAX {
			cpu.status.clear_flag(StatusFlags::Carry);
			cpu.mem_write_long(low_addr, high_addr, a);

			cpu.exe_dec(low_addr, high_addr);
			let v = cpu.mem_read_long(low_addr, high_addr);
			assert_eq!(v, a.wrapping_sub(1));
		}
	}

	#[test]
	fn test_ina() {
		let mut cpu = get_test_cpu();

		// 8 bit
		cpu.status.set_flag(StatusFlags::Accumulator8bit);
		for a in 0..u16::MAX {
			cpu.set_acc(a);
			cpu.exe_ina();
			assert_eq!(cpu.get_acc(), a.wrapping_add(1) & 0xFF);
		}

		// 16 bit
		cpu.status.clear_flag(StatusFlags::Accumulator8bit);
		for a in 0..u16::MAX {
			cpu.set_acc(a);
			cpu.exe_ina();
			assert_eq!(cpu.get_acc(), a.wrapping_add(1));
		}
	}

	#[test]
	fn test_inx() {
		let mut cpu = get_test_cpu();

		// 8 bit
		cpu.status.set_flag(StatusFlags::XYreg8bit);
		for a in 0..u16::MAX {
			cpu.set_x(a);
			cpu.exe_inx();
			assert_eq!(cpu.get_x(), a.wrapping_add(1) & 0xFF);
		}

		// 16 bit
		cpu.status.clear_flag(StatusFlags::XYreg8bit);
		for a in 0..u16::MAX {
			cpu.set_x(a);
			cpu.exe_inx();
			assert_eq!(cpu.get_x(), a.wrapping_add(1));
		}
	}

	#[test]
	fn test_iny() {
		let mut cpu = get_test_cpu();

		// 8 bit
		cpu.status.set_flag(StatusFlags::XYreg8bit);
		for a in 0..u16::MAX {
			cpu.set_y(a);
			cpu.exe_iny();
			assert_eq!(cpu.get_y(), a.wrapping_add(1) & 0xFF);
		}

		// 16 bit
		cpu.status.clear_flag(StatusFlags::XYreg8bit);
		for a in 0..u16::MAX {
			cpu.set_y(a);
			cpu.exe_iny();
			assert_eq!(cpu.get_y(), a.wrapping_add(1));
		}
	}

	#[test]
	fn test_lsr_8() {
		let mut cpu = get_test_cpu();
		cpu.status.set_flag(StatusFlags::Accumulator8bit);
		let low_addr = 0x7F00AB;
		let high_addr = 0;
		for a in 0..u8::MAX {
			cpu.status.clear_flag(StatusFlags::Carry);
			cpu.mem_write(low_addr, a);
			cpu.exe_lsr(low_addr, high_addr);
			let v = cpu.mem_read(low_addr);
			assert_eq!(v, a >> 1);
		}
	}

	#[test]
	fn test_lsr_16() {
		let mut cpu = get_test_cpu();
		cpu.status.clear_flag(StatusFlags::Accumulator8bit);
		let low_addr = 0x7F00AB;
		let high_addr = 0x7F00AC;
		for a in 0..u16::MAX {
			cpu.status.clear_flag(StatusFlags::Carry);
			cpu.mem_write_long(low_addr, high_addr, a);
			cpu.exe_lsr(low_addr, high_addr);
			let v = cpu.mem_read_long(low_addr, high_addr);
			assert_eq!(v, a >> 1);
		}
	}

	#[test]
	fn test_lsra() {
		let mut cpu = get_test_cpu();
		
		// 8 bit
		cpu.status.set_flag(StatusFlags::Accumulator8bit);
		for a in 0..u16::MAX {
			cpu.set_acc(a);
			cpu.exe_lsra();
			assert_eq!(cpu.get_acc(), (a & 0xFF) >> 1);
		}

		// 16 bit
		cpu.status.clear_flag(StatusFlags::Accumulator8bit);
		for a in 0..u16::MAX {
			cpu.set_acc(a);
			cpu.exe_lsra();
			assert_eq!(cpu.get_acc(), a >> 1);
		}
	}


	#[test]
	fn test_sbc_bit_8() {
		let mut cpu = get_test_cpu();
		cpu.status.set_flag(StatusFlags::Accumulator8bit);
		cpu.status.clear_flag(StatusFlags::Decimal);

		for a in 0..u8::MAX as u16 {
			for b in 0..u16::MAX {
				cpu.set_acc(a);
				cpu.status.set_flag(StatusFlags::Carry);
				cpu.exe_sbc(b);
				assert_eq!(a.wrapping_sub(b) & 0xFF, cpu.get_acc());
			}
		}
	}
	
	#[test]
	fn test_sbc_bit_16() {
		let mut cpu = get_test_cpu();
		cpu.status.clear_flag(StatusFlags::Accumulator8bit);
		cpu.status.clear_flag(StatusFlags::Decimal);

		for a in 0..u8::MAX as u16 {
			for b in 0..u16::MAX {
				cpu.set_acc(a);
				cpu.status.set_flag(StatusFlags::Carry);
				cpu.exe_sbc(b);
				assert_eq!(a.wrapping_sub(b), cpu.get_acc());
			}
		}
	}

}