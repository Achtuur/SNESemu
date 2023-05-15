use crate::apu::{SApu, statusword::StatusWord};

impl SApu {
    pub fn exe_brk(&mut self) {
        self.stack_push(self.status.get_bits());
        self.stack_push_long(self.pc);
        self.status.set_flag(StatusWord::Break);
        self.status.clear_flag(StatusWord::InterruptEnable);
        // self.pc = ??
    }

    pub fn exe_rti(&mut self) {
        self.pc = self.stack_pop_long();
        self.status = StatusWord::from_bits(self.stack_pop() as u16).unwrap();
    }
}