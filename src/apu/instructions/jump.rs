use crate::apu::SApu;

impl SApu {
    pub fn exe_jmp(&mut self, target_addr: u16) {
        self.pc = target_addr;
        // Compensate for +3 from instruction length so pc lands properly
        self.pc -= 3;
    }

    pub fn exe_call(&mut self, target_addr: u16) {
        self.stack_push_long(self.pc);
        self.pc = target_addr;
        // Compensate for +3 from instruction length so pc lands properly
        self.pc -= 3
    }

    /// PCALL uses table look up with argument, however this is already handled in `instrdata`
    /// 
    /// This function should simply receive the target address, similarly to `exe_call`
    pub fn exe_pcall(&mut self, target_addr: u16) {
        self.exe_call(target_addr);
    }

    pub fn exe_tcall(&mut self, table_entry: u8) {
        let target_addr = 0xFFDE - 2 * (15 - table_entry as u16);
        self.exe_call(target_addr);
    }

    pub fn exe_ret(&mut self) {
        self.pc = self.stack_pop_long();
    }
}