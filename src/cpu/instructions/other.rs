use crate::cpu::SCpu;

impl SCpu {
	
	/// No Operation (Implied)
	pub fn exe_nop(&mut self, data: u16) {
		//NOP
	}
	
	/// Stop Processor (Implied)
	pub fn exe_stp(&mut self, data: u16) {
		// NOT IMPLEMENTED ON PURPOSE
	}
	
	/// <em>Reserved for Future Expansion</em> ()
	pub fn exe_wdm(&mut self, data: u16) {
		//NOP
	}	

	/// Exchange lower and upper byte of accumulator, always does 16 bit (Implied)
	pub fn exe_xba(&mut self, data: u16) {
		self.acc = (self.acc & 0xFF00) >> 8 | (self.acc & 0x00FF) << 8;
	}
	
}