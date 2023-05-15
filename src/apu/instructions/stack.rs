use crate::apu::{SApu, statusword::StatusWord};

impl SApu {
    pub fn exe_pushp(&mut self) {
        self.stack_push(self.status.get_bits());
    }

    pub fn exe_pusha(&mut self) {
        self.stack_push(self.acc);
    }

    pub fn exe_pushx(&mut self) {
        self.stack_push(self.x);
    }

    pub fn exe_pushy(&mut self) {
        self.stack_push(self.y);
    }

    pub fn exe_popp(&mut self) {
        self.status = StatusWord::from_bits(self.stack_pop() as u16).unwrap();
    }

    pub fn exe_popa(&mut self) {
        self.acc = self.stack_pop();
    }

    pub fn exe_popx(&mut self) {
        self.x = self.stack_pop();
    }

    pub fn exe_popy(&mut self) {
        self.y = self.stack_pop();
    }
}