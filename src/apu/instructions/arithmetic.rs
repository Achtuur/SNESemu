use crate::{apu::{SApu, execute::instrdata::InstrData, statusword::StatusWord}, bit_set, bit_slice};

use super::AddressingMode;

impl SApu {
    pub fn exe_adc(&mut self, instr_data: InstrData) {
        // Get lhs and rhs of addition, depending on addressing mode
        let (lhs, rhs) = match instr_data.addr_mode {
            AddressingMode::Immediate => (self.acc, instr_data.imm_data.unwrap()),

            AddressingMode::DPImmediate |
            AddressingMode::DPtoDP |
            AddressingMode::XIndirectYIndirect => (instr_data.addr_data.unwrap(), instr_data.imm_data.unwrap()),

            _ => (self.acc, instr_data.addr_data.unwrap()),

        };

        let sum = lhs.wrapping_add(rhs).wrapping_add(self.carry());

        // If LHS of operation larger than sum, sum is outside u8 range, meaning carry should be set
        self.status.set(StatusWord::Carry, lhs > sum);

        // If sign bit of lhs and rhs matches with each other, but not the sum, overflow occured
        self.status.set(StatusWord::Overflow, 
            bit_set!(lhs, 7) && bit_set!(rhs, 7) && !bit_set!(sum, 7) ||
            !bit_set!(lhs, 7) && !bit_set!(rhs, 7) && bit_set!(sum, 7) 
        );

        // If sum[0..3] < lhs[0..3] (|| rhs[0..3]) there was a carry from lower to upper nybble
        // Can also check rhs[0..3] as sum[0..3] cannot be larger than either lhs or rhs if there was a carry
        let lower_nybble_diff = bit_slice!(sum, 0, 3).wrapping_sub(bit_slice!(lhs, 0, 3));
        // Enable half-carry flag if carry from lower nybble to higher nybble (difference < 0)
        self.status.set(StatusWord::HalfCarry, bit_set!(lower_nybble_diff, 7));

        // If result is zero, set zero flag
        self.status.set(StatusWord::Zero, sum == 0);

        // If result is negative, set negative flag
        self.status.set(StatusWord::Negative, bit_set!(sum, 7));

        // Write back to correct register / address
        match instr_data.addr_mode {
            // Write back to arg_addr
            AddressingMode::DPImmediate |
            AddressingMode::DPtoDP |
            AddressingMode::XIndirectYIndirect => self.mem_write(instr_data.arg_addr.unwrap(), sum),

            _ => self.acc = sum,
        }
    }
}