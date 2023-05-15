use crate::cpu::{SCpu, statusflag::StatusFlags};

impl SCpu {

	fn branch(&mut self, target_addr: u32) {
		self.pc = target_addr as u16;
		// Correct pc for instruction length when branch is taken
		self.pc = self.pc.wrapping_sub(2);
	}
	
	/// Branch if Carry Clear (Program Counter Relative)
	pub fn exe_bcc(&mut self, target_addr: u32) {
		if !self.status.contains(StatusFlags::Carry) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Carry Set (Program Counter Relative)
	pub fn exe_bcs(&mut self, target_addr: u32) {
		if self.status.contains(StatusFlags::Carry) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Equal (Program Counter Relative)
	pub fn exe_beq(&mut self, target_addr: u32) {
		if self.status.contains(StatusFlags::Zero) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Minus (Program Counter Relative)
	pub fn exe_bmi(&mut self, target_addr: u32) {
		if self.status.contains(StatusFlags::Negative) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Not Equal (Program Counter Relative)
	pub fn exe_bne(&mut self, target_addr: u32) {
		if !self.status.contains(StatusFlags::Zero) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Plus (Program Counter Relative)
	pub fn exe_bpl(&mut self, target_addr: u32) {
		if !self.status.contains(StatusFlags::Negative) {
			self.branch(target_addr);
		}
	}
	
	/// Branch Always (Program Counter Relative)
	pub fn exe_bra(&mut self, target_addr: u32) {
		self.branch(target_addr);
	}
	
	/// Branch Long Always (Program Counter Relative Long)
	/// 
	/// The 'long' part here means that the `target_addr` range is larger compared to non long branch instructions.
	/// For the execution here this doesnt matter, as the `target_addr` is calculated beforehand
	pub fn exe_brl(&mut self, target_addr: u32) {
		self.branch(target_addr);
	}
	
	/// Branch if Overflow Clear (Program Counter Relative)
	pub fn exe_bvc(&mut self, target_addr: u32) {
		if !self.status.contains(StatusFlags::Overflow) {
			self.branch(target_addr);
		}
	}
	
	/// Branch if Overflow Set (Program Counter Relative)
	pub fn exe_bvs(&mut self, target_addr: u32) {
		if self.status.contains(StatusFlags::Overflow) {
			self.branch(target_addr);
		}
	}
}


#[cfg(test)]
mod tests {
	use crate::{cpu::{statusflag::StatusFlags, SCpu}, arc_mut, apu::memory::ApuMemory, ppu::memory::PpuMemory};
	use crate::to_long;

	fn get_test_cpu() -> SCpu {
		let mut cpu = SCpu::new();
		let ppumem = arc_mut!(PpuMemory::new());
        cpu.memory.set_ppumemory_ref(ppumem.clone());
		let apumem = arc_mut!(ApuMemory::new());
        cpu.memory.set_apumemory_ref(apumem.clone());
		cpu
	}

	#[test]
	fn test_bcc() {
		let mut cpu = get_test_cpu();
		cpu.pbr = 0;
		cpu.pc = 0xABCD;
		let target_addr = cpu.get_pc_addr() + 2 + 0xAD;
		// Should NOT branch now
		cpu.status.set_flag(StatusFlags::Carry);
		cpu.exe_bcc(target_addr);
		assert_eq!(cpu.get_pc_addr(), to_long!(cpu.pbr, cpu.pc));
		// Should branch now
		cpu.status.clear_flag(StatusFlags::Carry);
		cpu.exe_bcc(target_addr);
		assert_eq!(cpu.get_pc_addr(), target_addr);
	}

	#[test]
	fn test_bcs() {
		let mut cpu = get_test_cpu();
		cpu.pbr = 0;
		cpu.pc = 0xABCD;
		let target_addr = cpu.get_pc_addr() + 2 + 0xAD;
		// Should NOT branch now
		cpu.status.clear_flag(StatusFlags::Carry);
		cpu.exe_bcs(target_addr);
		assert_eq!(cpu.get_pc_addr(), to_long!(cpu.pbr, cpu.pc));
		// Should branch now
		cpu.status.set_flag(StatusFlags::Carry);
		cpu.exe_bcs(target_addr);
		assert_eq!(cpu.get_pc_addr(), target_addr);
	}

	#[test]
	fn test_beq() {
		let mut cpu = get_test_cpu();
		cpu.pbr = 0;
		cpu.pc = 0xABCD;
		let target_addr = cpu.get_pc_addr() + 2 + 0xAD;
		// Should NOT branch now
		cpu.status.clear_flag(StatusFlags::Zero);
		cpu.exe_beq(target_addr);
		assert_eq!(cpu.get_pc_addr(), to_long!(cpu.pbr, cpu.pc));
		// Should branch now
		cpu.status.set_flag(StatusFlags::Zero);
		cpu.exe_beq(target_addr);
		assert_eq!(cpu.get_pc_addr(), target_addr);
	}

	#[test]
	fn test_bmi() {
		let mut cpu = get_test_cpu();
		cpu.pbr = 0;
		cpu.pc = 0xABCD;
		let target_addr = cpu.get_pc_addr() + 2 + 0xAD;
		// Should NOT branch now
		cpu.status.clear_flag(StatusFlags::Negative);
		cpu.exe_bmi(target_addr);
		assert_eq!(cpu.get_pc_addr(), to_long!(cpu.pbr, cpu.pc));
		// Should branch now
		cpu.status.set_flag(StatusFlags::Negative);
		cpu.exe_bmi(target_addr);
		assert_eq!(cpu.get_pc_addr(), target_addr);
	}

	#[test]
	fn test_bne() {
		let mut cpu = get_test_cpu();
		cpu.pbr = 0;
		cpu.pc = 0xABCD;
		let target_addr = cpu.get_pc_addr() + 2 + 0xAD;
		// Should NOT branch now
		cpu.status.set_flag(StatusFlags::Zero);
		cpu.exe_bne(target_addr);
		assert_eq!(cpu.get_pc_addr(), to_long!(cpu.pbr, cpu.pc));
		// Should branch now
		cpu.status.clear_flag(StatusFlags::Zero);
		cpu.exe_bne(target_addr);
		assert_eq!(cpu.get_pc_addr(), target_addr);
	}

	#[test]
	fn test_bpl() {
		let mut cpu = get_test_cpu();
		cpu.pbr = 0;
		cpu.pc = 0xABCD;
		let target_addr = cpu.get_pc_addr() + 2 + 0xAD;
		// Should NOT branch now
		cpu.status.set_flag(StatusFlags::Negative);
		cpu.exe_bpl(target_addr);
		assert_eq!(cpu.get_pc_addr(), to_long!(cpu.pbr, cpu.pc));
		// Should branch now
		cpu.status.clear_flag(StatusFlags::Negative);
		cpu.exe_bpl(target_addr);
		assert_eq!(cpu.get_pc_addr(), target_addr);
	}

	#[test]
	fn test_bra() {
		let mut cpu = get_test_cpu();
		cpu.pbr = 0;
		cpu.pc = 0xABCD;
		let target_addr = cpu.get_pc_addr() + 2 + 0xAD;
		
		cpu.exe_bra(target_addr);
		assert_eq!(cpu.get_pc_addr(), target_addr);
	}

	#[test]
	fn test_brl() {
		let mut cpu = get_test_cpu();
		cpu.pbr = 0;
		cpu.pc = 0xABCD;
		let target_addr = cpu.get_pc_addr() + 2 + 0xAD;
		
		cpu.exe_brl(target_addr);
		assert_eq!(cpu.get_pc_addr(), target_addr);
	}

	#[test]
	fn test_bvc() {
		let mut cpu = get_test_cpu();
		cpu.pbr = 0;
		cpu.pc = 0xABCD;
		let target_addr = cpu.get_pc_addr() + 2 + 0xAD;
		// Should NOT branch now
		cpu.status.set_flag(StatusFlags::Overflow);
		cpu.exe_bvc(target_addr);
		assert_eq!(cpu.get_pc_addr(), to_long!(cpu.pbr, cpu.pc));
		// Should branch now
		cpu.status.clear_flag(StatusFlags::Overflow);
		cpu.exe_bvc(target_addr);
		assert_eq!(cpu.get_pc_addr(), target_addr);
	}

	#[test]
	fn test_bvs() {
		let mut cpu = get_test_cpu();
		cpu.pbr = 0;
		cpu.pc = 0xABCD;
		let target_addr = cpu.get_pc_addr() + 2 + 0xAD;
		// Should NOT branch now
		cpu.status.clear_flag(StatusFlags::Overflow);
		cpu.exe_bvs(target_addr);
		assert_eq!(cpu.get_pc_addr(), to_long!(cpu.pbr, cpu.pc));
		// Should branch now
		cpu.status.set_flag(StatusFlags::Overflow);
		cpu.exe_bvs(target_addr);
		assert_eq!(cpu.get_pc_addr(), target_addr);
	}
}