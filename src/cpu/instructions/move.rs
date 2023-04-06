use crate::cpu::Cpu;

impl Cpu {
	
	/// Block Move Negative (Block Move)
	/// 
	/// Instruction only works in 16 bit mode for both acc and X, Y registers
	/// 
	/// This instruction moves a single byte from source to destination and increments x/y and decrements accumulator.
	/// If accumulator is `0xFFFF` then that means the move is completed and the next instruction will be read.
	/// If not, then PC is decremented by 3 such that this instruction is executed again and the next byte is moved.
	/// 
	/// The reason we don't move all the bytes at once is because NMI interrupts can happen between moves
	/// and it thus easier to interpret every byte moved as a separate instruction to homogenise it with all other instructions
	pub fn exe_mvn(&mut self, source: u32, dest: u32) {
		// Move data
		let source_byte = self.mem_read(source);
		self.mem_write(dest, source_byte);

		// overwrite dbr
		self.dbr = (dest >> 16) as u8;

		// Increment counter
		self.x = self.x.wrapping_add(1);
		self.y = self.y.wrapping_add(1);
		self.acc = self.acc.wrapping_sub(1);
		if self.get_acc() != 0xFFFF {
			//decrement by 3 to counteract the increment by 3 that happens after execution (thus keeping pc at this instruction)
			self.pc = self.pc.wrapping_sub(3); 
		}
	}
	
	/// Block Move Positive (Block Move)
	///
	/// Instruction only works in 16 bit mode for both acc and X, Y registers
	/// 
	/// This instruction moves a single byte from source to destination and increments x/y and decrements accumulator.
	/// If accumulator is `0xFFFF` then that means the move is completed and the next instruction will be read.
	/// If not, then PC is decremented by 3 such that this instruction is executed again and the next byte is moved.
	/// 
	/// The reason we don't move all the bytes at once is because NMI interrupts can happen between moves
	/// and it thus easier to interpret every byte moved as a separate instruction to homogenise it with all other instructions
	pub fn exe_mvp(&mut self, source: u32, dest: u32) {
		// Move data
		let source_byte = self.mem_read(source);
		self.mem_write(dest, source_byte);
		
		// overwrite dbr
		self.dbr = (dest >> 16) as u8;

		// Increment counter
		self.x = self.x.wrapping_sub(1);
		self.y = self.y.wrapping_sub(1);
		self.acc = self.acc.wrapping_sub(1);
		if self.get_acc() != 0xFFFF {
			//decrement by 3 to counteract the increment by 3 that happens after execution (thus keeping pc at this instruction)
			self.pc = self.pc.wrapping_sub(3); 
		}
	}
	
	
}