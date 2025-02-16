pub mod instrdata;

use crate::to_word;

use super::{instructions::Instruction, processorstatusflag::ProcessorStatusFlags, Cpu, CpuError};

impl Cpu<'_> {
    pub fn execute_nmi(&mut self) -> Result<(), CpuError> {
        self.exe_interrupt(0x00FFEA);
        Ok(())
    }

    pub fn execute_irq(&mut self) -> Result<(), CpuError> {
        self.exe_interrupt(0x00FFEE);
        Ok(())
    }

    /// Executes `instr`, increments PC and adds to cycle wait time
    pub fn execute_op(&mut self, op: u8) -> Result<(), CpuError> {
        let instr = Instruction::from_op(op);
        self.execute_instr(&instr);
        self.wait_cycles = instr.cycle_count();
        Ok(())
    }

    fn execute_instr(&mut self, instr: &Instruction) {
        use Instruction as I;
        let instr_data = self.get_instruction_data(instr);
        let data = instr_data.data;
        let low_addr = instr_data.low_addr;
        let high_addr = instr_data.high_addr;

        tracing::debug!(
            "[A: {:#04X}, X: {:#04X}, Y: {:#04X}]",
            self.acc(), self.x(), self.y()
        );

        tracing::debug!(
            "[PC: {:#2X}]: Executing {:?} ({:#02X}, {:#06X})",
            self.get_pc_addr(), instr, data, low_addr
        );

        match instr {
            I::Adc(_) => self.exe_adc(data),

            I::And(_) => {
                self.exe_and(data)
            }

            I::Asl(_) => self.exe_asl(low_addr, high_addr),

            I::Asla => self.exe_asla(),
            I::Blt => self.exe_bcc(low_addr),
            I::Bge => self.exe_bcs(low_addr),
            I::Beq => self.exe_beq(low_addr),

            I::Bit(_) => self.exe_bit(data),

            I::Bmi => self.exe_bmi(low_addr),
            I::Bne => self.exe_bne(low_addr),
            I::Bpl => self.exe_bpl(low_addr),
            I::Bra => self.exe_bra(low_addr),
            I::Brk => self.exe_brk(data),
            I::Brl => self.exe_brl(low_addr),
            I::Bvc => self.exe_bvc(low_addr),
            I::Bvs => self.exe_bvs(low_addr),
            I::Clc => self.exe_clc(),
            I::Cld => self.exe_cld(),
            I::Cli => self.exe_clc(),
            I::Clv => self.exe_clv(),

            I::Cmp(_) => self.exe_cmp(data),

            I::Cop => self.exe_cop(data),
            I::Cpx(_) => self.exe_cpx(data),
            I::Cpy(_) => self.exe_cpy(data),

            I::Dea => self.exe_dea(),

            I::Dec(_) => self.exe_dec(low_addr, high_addr),

            I::Dex => self.exe_dex(),
            I::Dey => self.exe_dey(),

            I::Eor(_) => self.exe_eor(data),

            I::Inc(_) => self.exe_inc(low_addr, high_addr),

            I::Ina => self.exe_ina(),
            I::Inx => self.exe_inx(),
            I::Iny => self.exe_iny(),

            I::Jmp(_) => self.exe_jmp(low_addr),

            I::Jml(_) => self.exe_jml(low_addr),

            I::Jsl => self.exe_jsl(low_addr),

            I::Jsr(_) => self.exe_jsr(low_addr),

            I::Lda(_) => self.exe_lda(data),

            I::Ldx(_) => self.exe_ldx(data),
            I::Ldy(_) => self.exe_ldy(data),

            I::Lsra => self.exe_lsra(),
            I::Lsr(_) => self.exe_lsr(low_addr, high_addr),

            I::Mvn => self.exe_mvn(low_addr, high_addr),
            I::Mvp => self.exe_mvp(low_addr, high_addr),

            I::Nop => self.exe_nop(data),

            I::Ora(_) => self.exe_ora(data),

            I::Pea => self.exe_pea(data),
            I::Pei => self.exe_pei(data),
            I::Per => self.exe_per(data),
            I::Pha => self.exe_pha(data),
            I::Phb => self.exe_phb(data),
            I::Phd => self.exe_phd(data),
            I::Phk => self.exe_phk(data),
            I::Php => self.exe_php(data),
            I::Phx => self.exe_phx(data),
            I::Phy => self.exe_phy(data),
            I::Pla => self.exe_pla(data),
            I::Plb => self.exe_plb(data),
            I::Pld => self.exe_pld(data),
            I::Plp => self.exe_plp(data),
            I::Plx => self.exe_plx(data),
            I::Ply => self.exe_ply(data),

            I::Rep => self.exe_rep(data as u8),

            I::Rola => self.exe_rola(),
            I::Rol(_) => self.exe_rol(low_addr, high_addr),

            I::Rora => self.exe_rora(),
            I::Ror(_) => self.exe_ror(low_addr, high_addr),

            I::Rti => self.exe_rti(data),
            I::Rtl => self.exe_rtl(),
            I::Rts => self.exe_rts(),

            I::Sbc(_) => self.exe_sbc(data),

            I::Sec => self.exe_sec(),
            I::Sed => self.exe_sed(),
            I::Sei => self.exe_sei(),
            I::Sep => self.exe_sep(data as u8),

            I::Sta(_) => self.exe_sta(low_addr, high_addr),

            I::Stp => self.exe_stp(data),

            I::Stx(_) => self.exe_stx(low_addr, high_addr),
            I::Sty(_) => self.exe_sty(low_addr, high_addr),
            I::Stz(_) => self.exe_stz(low_addr, high_addr),

            I::Tax => self.exe_tax(),
            I::Tay => self.exe_tay(),
            I::Tcd => self.exe_tcd(),
            I::Tcs => self.exe_tcs(),
            I::Tdc => self.exe_tdc(),

            I::Trb(_) => self.exe_trb(low_addr, high_addr),
            I::Tsb(_) => self.exe_tsb(low_addr, high_addr),

            I::Tsc => self.exe_tsc(),
            I::Tsx => self.exe_tsx(),
            I::Txa => self.exe_txa(),
            I::Txs => self.exe_txs(),
            I::Txy => self.exe_txy(),
            I::Tya => self.exe_tya(),
            I::Tyx => self.exe_tyx(),

            I::Wai => self.exe_wai(data),
            I::Wdm => self.exe_wdm(data),
            I::Xba => self.exe_xba(data),
            I::Xce => self.exe_xce(data),
        }
    }
}
