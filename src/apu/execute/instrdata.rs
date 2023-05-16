use crate::{to_word, nth_bit, apu::{instructions::{AddressingMode, instructions::Instruction}, SApu}};



pub struct InstrData {
    /// Address that is calculated from the arguments
    pub arg_addr: Option<u16>,
    /// Byte that is stored inside `arg_addr`
    pub addr_data: Option<u8>,
    /// Address for high byte of 16 bit operations
    pub word_addr: Option<u16>,
    /// Data contained in `to_word!(read(word_addr), read(arg_addr))`
    pub word_data: Option<u16>,

    /// Byte that is given by arguments if it represents an immediate
    /// 
    /// For XYrelative and dp_to_dp modes this holds the right hand side of the instruction
    pub imm_data: Option<u8>,
    /// Target address for jump and branch instructions
    pub target_addr: Option<u16>,
    /// Addressing mode of this data, to determine target register or address
    pub addr_mode: AddressingMode,
}

impl InstrData {
    pub fn new() -> InstrData {
        InstrData {
            arg_addr: None,
            addr_data: None,
            imm_data: None,
            target_addr: None,
            word_addr: None,
            word_data: None,
            addr_mode: AddressingMode::Implied,
        }
    }
}

impl SApu {
    pub fn get_instr_data(&mut self, instr: &Instruction) -> InstrData {

        let mut arg0: u8 = 0;
        let mut arg1: u8 = 0;

        let len = instr.get_length();

        if len > 0 {
            arg0 = self.mem_read(self.pc.wrapping_add(1));
        }

        if len > 1 {
            arg1 = self.mem_read(self.pc.wrapping_add(2));
        }

        let mut instr_data = match instr.get_addressing_mode() {
            AddressingMode::Immediate          => self.get_immediate_data(arg0),
            AddressingMode::DPImmediate        => self.get_dpimmediate_data(arg0, arg1),
            AddressingMode::Implied            => InstrData::new(),
            AddressingMode::DirectPage         => self.get_dp_data(arg0),
            AddressingMode::DirectPageX        => self.get_dpx_data(arg0),
            AddressingMode::DirectPageY        => self.get_dpy_data(arg0),
            AddressingMode::Absolute           => self.get_abs_data(arg0, arg1),
            AddressingMode::AbsoluteX          => self.get_absx_data(arg0, arg1),
            AddressingMode::AbsoluteY          => self.get_absy_data(arg0, arg1),
            AddressingMode::AbsoluteBit        => self.get_absbit_data(arg0, arg1),
            AddressingMode::Relative           => self.get_relative_data(arg0),
            AddressingMode::DPRelative         => self.get_dprelative_data(arg0, arg1),
            AddressingMode::DPXRelative        => self.get_dpxrelative_data(arg0, arg1),
            AddressingMode::XIndirect          => self.get_xindirect_data(),
            AddressingMode::IndirectX          => self.get_indirectx_data(arg0),
            AddressingMode::IndirectY          => self.get_indirecty_data(arg0),
            AddressingMode::DPtoDP             => self.get_dptodp_data(arg0, arg1),
            AddressingMode::XIndirectYIndirect => self.get_xindirect_yindirect_data(),
            AddressingMode::Word               => self.get_word_data(arg0),
            AddressingMode::SpecialPointer     => self.get_specialpointer_data(arg0),
            AddressingMode::IncrementX         => self.get_incrementX_data(),
        };

        instr_data.addr_mode = instr.get_addressing_mode();
        instr_data
    }

    fn get_immediate_data(&mut self, arg0: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        instr_data.imm_data = Some(arg0);
        instr_data
    }
    
    fn get_dpimmediate_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        let dp_addr = self.get_dp_address(arg0);
        instr_data.arg_addr = Some(dp_addr);
        instr_data.addr_data = Some(self.mem_read(dp_addr));
        instr_data.imm_data = Some(arg1);
        instr_data
    }

    fn get_dp_data(&mut self, arg0: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        let dp_addr = self.get_dp_address(arg0);
        instr_data.arg_addr = Some(dp_addr);
        instr_data.addr_data = Some(self.mem_read(dp_addr));
        instr_data
    }

    fn get_dpx_data(&mut self, arg0: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        let dp_addr = self.get_dp_address(arg0.wrapping_add(self.x));
        instr_data.arg_addr = Some(dp_addr);
        instr_data.addr_data = Some(self.mem_read(dp_addr));
        instr_data
    }

    fn get_dpy_data(&mut self, arg0: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        let dp_addr = self.get_dp_address(arg0.wrapping_add(self.y));
        instr_data.arg_addr = Some(dp_addr);
        instr_data.addr_data = Some(self.mem_read(dp_addr));
        instr_data
    }

    fn get_abs_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        let addr = to_word!(arg1, arg0);
        instr_data.arg_addr = Some(addr);
        instr_data.addr_data = Some(self.mem_read(addr));
        instr_data
    }

    fn get_absx_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        let addr = to_word!(arg1, arg0).wrapping_add(self.x as u16);
        instr_data.arg_addr = Some(addr);
        instr_data.addr_data = Some(self.mem_read(addr));
        instr_data
    }

    fn get_absy_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        let addr = to_word!(arg1, arg0).wrapping_add(self.y as u16);
        instr_data.arg_addr = Some(addr);
        instr_data.addr_data = Some(self.mem_read(addr));
        instr_data
    }

    fn get_absbit_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        // Gonna assume the memory is zero page
        let mut instr_data = InstrData::new();
        let dp_addr = self.get_dp_address(arg0);
        instr_data.arg_addr = Some(dp_addr);
        let addr_data =  self.mem_read(dp_addr);
        instr_data.addr_data = Some(nth_bit!(addr_data, arg1));
        instr_data
    }

    fn get_indirectx_data(&mut self, arg0: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        // Transform argument into address, indexed by X
        let arg_dp_addr = self.get_dp_address(arg0.wrapping_add(self.x));
        let arg_dp_data = self.mem_read(arg_dp_addr);
        let indirect_addr = self.get_dp_address(arg_dp_data);

        // Address & data used by instruction is given by indirect_addr
        instr_data.arg_addr = Some(indirect_addr);
        instr_data.addr_data = Some(self.mem_read(indirect_addr));
        instr_data
    }

    fn get_indirecty_data(&mut self, arg0: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        // Transform argument into address, indexed by X
        let arg_dp_addr = self.get_dp_address(arg0);
        let arg_dp_data = self.mem_read(arg_dp_addr);
        let indirect_addr = self.get_dp_address(arg_dp_data).wrapping_add(self.y as u16);
        
        // Address & data used by instruction is given by indirect_addr
        instr_data.arg_addr = Some(indirect_addr);
        instr_data.addr_data = Some(self.mem_read(indirect_addr));
        instr_data
    }


    /// `ADC A, (X)`, data is inside dp address that x points to
    fn get_xindirect_data(&mut self) -> InstrData {
        let mut instr_data = InstrData::new();
        let dp_addr = self.get_dp_address(self.x);
        instr_data.arg_addr = Some(dp_addr);
        instr_data.addr_data = Some(self.mem_read(dp_addr));
        instr_data
    }


    fn get_incrementX_data(&mut self) -> InstrData {
        let instr_data = self.get_xindirect_data();
        self.x = self.x.wrapping_add(1);
        instr_data
    }

    fn get_xindirect_yindirect_data(&mut self) -> InstrData {
        let mut instr_data = InstrData::new();
        let x_dp = self.get_dp_address(self.x);
        instr_data.arg_addr = Some(x_dp);
        instr_data.addr_data = Some(self.mem_read(x_dp));

        let y_dp = self.get_dp_address(self.y);
        instr_data.imm_data = Some(self.mem_read(y_dp));
        instr_data
    }

    fn get_dptodp_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        let arg0_dp = self.get_dp_address(arg0);
        instr_data.arg_addr = Some(arg0_dp);
        instr_data.addr_data = Some(self.mem_read(arg0_dp));

        let arg1_dp = self.get_dp_address(arg1);
        instr_data.imm_data = Some(self.mem_read(arg1_dp));
        instr_data
    }

    fn get_relative_data(&mut self, arg0: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        instr_data.target_addr = Some(self.pc.wrapping_add_signed(arg0 as i16));
        instr_data
    }

    fn get_dprelative_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        instr_data.target_addr = Some(self.pc.wrapping_add_signed(arg0 as i16));

        let dp_addr = self.get_dp_address(arg1);
        instr_data.arg_addr = Some(dp_addr);
        instr_data.addr_data = Some(self.mem_read(dp_addr));

        instr_data
    }


    fn get_dpxrelative_data(&mut self, arg0: u8, arg1: u8) -> InstrData {
        let mut instr_data = InstrData::new();

        let dp_addr = self.get_dp_address(arg0.wrapping_add(self.x));
        instr_data.arg_addr = Some(dp_addr);
        instr_data.addr_data = Some(self.mem_read(dp_addr));

        instr_data.target_addr = Some(self.pc.wrapping_add_signed(arg1 as i16));
        instr_data
    }

    fn get_word_data(&mut self, arg0: u8) -> InstrData {
        let mut instr_data = InstrData::new();
        
        let lb_addr = self.get_dp_address(arg0);
        let hb_addr = self.get_dp_address(arg0.wrapping_add(1));
        let low_byte = self.mem_read(lb_addr);
        let high_byte = self.mem_read(hb_addr);

        instr_data.arg_addr = Some(lb_addr);
        instr_data.word_addr = Some(hb_addr);
        instr_data.word_data = Some(to_word!(high_byte, low_byte));
        instr_data
    }

    fn get_specialpointer_data(&mut self, arg0: u8) -> InstrData {
        let mut instr_data = InstrData::new();

        let addr = to_word!(0xFF, arg0);
        instr_data.target_addr = Some(addr);
        instr_data
    }
}