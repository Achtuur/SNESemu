use crate::cpu::Cpu;

impl Cpu {
	
	/// Jump (Absolute)
	pub fn exe_jmp(&mut self, data: u16) {
		todo!()
	}
	
	/// Jump (Absolute Long)
	pub fn exe_jml(&mut self, data: u16) {
		todo!()
	}
	
	/// Jump to Subroutine (Absolute)
	pub fn exe_jsr(&mut self, data: u16) {
		todo!()
	}
	
	/// Jump to Subroutine (Absolute Long)
	pub fn exe_jsl(&mut self, data: u16) {
		todo!()
	}
	
	/// Return from Subroutine Long (Stack (RTL))
	pub fn exe_rtl(&mut self, data: u16) {
		todo!()
	}
	
	/// Return from Subroutine (Stack (RTS))
	pub fn exe_rts(&mut self, data: u16) {
		todo!()
	}
	
	
}