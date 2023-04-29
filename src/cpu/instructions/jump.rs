use crate::cpu::SCpu;

impl SCpu {
	
	/// Jump (Absolute)
	pub fn exe_jmp(&mut self, long_addr: u32) {
		self.pc = long_addr as u16;
	}
	
	/// Jump (Absolute Long)
	pub fn exe_jml(&mut self, long_addr: u32) {
		self.pc = long_addr as u16;
		self.pbr = (long_addr >> 16) as u8
	}
	
	/// Jump to Subroutine (Absolute)
	pub fn exe_jsr(&mut self, long_addr: u32) {
		// push program counter plus 3 to get $OP of next instruction
		self.push_long_stack(self.pc.wrapping_add(2));
		self.pc = long_addr as u16;
	}
	
	/// Jump to Subroutine (Absolute Long)
	pub fn exe_jsl(&mut self, long_addr: u32) {
		// push program bank register
		self.push_byte_stack(self.pbr);
		// push program counter plus 3 to get $OP of next instruction
		self.push_long_stack(self.pc.wrapping_add(3));
		self.pc = long_addr as u16;
		self.pbr = (long_addr >> 16) as u8;
	}
	
	/// Return from Subroutine Long (Stack (RTL))
	pub fn exe_rtl(&mut self) {
		// pull and increment program counter
		self.pc = self.pull_long_stack().wrapping_add(1);
		// pull program bank register
		self.pbr = self.pull_byte_stack();
	}
	
	/// Return from Subroutine (Stack (RTS))
	pub fn exe_rts(&mut self) {
		// pull and increment program counter
		self.pc = self.pull_long_stack().wrapping_add(1);
	}
	
	
}