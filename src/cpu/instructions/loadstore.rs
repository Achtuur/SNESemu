use crate::cpu::{processorstatusflag::ProcessorStatusFlags, Cpu};

impl Cpu<'_> {
    /// Load Accumulator from Memory (DP Indexed Indirect,X)
    ///
    /// `low_addr` is the address for the low byte, `high_addr` is the address for the high byte
    pub fn exe_lda(&mut self, data: u16) {
        self.set_acc(data);
        self.set_acc_nz_flag();
    }

    /// Load Index Register X from Memory (Immediate)
    pub fn exe_ldx(&mut self, data: u16) {
        self.set_x(data);
        self.set_x_nz_flag();
    }

    /// Load Index Register Y from Memory (Immediate)
    pub fn exe_ldy(&mut self, data: u16) {
        self.set_y(data);
        self.set_y_nz_flag();
    }

    /// Store Accumulator to Memory (DP Indexed Indirect,X)
    pub fn exe_sta(&mut self, low_addr: u32, high_addr: u32) {
        match self.is_emulation_mode() {
            true => {
                self.mem_write(low_addr, self.acc() as u8);
            }

            false => {
                self.mem_write_long(low_addr, high_addr, self.acc());
            }
        }
    }

    /// Store Index Register X to Memory (Direct Page)
    pub fn exe_stx(&mut self, low_addr: u32, high_addr: u32) {
        match self.is_emulation_mode() {
            true => {
                self.mem_write(low_addr, self.x() as u8);
            }

            false => {
                self.mem_write_long(low_addr, high_addr, self.x());
            }
        }
    }

    /// Store Index Register Y to Memory (Direct Page)
    pub fn exe_sty(&mut self, low_addr: u32, high_addr: u32) {
        match self.is_emulation_mode() {
            true => {
                self.mem_write(low_addr, self.y() as u8);
            }

            false => {
                self.mem_write_long(low_addr, high_addr, self.y());
            }
        }
    }

    /// Store Zero to Memory (Direct Page)
    pub fn exe_stz(&mut self, low_addr: u32, high_addr: u32) {
        match self.is_emulation_mode() {
            true => {
                self.mem_write(low_addr, 0);
            }

            false => {
                self.mem_write_long(low_addr, high_addr, 0);
            }
        }
    }
}
