use crate::cpu::Cpu;

impl Cpu<'_> {
    /// Transfer Accumulator to Index Register X (Implied)
    pub fn exe_tax(&mut self) {
        self.set_x(self.acc());
    }

    /// Transfer Accumulator to Index Register Y (Implied)
    pub fn exe_tay(&mut self) {
        self.set_y(self.acc());
    }

    /// Transfer 16-bit Accumulator to Direct Page Register (Implied)
    pub fn exe_tcd(&mut self) {
        self.dp = self.acc();
    }

    /// Transfer 16-bit Accumulator to Stack Pointer (Implied)
    pub fn exe_tcs(&mut self) {
        self.sp = self.acc();
    }

    /// Transfer Direct Page Register to 16-bit Accumulator (Implied)
    pub fn exe_tdc(&mut self) {
        self.set_acc(self.dp);
    }

    /// Transfer Stack Pointer to 16-bit Accumulator (Implied)
    pub fn exe_tsc(&mut self) {
        self.set_acc(self.sp);
    }

    /// Transfer Stack Pointer to Index Register X (Implied)
    pub fn exe_tsx(&mut self) {
        self.set_x(self.sp);
    }

    /// Transfer Index Register X to Accumulator (Implied)
    pub fn exe_txa(&mut self) {
        self.set_acc(self.x());
    }

    /// Transfer Index Register X to Stack Pointer (Implied)
    pub fn exe_txs(&mut self) {
        self.sp = self.x();
    }

    /// Transfer Index Register X to Index Register Y (Implied)
    pub fn exe_txy(&mut self) {
        self.set_y(self.x());
    }

    /// Transfer Index Register Y to Accumulator (Implied)
    pub fn exe_tya(&mut self) {
        self.set_acc(self.y());
    }

    /// Transfer Index Register Y to Index Register X (Implied)
    pub fn exe_tyx(&mut self) {
        self.set_x(self.y())
    }
}
