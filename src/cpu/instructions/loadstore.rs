use crate::cpu::Cpu;

impl Cpu {
	
	/// Load Accumulator from Memory (DP Indexed Indirect,X)
	pub fn exe_lda(&mut self, data: u16) {
		todo!()
	}
	
	/// Load Index Register X from Memory (Immediate)
	pub fn exe_ldx(&mut self, data: u16) {
		todo!()
	}
	
	/// Load Index Register Y from Memory (Immediate)
	pub fn exe_ldy(&mut self, data: u16) {
		todo!()
	}
	
	/// Store Accumulator to Memory (DP Indexed Indirect,X)
	pub fn exe_sta(&mut self, data: u16) {
		todo!()
	}
	
	/// Store Index Register X to Memory (Direct Page)
	pub fn exe_stx(&mut self, data: u16) {
		todo!()
	}
	
	/// Store Index Register Y to Memory (Direct Page)
	pub fn exe_sty(&mut self, data: u16) {
		todo!()
	}
	
	/// Store Zero to Memory (Direct Page)
	pub fn exe_stz(&mut self, data: u16) {
		todo!()
	}
	
	
}