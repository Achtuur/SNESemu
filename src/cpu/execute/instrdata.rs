use crate::cpu::{instructions::{AddressingMode, instructions::Instruction}, Cpu};


/// Combines `pointer_bank ($BB)`, `pointer_high ($HH)`, `pointer_low ($LL)` into `$BBHHLL`
/// 
/// Or combines `$bank ($BB)` and `$hhll ($HHLL)` into `$BBHHLL`
macro_rules! combine_address {
    ($bank: expr, $hh: expr, $ll: expr) => {{
        (($bank as u32) << 16) | (($hh as u32) << 8) | ($ll as u32)
    }};

    ($bank: expr, $hhll: expr) => {
        combine_address!($bank, ($hhll >> 8) as u8, $hhll as u8)
    }
}

/// Container for holding data that an instruction can take as input
pub struct InstrData {
    /// Address of first byte needed for instruction
    pub low_addr: u32,
    /// low_addr + 1 that wraps properly based on addressing mode
    pub high_addr: u32,
    /// low_addr + 2 that wraps properly based on addressing mode
    pub long_addr: u32,
    /// Data inside low_addr + 1
    pub data: u16,
}

impl InstrData {
    pub fn new(data: u16, low_addr: u32, high_addr: u32, long_addr: u32) -> Self {
        InstrData { low_addr, high_addr, long_addr, data }
    }
}

impl Cpu {
    /// Returns `InstrData` struct based on instruction addressing mode
    /// 
    /// Source: [6502.org](http://www.6502.org/tutorials/65c816opcodes.html)
    pub fn get_instruction_data(&mut self, instr: &Instruction) -> InstrData {
        use AddressingMode::*;
        
        let (arg0, arg1, arg2) = match instr.get_length() {
            1 => (0, 0, 0), // Implied
            2 => {
                let arg0 = self.mem_read(self.get_pc_addr());
                (arg0, 0, 0)
            },
            3 => {
                let arg0 = self.mem_read(self.get_pc_addr());
                let arg1 = self.mem_read(self.get_pc_addr());
                (arg0, arg1, 0)
            },
            4 => {
                let arg0 = self.mem_read(self.get_pc_addr());
                let arg1 = self.mem_read(self.get_pc_addr());
                let arg2 = self.mem_read(self.get_pc_addr());
                (arg0, arg1, arg2)
            }
            _ => unreachable!("No instruction has more than 3 arguments")
        };
        
        match instr.get_addressing_mode() {
            Absolute => self.get_absolute_data(arg0, arg1),
            AbsoluteX => self.get_absolutex_data(arg0, arg1),
            AbsoluteY => self.get_absolutey_data(arg0, arg1),
            AbsoluteIndirect => self.get_absolute_indirect_data(arg0, arg1),
            AbsoluteIndirectX => self.get_absolute_indirectx_data(arg0, arg1),
            AbsoluteIndirectLong => self.get_absolute_indirect_long_data(arg0, arg1),
            Direct => self.get_direct_data(arg0),
            DirectX => self.get_directx_data(arg0),
            DirectY => self.get_directy_data(arg0),
            Implied => InstrData::new(0, 0, 0, 0),
            Immediate => self.get_immediate(arg0),
            ImmediateLong => self.get_immediate_long(arg0, arg1),
            Indirect => self.get_indirect_data(arg0),
            IndirectX => self.get_indirectx_data(arg0),
            IndirectY => self.get_indirecty_data(arg0),
            IndirectLong => self.get_indirect_long_data(arg0),
            IndirectLongY => self.get_indirecty_long_data(arg0),
            Long => self.get_long_data(arg0, arg1, arg2),
            LongX => self.get_longx_data(arg0, arg1, arg2),
            Relative => self.get_relative_data(arg0),
            RelativeLong => self.get_relative_long_data(arg0, arg1),
            StackRelative => self.get_stackrelative_data(arg0),
            StackRelativeY => self.get_stackrelativey_data(arg0),
            Move => self.get_move_data(arg0, arg1),
        }
    }
    
    /// $OP $LL $HH
    /// 
    /// # Returns
    /// * data: value in $rrHHLL
    /// * low_addr: $rrHHLL
    /// * high_addr: $rrHHLL + 1
    fn get_absolute_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let low_addr = combine_address!(self.pbr, arg1, arg0);
        let high_addr = low_addr.wrapping_add(1);
        
        let data = self.mem_read_long(low_addr, high_addr);
        
        InstrData::new(data, low_addr, high_addr, 0)
    }
    
    /// $OP $LL $HH
    /// 
    /// # Returns
    /// * data: value in $rrHHLL
    /// * low_addr: $rrHHLL
    /// * high_addr: $rrHHLL + 1 + X
    fn get_absolutex_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let low_addr = combine_address!(self.pbr, arg1, arg0);
        let low_addr = low_addr.wrapping_add(self.x as u32);
        let high_addr = low_addr.wrapping_add(1);
        
        let data = self.mem_read_long(low_addr, high_addr);
        
        InstrData::new(data, low_addr, high_addr, 0)
    }
    
    /// $OP $LL $HH
    /// 
    /// # Returns
    /// * data: value in $rrHHLL
    /// * low_addr: $rrHHLL
    /// * high_addr: $rrHHLL + 1 + Y
    fn get_absolutey_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let low_addr = combine_address!(self.pbr, arg1, arg0);
        let low_addr = low_addr.wrapping_add(self.y as u32);
        let high_addr = low_addr.wrapping_add(1);
        
        let data = self.mem_read_long(low_addr, high_addr);
        
        InstrData::new(data, low_addr, high_addr, 0)
    }
    
    /// $OP $LL $HH
    /// 
    /// reads `$ll = $00HHLL` and `$hh = $00HHLL + 1`
    /// 
    /// # Returns
    /// * low_addr: `$rrhhll`
    fn get_absolute_indirect_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let low_pointer = combine_address!(0, arg1, arg0);
        let high_pointer = low_pointer.wrapping_add(1);
        
        let hhll = self.mem_read_long(low_pointer, high_pointer) as u32;
        let dest_addr = combine_address!(self.pbr, hhll);
        let dest_addr = ((self.pbr as u32) << 16) | hhll;
        
        InstrData::new(0, dest_addr, 0, 0)
        
    }
    
    /// $OP $LL $HH
    /// 
    /// reads `$ll = $00HHLL + X` and `$hh = $00HHLL + 1 + X`
    /// 
    /// # Returns
    /// * low_addr: `$rrhhll`
    fn get_absolute_indirectx_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let low_pointer = combine_address!(0, arg1, arg0);
        let high_pointer = low_pointer.wrapping_add(1).wrapping_add(self.x as u32);
        
        let hhll = self.mem_read_long(low_pointer, high_pointer) as u32;
        let dest_addr = ((self.pbr as u32) << 16) | hhll;
        
        InstrData::new(0, dest_addr, 0, 0)
        
    }
    
    /// $OP $LL $HH
    /// 
    /// reads `$ll = $00HHLL`, `$mm = $00HHLL + 1` and `$hh = $00HHLL + 2`
    /// 
    /// # Returns
    /// * low_addr: `$hhmmll`
    fn get_absolute_indirect_long_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let low_pointer = ((arg1 as u32) << 8) | (arg0 as u32);
        let mid_pointer = low_pointer.wrapping_add(1);
        let high_pointer = low_pointer.wrapping_add(2);
        
        let mmll = self.mem_read_long(low_pointer, high_pointer) as u32;
        let hh = self.mem_read(high_pointer) as u32;
        let dest_addr = (hh << 16) | mmll;
        
        InstrData::new(0, dest_addr, 0, 0)
    }
    
    /// $OP $LL
    /// 
    /// reads `$dl = D + $LL` and `$dh = D + $LL + 1`
    /// 
    /// `$dl` and `$dh` wrap around 16 bytes, meaning that if `D = $FF00` and `$LL = $FF` then low_addr = `$00FFFF` and high_addr = `$000000`
    /// 
    /// # Returns
    /// * data: value in `$dh` as high byte, value in `$dl` as low byte
    /// * low_addr: `$dl`
    /// * high_adr: `$dh`
    fn get_direct_data(&mut self, arg0: u8) -> InstrData {
        let low_addr = self.dp.wrapping_add(arg0 as u16);
        let high_addr = low_addr.wrapping_add(1);
        
        let data = self.mem_read_long(low_addr as u32, high_addr as u32);
        InstrData::new(data, low_addr as u32, high_addr as u32, 0)
    }
    
    /// $OP $LL
    /// 
    /// reads `$dl = D + $LL + X` and `$dh = D + $LL + 1 + X`
    /// 
    /// `$dl` and `$dh` wrap around 16 bytes, meaning that if `D = $FF00` and `$LL = $FF` then low_addr = `$00FFFF` and high_addr = `$000000`
    /// 
    /// # Returns
    /// * data: value in `$dh` as high byte, value in `$dl` as low byte
    /// * low_addr: `$dl`
    /// * high_adr: `$dh`
    fn get_directx_data(&mut self, arg0: u8) -> InstrData {
        let low_addr = self.dp.wrapping_add(arg0 as u16).wrapping_add(self.x);
        let high_addr = low_addr.wrapping_add(1);
        
        let data = self.mem_read_long(low_addr as u32, high_addr as u32);
        InstrData::new(data, low_addr as u32, high_addr as u32, 0)
    }
    
    /// $OP $LL
    /// 
    /// reads `$dl = D + $LL + Y` and `$dh = D + $LL + 1 + Y`
    /// 
    /// `$dl` and `$dh` wrap around 16 bytes, meaning that if `D = $FF00` and `$LL = $FF`
    ///  then low_addr = `$00FFFF` and high_addr = `$000000`
    /// 
    /// # Returns
    /// * data: value in `$dh` as high byte, value in `$dl` as low byte
    /// * low_addr: `$dl`
    /// * high_adr: `$dh`
    fn get_directy_data(&mut self, arg0: u8) -> InstrData {
        let low_addr = self.dp.wrapping_add(arg0 as u16).wrapping_add(self.y);
        let high_addr = low_addr.wrapping_add(1);
        
        let data = self.mem_read_long(low_addr as u32, high_addr as u32);
        InstrData::new(data, low_addr as u32, high_addr as u32, 0)
    }

    fn get_indirect_data(&mut self, arg0: u8) -> InstrData {
        let pointer_lo = self.dp.wrapping_add(arg0 as u16);
        let pointer_hi = pointer_lo.wrapping_add(1);

        let ll = self.mem_read(pointer_lo as u32);
        let hh = self.mem_read(pointer_hi as u32);

        let low_addr = combine_address!(self.pbr, hh, ll);
        let high_addr = low_addr.wrapping_add(1);

        let data = self.mem_read_long(low_addr, high_addr);
        InstrData::new(data, low_addr, high_addr, 0)
    }

    fn get_indirectx_data(&mut self, arg0: u8) -> InstrData {
        let pointer_lo = self.dp.wrapping_add(arg0 as u16).wrapping_add(self.x);
        let pointer_hi = pointer_lo.wrapping_add(1);

        let ll = self.mem_read(pointer_lo as u32);
        let hh = self.mem_read(pointer_hi as u32);

        let low_addr = combine_address!(self.pbr, hh, ll);
        let high_addr = low_addr.wrapping_add(1);

        let data = self.mem_read_long(low_addr, high_addr);
        InstrData::new(data, low_addr, high_addr, 0)
    }

    fn get_indirecty_data(&mut self, arg0: u8) -> InstrData {
        let pointer_lo = self.dp.wrapping_add(arg0 as u16);
        let pointer_hi = pointer_lo.wrapping_add(1);

        let ll = self.mem_read(pointer_lo as u32);
        let hh = self.mem_read(pointer_hi as u32);

        let low_addr = combine_address!(self.pbr, hh, ll).wrapping_add(self.y as u32);
        let high_addr = low_addr.wrapping_add(1);

        let data = self.mem_read_long(low_addr, high_addr);
        InstrData::new(data, low_addr, high_addr, 0)
    }

    fn get_indirect_long_data(&mut self, arg0: u8) -> InstrData {
        let pointer_lo = self.dp.wrapping_add(arg0 as u16);
        let pointer_mid = pointer_lo.wrapping_add(1);
        let pointer_hi = pointer_lo.wrapping_add(2);

        let ll = self.mem_read(pointer_lo as u32);
        let mm = self.mem_read(pointer_mid as u32);
        let hh = self.mem_read(pointer_hi as u32);

        let low_addr = combine_address!(hh, mm, ll);
        let high_addr = low_addr.wrapping_add(1);

        let data = self.mem_read_long(low_addr, high_addr);
        InstrData::new(data, low_addr, high_addr, 0)
    }

    fn get_indirecty_long_data(&mut self, arg0: u8) -> InstrData {
        let pointer_lo = self.dp.wrapping_add(arg0 as u16);
        let pointer_mid = pointer_lo.wrapping_add(1);
        let pointer_hi = pointer_lo.wrapping_add(2);

        let ll = self.mem_read(pointer_lo as u32);
        let mm = self.mem_read(pointer_mid as u32);
        let hh = self.mem_read(pointer_hi as u32);

        let low_addr = combine_address!(hh, mm, ll).wrapping_add(self.y as u32);
        let high_addr = low_addr.wrapping_add(1);

        let data = self.mem_read_long(low_addr, high_addr);
        InstrData::new(data, low_addr, high_addr, 0)
    }

    fn get_immediate(&mut self, arg0: u8) -> InstrData {
        InstrData::new(arg0 as u16, 0, 0, 0)
    }

    fn get_immediate_long(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let data = (arg1 as u16) << 8 | (arg0 as u16);
        InstrData::new(data, 0, 0, 0)
    }

    fn get_long_data(&mut self, arg0: u8, arg1: u8, arg2: u8) -> InstrData {
        let low_addr = combine_address!(arg2, arg1, arg0);
        let high_addr = low_addr.wrapping_add(1);
        let data = self.mem_read_long(low_addr, high_addr);
        InstrData::new(data, low_addr, high_addr, 0)
    }

    fn get_longx_data(&mut self, arg0: u8, arg1: u8, arg2: u8) -> InstrData {
        let low_addr = combine_address!(arg2, arg1, arg0).wrapping_add(self.x as u32);
        let high_addr = low_addr.wrapping_add(1);
        let data = self.mem_read_long(low_addr, high_addr);
        InstrData::new(data, low_addr, high_addr, 0)
    }

    fn get_relative_data(&mut self, arg0: u8) -> InstrData {
        let target_addr = if (arg0 >> 7) == 1 {
            self.pc.wrapping_add(2).wrapping_add(arg0 as u16)
        } else {
            self.pc.wrapping_sub(254).wrapping_add(arg0 as u16)
        };

        let target_addr = combine_address!(self.pbr, target_addr);
        InstrData::new(0, target_addr, 0, 0)
    }

    fn get_relative_long_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let hhll = combine_address!(0, arg1, arg0) as u16;
        let target_addr = self.pc.wrapping_add(3).wrapping_add(hhll);

        let target_addr = combine_address!(self.pbr, target_addr);
        InstrData::new(0, target_addr, 0, 0)
    }

    /// $OP $TT $SS
    /// 
    /// * Source address = $SSXXXX where $XXXX is value of X register
    /// * Destination address = $TTYYYY where $YYYY is value of Y register
    /// 
    /// # Returns
    /// * `low_addr`: source address
    /// * `high_addr`: destination address
    fn get_move_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let dest = combine_address!(arg0, (self.y >> 8) as u8, self.y as u8);
        let source = combine_address!(arg1, (self.x >> 8) as u8, self.x as u8);

        InstrData::new(0, source, dest, 0)
    }

    fn get_stackrelative_data(&mut self, arg0: u8) -> InstrData {
        let low_addr: u16 = self.sp.wrapping_add(arg0 as u16);
        let high_addr: u16 = low_addr.wrapping_add(1);

        let data = self.mem_read_long(low_addr as u32, high_addr as u32);
        InstrData::new(data, low_addr as u32, high_addr as u32, 0)
    }
    
    fn get_stackrelativey_data(&mut self, arg0: u8) -> InstrData {
        let pointer_lo: u16 = self.sp.wrapping_add(arg0 as u16);
        let pointer_hi: u16 = pointer_lo.wrapping_add(1);

        let ll = self.mem_read(pointer_lo as u32);
        let hh = self.mem_read(pointer_hi as u32);

        let low_addr = combine_address!(self.pbr, hh, ll).wrapping_add(self.y as u32);
        let high_addr = low_addr.wrapping_add(1);

        let data = self.mem_read_long(low_addr, high_addr);
        InstrData::new(data, low_addr, high_addr, 0)
    }
}

