use crate::cpu::Cpu;

impl Cpu {
	
	/// Break (Stack/Interrupt)
	pub fn exe_brk(&mut self, data: u16) {
		todo!()
	}
	
	/// Co-Processor (Stack/Interrupt)
	pub fn exe_cop(&mut self, data: u16) {
		todo!()
	}
	
	/// Return from Interrupt (Stack (RTI))
	pub fn exe_rti(&mut self, data: u16) {
		todo!()
	}
	
	/// Wait for Interrupt (Implied)
	pub fn exe_wai(&mut self, data: u16) {
		todo!()
	}
	
	
}