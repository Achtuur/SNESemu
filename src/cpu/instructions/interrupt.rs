use crate::cpu::Cpu;

impl Cpu {
	
	pub fn exe_interrupt(&mut self, interrupt_vector: u32) {
		self.push_byte_stack(self.pbr);
		self.push_long_stack(self.pc.wrapping_add(2));
		self.push_byte_stack(self.status.get_bits());
		self.pc = self.mem_read_long(interrupt_vector, interrupt_vector.wrapping_add(1));
	}

	/// Break (Stack/Interrupt)
	pub fn exe_brk(&mut self, data: u16) {
		self.exe_interrupt(0x00FFE6);
	}
	
	/// Co-Processor (Stack/Interrupt)
	pub fn exe_cop(&mut self, data: u16) {
		self.exe_interrupt(0x00FFE4);
	}
	
	/// Return from Interrupt (Stack (RTI))
	pub fn exe_rti(&mut self, data: u16) {
		let status_bits = self.pull_byte_stack();
		self.status.set_bits(status_bits);
		self.pc = self.pull_long_stack();
		self.pbr = self.pull_byte_stack();

	}
	
	/// Wait for Interrupt (Implied)
	pub fn exe_wai(&mut self, data: u16) {
		todo!()
	}
	
	
}