pub mod instrdata;


use super::{instructions::instructions::Instruction, SCpu, CpuError, NMI_PENDING, IRQ_PENDING, statusflag::StatusFlags};
use super::{instructions::AddressingMode};
impl SCpu {

    pub fn execute_nmi(&mut self) -> Result<(), CpuError> {
        self.exe_interrupt(0x00FFEA);
        *NMI_PENDING.lock().unwrap() = false;
        Ok(())
    }

    pub fn execute_irq(&mut self) -> Result<(), CpuError> {
        self.exe_interrupt(0x00FFEE);
        *IRQ_PENDING.lock().unwrap() = false;
        Ok(())
    }

    /// Executes `instr`, increments PC and adds to cycle wait time
    pub fn execute_instruction(&mut self, instr: Instruction) -> Result<(), CpuError> {
        self.execute_op(&instr);

        self.get_cycle_time(&instr);

        self.pc = self.pc.wrapping_add(self.get_instr_length(&instr) as u16);

        Ok(())
    }

    fn execute_op(&mut self, instr: &Instruction) {
        use Instruction::*;
        let instr_data = self.get_instruction_data(&instr);
        let data = instr_data.data;
        let low_addr = instr_data.low_addr;
        let high_addr = instr_data.high_addr;
        let jump_addr = instr_data.jump_addr;

        println!("{0:?} {1:02X} {2:02X} {3:02X} {4:06X}", instr, data, low_addr, high_addr, jump_addr);

        match instr {
            ADC(_) => self.exe_adc(data),
            AND(_) => self.exe_and(data),
            ASLA(_) => self.exe_asla(),
            ASL(_) => self.exe_asl(low_addr, high_addr),
            BLT(_) => self.exe_bcc(low_addr),
            BGE(_) => self.exe_bcs(low_addr),
            BEQ(_) => self.exe_beq(low_addr),
            BIT(_) => self.exe_bit(data),
            BMI(_) => self.exe_bmi(low_addr),
            BNE(_) => self.exe_bne(low_addr),
            BPL(_) => self.exe_bpl(low_addr),
            BRA(_) => self.exe_bra(low_addr),
            BRK(_) => self.exe_brk(data),
            BRL(_) => self.exe_brl(low_addr),
            BVC(_) => self.exe_bvc(low_addr),
            BVS(_) => self.exe_bvs(low_addr),
            CLC(_) => self.exe_clc(),
            CLD(_) => self.exe_cld(),
            CLI(_) => self.exe_clc(),
            CLV(_) => self.exe_clv(),
            CMP(_) => self.exe_cmp(data),
            COP(_) => self.exe_cop(data),
            CPX(_) => self.exe_cpx(data),
            CPY(_) => self.exe_cpy(data),
            DEA(_) => self.exe_dea(),
            DEC(_) => self.exe_dec(low_addr, high_addr),
            DEX(_) => self.exe_dex(),
            DEY(_) => self.exe_dey(),
            EOR(_) => self.exe_eor(data),
            INC(_) => self.exe_inc(low_addr, high_addr),
            INA(_) => self.exe_ina(),
            INX(_) => self.exe_inx(),
            INY(_) => self.exe_iny(),
            JMP(_) => self.exe_jmp(jump_addr),
            JML(_) => self.exe_jml(jump_addr),
            JSL(_) => self.exe_jsl(jump_addr),
            JSR(_) => self.exe_jsr(jump_addr),
            LDA(_) => self.exe_lda(data),
            LDX(_) => self.exe_ldx(data),
            LDY(_) => self.exe_ldy(data),
            LSRA(_) => self.exe_lsra(),
            LSR(_) => self.exe_lsr(low_addr, high_addr),
            MVN(_) => self.exe_mvn(low_addr, high_addr),
            MVP(_) => self.exe_mvp(low_addr, high_addr),
            NOP(_) => self.exe_nop(data),
            ORA(_) => self.exe_ora(data),
            PEA(_) => self.exe_pea(data),
            PEI(_) => self.exe_pei(data),
            PER(_) => self.exe_per(data),
            PHA(_) => self.exe_pha(data),
            PHB(_) => self.exe_phb(data),
            PHD(_) => self.exe_phd(data),
            PHK(_) => self.exe_phk(data),
            PHP(_) => self.exe_php(data),
            PHX(_) => self.exe_phx(data),
            PHY(_) => self.exe_phy(data),
            PLA(_) => self.exe_pla(data),
            PLB(_) => self.exe_plb(data),
            PLD(_) => self.exe_pld(data),
            PLP(_) => self.exe_plp(data),
            PLX(_) => self.exe_plx(data),
            PLY(_) => self.exe_ply(data),
            REP(_) => self.exe_rep(data as u8),
            ROLA(_) => self.exe_rola(),
            ROL(_)  => self.exe_rol(low_addr, high_addr),
            RORA(_) => self.exe_rora(),
            ROR(_)  => self.exe_ror(low_addr, high_addr),
            RTI(_) => self.exe_rti(data),
            RTL(_) => self.exe_rtl(),
            RTS(_) => self.exe_rts(),
            SBC(_) => self.exe_sbc(data),
            SEC(_) => self.exe_sec(),
            SED(_) => self.exe_sed(),
            SEI(_) => self.exe_sei(),
            SEP(_) => self.exe_sep(data as u8),
            STA(_) => self.exe_sta(low_addr, high_addr),
            STP(_) => self.exe_stp(data),
            STX(_) => self.exe_stx(low_addr, high_addr),
            STY(_) => self.exe_sty(low_addr, high_addr),
            STZ(_) => self.exe_stz(low_addr, high_addr),
            TAX(_) => self.exe_tax(),
            TAY(_) => self.exe_tay(),
            TCD(_) => self.exe_tcd(),
            TCS(_) => self.exe_tcs(),
            TDC(_) => self.exe_tdc(),
            TRB(_) => self.exe_trb(low_addr, high_addr),
            TSB(_) => self.exe_tsb(low_addr, high_addr),
            TSC(_) => self.exe_tsc(),
            TSX(_) => self.exe_tsx(),
            TXA(_) => self.exe_txa(),
            TXS(_) => self.exe_txs(),
            TXY(_) => self.exe_txy(),
            TYA(_) => self.exe_tya(),
            TYX(_) => self.exe_tyx(),
            WAI(_) => self.exe_wai(data),
            WDM(_) => self.exe_wdm(data),
            XBA(_) => self.exe_xba(data),
            XCE(_) => self.exe_xce(data),
        }
    }

    /// Get cycle time of instruction, subtracts/adds cycles when certain flags are on
    fn get_cycle_time(&mut self, instr: &Instruction) {
        use Instruction::*;
        use AddressingMode::*;
        self.wait_cycles = instr.get_cycle_time();

        // Add accumulator
        match instr {
            // Add !(accumulator flag) * 2
            ASL(Direct) | ASL(DirectX) | ASL(Absolute) | ASL(AbsoluteX) |
            DEC(Direct) | DEC(DirectX) | DEC(Absolute) | DEC(AbsoluteX) |
            INC(Direct) | INC(DirectX) | INC(Absolute) | INC(AbsoluteX) |
            LSR(Direct) | LSR(DirectX) | LSR(Absolute) | LSR(AbsoluteX) |
            ROL(Direct) | ROL(DirectX) | ROL(Absolute) | ROL(AbsoluteX) |
            ROR(Direct) | ROR(DirectX) | ROR(Absolute) | ROR(AbsoluteX) |
            TRB(Direct) | TRB(Absolute) | 
            TSB(Direct) | TSB(Absolute) => {
                // Add 2 cycles if acc flag is 0
                self.wait_cycles += 2 * (1 - self.status.accflag_as_u8() as usize)
            },

            // Add !(accumulator flag) * 1
            ADC(_) | AND(_) | BIT(_) | CMP(_) | 
            EOR(_) | LDA(_) | ORA(_) | SBC(_) | 
            PHA(_) | PLA(_) | STA(_) | STZ(_) => {
                // Add 1 cycle if acc flag is 0
                self.wait_cycles += 1 - self.status.accflag_as_u8() as usize;
            }
            _ => {},
        }

        // Add xy reg
        match instr {

            LDX(AbsoluteY) | LDY(AbsoluteX) => {
                self.wait_cycles += 2* (1 - self.status.xyflag_as_u8() as usize);
            }

            ADC(IndirectY) | ADC(AbsoluteY) | ADC(AbsoluteX) | 
            AND(IndirectY) | AND(AbsoluteY) | AND(AbsoluteX) |
            CMP(IndirectY) | CMP(AbsoluteY) | CMP(AbsoluteX) |
            EOR(IndirectY) | EOR(AbsoluteY) | EOR(AbsoluteX) |
            LDA(IndirectY) | LDA(AbsoluteY) | LDA(AbsoluteX) |
            ORA(IndirectY) | ORA(AbsoluteY) | ORA(AbsoluteX) |
            SBC(IndirectY) | SBC(AbsoluteY) | SBC(AbsoluteX) |
            BIT(AbsoluteX)  => {
                self.wait_cycles += 1 - self.status.xyflag_as_u8() as usize;
            }

            _ => {},
        }

        // Add if Dl == 0
        match instr.get_addressing_mode() {
            Direct | DirectX | DirectY |
            Indirect | IndirectX | IndirectY |
            IndirectLong | IndirectLongY => {
                if self.dp != 0 {
                    self.wait_cycles += 1;
                }
            }
            _ => {},
        }

    }

    /// Get length in bytes of instruction, subtracts 1 if accumulator or xyreg flags are set for certain instructions
    fn get_instr_length(&self, instr: &Instruction) -> usize {
        use Instruction::*;
        use AddressingMode::*;
        let len = instr.get_length();
        // sub accumulator or xy register
        match instr {
            ADC(Immediate) | AND(Immediate) | BIT(Immediate) | 
            CMP(Immediate) | EOR(Immediate) | LDA(Immediate) | 
            ORA(Immediate) | SBC(Immediate) => {
                // Add 1 if accumulator flag = 0
                len + (1 - self.status.accflag_as_u8() as usize)
            },
            LDX(Immediate) | LDY(Immediate) | CPX(Immediate) | CPY(Immediate) => {
                // Add 1 if xy flag = 0
                len + (1 - self.status.xyflag_as_u8() as usize)
            }
            
            _ => len
        }
    }
}
