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
	}

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
			}
		}
	}

	fn exe_adc_bin(&mut self, data: u16) -> u16 {
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			// 8 bit addition
			true => {		
				// Transform data to u8 and perform addition
				let data = data as u8;
				let acc = self.acc as u8;
				let sum = acc.wrapping_add(data).wrapping_add(self.carry() as u8);

				// Check flags
				self.status.clear_flag(
					ProcessorStatusFlags::Carry | 
					ProcessorStatusFlags::Overflow |
					ProcessorStatusFlags::Negative |
					ProcessorStatusFlags::Zero
				);

				// Acc can only be larger than sum in the case of overflow
				if acc > sum {
					self.status.set_flag(ProcessorStatusFlags::Carry)
				}

				if sum == 0 {
					self.status.set_flag(ProcessorStatusFlags::Zero);
				}

				// If first bit is set
				if (sum as i8) < 0 {
					self.status.set_flag(ProcessorStatusFlags::Negative);
				}

				let acc_i8 = acc as i8;
				let data_i8 = data as i8;
				let sum_i8 = sum as i8;		

				// If sign of accumulator and data match and dont match sign of sum, there is overflow
				if acc_i8 < 0 && data_i8 < 0 && sum_i8 >= 0 || acc_i8 > 0 && data_i8 > 0 && sum_i8 <= 0 {
					self.status.set_flag(ProcessorStatusFlags::Overflow);
				}
				sum as u16
			},
			// 16 bit addition
			false => {
				let sum = self.acc.wrapping_add(data).wrapping_add(self.carry());

				// Check flags
				self.status.clear_flag(
					ProcessorStatusFlags::Carry | 
					ProcessorStatusFlags::Overflow |
					ProcessorStatusFlags::Negative |
					ProcessorStatusFlags::Zero
				);

				// Acc can only be larger than sum in the case of overflow
				if self.acc > sum {
					self.status.set_flag(ProcessorStatusFlags::Carry)
				}

				if sum == 0 {
					self.status.set_flag(ProcessorStatusFlags::Zero);
				}

				// If first bit is set
				if (sum as i16) < 0 {
					self.status.set_flag(ProcessorStatusFlags::Negative);
				}

				let acc_i16 = self.acc as i16;
				let data_i16 = data as i16;
				let sum_i16 = sum as i16;

				// If sign of accumulator and data match and dont match sign of sum, there is overflow
				if acc_i16 < 0 && data_i16 < 0 && sum_i16 >= 0 || acc_i16 > 0 && data_i16 > 0 && sum_i16 <= 0 {
					self.status.set_flag(ProcessorStatusFlags::Overflow);
				}
				sum
			}
		}
	}
	
	/// Arithmetic Shift Left (Direct Page)
	pub fn exe_asl(&mut self, data: u16) {
		todo!()
	}
	
	/// Decrement (Accumulator)
	pub fn exe_dea(&mut self, data: u16) {
		todo!()
	}
	
	/// Decrement (Direct Page) (illegal?)
	pub fn exe_dec(&mut self, data: u16) {
		todo!()
	}
	
	/// Decrement Index Register X (Implied)
	pub fn exe_dex(&mut self, data: u16) {
		todo!()
	}
	
	/// Decrement Index Register Y (Implied)
	pub fn exe_dey(&mut self, data: u16) {
		todo!()
	}
	
	/// Increment (Accumulator)
	pub fn exe_ina(&mut self, data: u16) {
		self.acc = match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			true => {
				let acc = (self.acc as u8).wrapping_add(1);
				
			}
			false => {

			}
		}
	}
	
	/// Increment (Direct Page)
	pub fn exe_inc(&mut self, data: u16) {
		todo!()
	}
	
	/// Increment Index Register X (Implied)
	pub fn exe_inx(&mut self, data: u16) {
		todo!()
	}
	
	/// Increment Index Register Y (Implied)
	pub fn exe_iny(&mut self, data: u16) {
		todo!()
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
		match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
			// 8 bit addition
			true => {		
				// Transform data to u8 and perform addition
				let data = data as u8;
				let acc = self.acc as u8;
				let sum = acc.wrapping_sub(data).wrapping_sub(1).wrapping_add(self.carry() as u8);

				// Check flags
				self.status.clear_flag(
					ProcessorStatusFlags::Carry | 
					ProcessorStatusFlags::Overflow |
					ProcessorStatusFlags::Negative |
					ProcessorStatusFlags::Zero
				);

				// Acc can only be larger than sum in the case of overflow
				if acc < sum {
					self.status.set_flag(ProcessorStatusFlags::Carry)
				}

				if sum == 0 {
					self.status.set_flag(ProcessorStatusFlags::Zero);
				}

				// If first bit is set
				if (sum as i8) < 0 {
					self.status.set_flag(ProcessorStatusFlags::Negative);
				}

				let acc_i8 = acc as i8;
				let data_i8 = data as i8;
				let sum_i8 = sum as i8;		

				// If sign of accumulator and data match and dont match sign of sum, there is overflow
				if acc_i8 < 0 && data_i8 > 0 && sum_i8 >= 0 || acc_i8 > 0 && data_i8 < 0 && sum_i8 <= 0 {
					self.status.set_flag(ProcessorStatusFlags::Overflow);
				}
				sum as u16
			},
			// 16 bit addition
			false => {
				let sum = self.acc.wrapping_sub(data).wrapping_sub(1).wrapping_add(self.carry());

				// Check flags
				self.status.clear_flag(
					ProcessorStatusFlags::Carry | 
					ProcessorStatusFlags::Overflow |
					ProcessorStatusFlags::Negative |
					ProcessorStatusFlags::Zero
				);

				// Acc can only be larger than sum in the case of overflow
				if self.acc < sum {
					self.status.set_flag(ProcessorStatusFlags::Carry)
				}

				if sum == 0 {
					self.status.set_flag(ProcessorStatusFlags::Zero);
				}

				// If first bit is set
				if (sum as i16) < 0 {
					self.status.set_flag(ProcessorStatusFlags::Negative);
				}

				let acc_i16 = self.acc as i16;
				let data_i16 = data as i16;
				let sum_i16 = sum as i16;

				// If sign of accumulator and data match and dont match sign of sum, there is overflow
				if acc_i16 < 0 && data_i16 > 0 && sum_i16 >= 0 || acc_i16 > 0 && data_i16 < 0 && sum_i16 <= 0 {
					self.status.set_flag(ProcessorStatusFlags::Overflow);
				}
				sum
			}
		}
	}
	
}