pub mod instrdata;


use super::{instructions::instructions::Instruction, Cpu, CpuError};

impl Cpu {

    pub fn execute_nmi(&mut self) -> Result<(), CpuError> {
        self.exe_interrupt(0x00FFEA);
        Ok(())
    }

    pub fn execute_irq(&mut self) -> Result<(), CpuError> {
        self.exe_interrupt(0x00FFEE);
        Ok(())
    }

    /// Executes `instr`, increments PC and adds to cycle wait time
    pub fn execute_instruction(&mut self, instr: Instruction) -> Result<(), CpuError> {
        self.execute_op(&instr);

        self.wait_cycles = instr.get_cycle_time();

        self.pc = self.pc.wrapping_add(instr.get_length() as u16);

        Ok(())
    }

    fn execute_op(&mut self, instr: &Instruction) {
        use Instruction::*;
        let instr_data = self.get_instruction_data(&instr);
        let data = instr_data.data;
        let low_addr = instr_data.low_addr;
        let high_addr = instr_data.high_addr;

        match instr {
            AdcAbs | AdcAbsLong | AdcAbsX | AdcAbsXLong | AdcAbsY | AdcDP |
            AdcDPI | AdcDPIX | AdcDPIY | AdcDPIYLong | AdcDPLong | AdcDPX |
            AdcImm | AdcSR | AdcSRY => self.exe_adc(data),

            AndAbs | AndAbsLong | AndAbsX | AndAbsXLong | AndAbsY | AndDP |
            AndDPI | AndDPIX | AndDPIY | AndDPIYLong | AndDPLong | AndDPX |
            AndImm | AndSR | AndSRY => self.exe_and(data),

            AslAbs | AslDP | AslDPX | AslAbsX => self.exe_asl(low_addr, high_addr),

            Asl => self.exe_asla(),
            Blt => self.exe_bcc(low_addr),
            Bge => self.exe_bcs(low_addr),
            Beq => self.exe_beq(low_addr),

            BitAbs | BitAbsX | BitDP | BitDPX | BitImm => self.exe_bit(data),

            Bmi => self.exe_bmi(low_addr),
            Bne => self.exe_bne(low_addr),
            Bpl => self.exe_bpl(low_addr),
            Bra => self.exe_bra(low_addr),
            Brk => self.exe_brk(data),
            Brl => self.exe_brl(low_addr),
            Bvc => self.exe_bvc(low_addr),
            Bvs => self.exe_bvs(low_addr),
            Clc => self.exe_clc(),
            Cld => self.exe_cld(),
            Cli => self.exe_clc(),
            Clv => self.exe_clv(),

            CmpAbs | CmpAbsLong | CmpAbsX | CmpAbsXLong | CmpAbsY | CmpDP |
            CmpDPI | CmpDPIX | CmpDPIY | CmpDPIYLong | CmpDPLong | CmpDPX |
            CmpImm | CmpSR | CmpSRY => self.exe_cmp(data),

            Cop => self.exe_cop(data),
            CpxAbs | CpxDP | CpxImm => self.exe_cpx(data),
            CpyAbs | CpyDP | CpyImm => self.exe_cpy(data),

            Dea => self.exe_dea(),

            DecAbs | DecAbsX | DecDP | DecDPX => self.exe_dec(low_addr, high_addr),

            Dex => self.exe_dex(),
            Dey => self.exe_dey(),

            EorAbs | EorAbsLong | EorAbsX | EorAbsXLong | EorAbsY | EorDP |
            EorDPI | EorDPIX | EorDPIY | EorDPIYLong | EorDPLong | EorDPX |
            EorImm | EorSR | EorSRY => self.exe_eor(data),

            IncAbs | IncAbsX | IncDP | IncDPX => self.exe_inc(low_addr, high_addr),

            Ina => self.exe_ina(),
            Inx => self.exe_inx(),
            Iny => self.exe_iny(),

            JmpAbs | JmpAbsIX | JmpIndirect => self.exe_jmp(low_addr),

            Jml | JmlAbsLong => self.exe_jml(low_addr),

            JslAbsLong => self.exe_jsl(low_addr),

            JsrAbs | JsrAbsIX => self.exe_jsr(low_addr),

            LdaAbs | LdaAbsLong | LdaAbsX | LdaAbsXLong | LdaAbsY | LdaDP |
            LdaDPI | LdaDPIX | LdaDPIY | LdaDPIYLong | LdaDPLong | LdaDPX |
            LdaImm | LdaSR | LdaSRY => self.exe_lda(low_addr, high_addr),

            LdxAbs | LdxAbsY | LdxDP | LdxDPY | LdxImm => self.exe_ldx(low_addr, high_addr),
            LdyAbs | LdyAbsX | LdyDP | LdyDPX | LdyImm => self.exe_ldy(low_addr, high_addr),

            Lsr => self.exe_lsra(),
            LsrAbs | LsrAbsX | LsrDP | LsrDPX => self.exe_lsr(low_addr, high_addr),

            Mvn => self.exe_mvn(low_addr, high_addr),
            Mvp => self.exe_mvp(low_addr, high_addr),

            Nop => self.exe_nop(data),

            OraAbs | OraAbsLong | OraAbsX | OraAbsXLong | OraAbsY | OraDP |
            OraDPI | OraDPIX | OraDPIY | OraDPIYLong | OraDPLong | OraDPX |
            OraImm | OraSR | OraSRY => self.exe_ora(data),

            Pea => self.exe_pea(data),
            Pei => self.exe_pei(data),
            Per => self.exe_per(data),
            Pha => self.exe_pha(data),
            Phb => self.exe_phb(data),
            Phd => self.exe_phd(data),
            Phk => self.exe_phk(data),
            Php => self.exe_php(data),
            Phx => self.exe_phx(data),
            Phy => self.exe_phy(data),
            Pla => self.exe_pla(data),
            Plb => self.exe_plb(data),
            Pld => self.exe_pld(data),
            Plp => self.exe_plp(data),
            Plx => self.exe_plx(data),
            Ply => self.exe_ply(data),

            RepImm => self.exe_rep(data as u8),

            Rol => self.exe_rola(),
            RolAbs | RolAbsX | RolDP | RolDPX  => self.exe_rol(low_addr, high_addr),

            Ror => self.exe_rora(),
            RorAbs | RorAbsX | RorDP | RorDPX  => self.exe_ror(low_addr, high_addr),

            Rti => self.exe_rti(data),
            Rtl => self.exe_rtl(),
            Rts => self.exe_rts(),

            SbcAbs | SbcAbsLong | SbcAbsX | SbcAbsXLong | SbcAbsY | SbcDP |
            SbcDPI | SbcDPIX | SbcDPIY | SbcDPIYLong | SbcDPLong | SbcDPX |
            SbcImm | SbcSR | SbcSRY => self.exe_sbc(data),

            Sec => self.exe_sec(),
            Sed => self.exe_sed(),
            Sei => self.exe_sei(),
            SepImm => self.exe_sep(data as u8),
            
            StaAbs | StaAbsLong | StaAbsX | StaAbsXLong | StaAbsY | StaDP |
            StaDPI | StaDPIX | StaDPIY | StaDPIYLong | StaDPLong | StaDPX |
            StaSR | StaSRY => self.exe_sta(low_addr, high_addr),

            Stp => self.exe_stp(data),

            StxAbs | StxDP | StxDPY => self.exe_stx(low_addr, high_addr),
            StyAbs | StyDP | StyDPX => self.exe_sty(low_addr, high_addr),
            StzAbs | StzAbsX | StzDP | StzDPX => self.exe_stz(low_addr, high_addr),

            Tax => self.exe_tax(),
            Tay => self.exe_tay(),
            Tcd => self.exe_tcd(),
            Tcs => self.exe_tcs(),
            Tdc => self.exe_tdc(),

            TrbAbs | TrbDP => self.exe_trb(low_addr, high_addr),
            TsbAbs | TsbDP => self.exe_tsb(low_addr, high_addr),

            Tsc => self.exe_tsc(),
            Tsx => self.exe_tsx(),
            Txa => self.exe_txa(),
            Txs => self.exe_txs(),
            Txy => self.exe_txy(),
            Tya => self.exe_tya(),
            Tyx => self.exe_tyx(),

            Wai => self.exe_wai(data),
            Wdm => self.exe_wdm(data),
            Xba => self.exe_xba(data),
            Xce => self.exe_xce(data),

        }
    }
}
