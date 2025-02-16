use addressing::AddressingMode;


pub(crate) mod arithmetic;
pub(crate) mod branch;
pub(crate) mod flag;
pub(crate) mod interrupt;
pub(crate) mod jump;
pub(crate) mod loadstore;
pub(crate) mod logical;
pub(crate) mod move_instr;
pub(crate) mod other;
pub(crate) mod stack;
pub(crate) mod transfer;
pub(crate) mod addressing;

// pub mod instructions;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Adc(AddressingMode),
    And(AddressingMode),
    Asl(AddressingMode),
    Asla,
    Blt,
    Bge,
    Beq,
    Bit(AddressingMode),
    Bmi,
    Bne,
    Bpl,
    Bra,
    Brk,
    Brl,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp(AddressingMode),
    Cop,
    Cpx(AddressingMode),
    Cpy(AddressingMode),
    Dea,
    Dec(AddressingMode),
    Dex,
    Dey,
    Eor(AddressingMode),
    Ina,
    Inc(AddressingMode),
    Inx,
    Iny,
    Jmp(AddressingMode),
    Jml(AddressingMode),
    Jsr(AddressingMode),
    /// addressingmode = absolutelong
    Jsl,
    Lda(AddressingMode),
    Ldx(AddressingMode),
    Ldy(AddressingMode),
    Lsr(AddressingMode),
    Lsra,
    Mvn,
    Mvp,
    Nop,
    Ora(AddressingMode),
    Pea,
    Pei,
    Per,
    Pha,
    Phb,
    Phd,
    Phk,
    Php,
    Phx,
    Phy,
    Pla,
    Plb,
    Pld,
    Plp,
    Plx,
    Ply,
    Rep,
    Rol(AddressingMode),
    Rola,
    Rora,
    Ror(AddressingMode),
    Rti,
    Rtl,
    Rts,
    Sbc(AddressingMode),
    Sec,
    Sed,
    Sei,
    Sep,
    Sta(AddressingMode),
    Stp,
    Stx(AddressingMode),
    Sty(AddressingMode),
    Stz(AddressingMode),
    Tax,
    Tay,
    Tcd,
    Tcs,
    Tdc,
    Trb(AddressingMode),
    Tsb(AddressingMode),
    Tsc,
    Tsx,
    Txa,
    Txs,
    Txy,
    Tya,
    Tyx,
    Wai,
    Wdm,
    Xba,
    Xce,
}


impl Instruction {
    pub fn byte_length(&self, acc_8_bit: bool) -> usize {
        match self {
            // Always 2, regardless of emulation mode
            Self::Rep | Self::Sep => 2,
            _ => 1 + self.addressing_mode().byte_length(acc_8_bit)
        }
    }

    pub fn cycle_count(&self) -> usize {
        self.addressing_mode().cycle_count()
    }

    pub fn addressing_mode(&self) -> AddressingMode {
        match self {
            Self::Adc(mode)
            | Self::And(mode)
            | Self::Asl(mode)
            | Self::Bit(mode)
            | Self::Cmp(mode)
            | Self::Cpx(mode)
            | Self::Cpy(mode)
            | Self::Dec(mode)
            | Self::Eor(mode)
            | Self::Inc(mode)
            | Self::Jmp(mode)
            | Self::Jsr(mode)
            | Self::Jml(mode)
            | Self::Lda(mode)
            | Self::Ldx(mode)
            | Self::Ldy(mode)
            | Self::Lsr(mode)
            | Self::Ora(mode)
            | Self::Rol(mode)
            | Self::Ror(mode)
            | Self::Sbc(mode)
            | Self::Sta(mode)
            | Self::Stx(mode)
            | Self::Sty(mode)
            | Self::Stz(mode)
            | Self::Trb(mode)
            | Self::Tsb(mode) => *mode,

            Self::Asla
            | Self::Lsra
            | Self::Rola
            | Self::Rora => AddressingMode::Accumulator,
            Self::Jsl => AddressingMode::AbsoluteLong,
            Self::Mvn
            | Self::Mvp => AddressingMode::Move,

            // Branch instructions
            Self::Blt
            | Self::Bge
            | Self::Beq
            | Self::Bmi
            | Self::Bne
            | Self::Bpl
            | Self::Bra
            | Self::Bvc
            | Self::Bvs => AddressingMode::Relative,

            Self::Brl => AddressingMode::RelativeLong,

            Self::Rep | Self::Sep => AddressingMode::Immediate,

            Self::Brk | Self::Clc | Self::Cld | Self::Cli | Self::Clv | Self::Cop | Self::Dea | Self::Dex | Self::Dey | Self::Ina | Self::Inx | Self::Iny
            | Self::Nop | Self::Pea | Self::Pei | Self::Per | Self::Pha | Self::Phb | Self::Phd | Self::Phk | Self::Php | Self::Phx | Self::Phy | Self::Pla | Self::Plb | Self::Pld
            | Self::Plp | Self::Plx | Self::Ply | Self::Rti | Self::Rtl | Self::Rts | Self::Sec | Self::Sed | Self::Sei | Self::Stp | Self::Tax | Self::Tay
            | Self::Tcd | Self::Tcs | Self::Tdc | Self::Tsc | Self::Tsx | Self::Txa | Self::Txs | Self::Txy | Self::Tya | Self::Tyx | Self::Wai | Self::Wdm | Self::Xba | Self::Xce => {
                AddressingMode::Implied
            }
        }
    }


    /// Returns `Instruction` enum variant from op code
    pub fn from_op(op: u8) -> Instruction {
        match op {
            0x61 => Instruction::Adc(AddressingMode::IndirectX),
            0x63 => Instruction::Adc(AddressingMode::StackRelative),
            0x65 => Instruction::Adc(AddressingMode::Direct),
            0x67 => Instruction::Adc(AddressingMode::IndirectLong),
            0x69 => Instruction::Adc(AddressingMode::Immediate),
            0x6D => Instruction::Adc(AddressingMode::Absolute),
            0x6F => Instruction::Adc(AddressingMode::AbsoluteLong),
            0x71 => Instruction::Adc(AddressingMode::IndirectY),
            0x72 => Instruction::Adc(AddressingMode::Indirect),
            0x73 => Instruction::Adc(AddressingMode::StackRelativeY),
            0x75 => Instruction::Adc(AddressingMode::DirectX),
            0x77 => Instruction::Adc(AddressingMode::IndirectLongY),
            0x79 => Instruction::Adc(AddressingMode::AbsoluteY),
            0x7D => Instruction::Adc(AddressingMode::AbsoluteX),
            0x7F => Instruction::Adc(AddressingMode::AbsoluteLongX),
            0x21 => Instruction::And(AddressingMode::IndirectX),
            0x23 => Instruction::And(AddressingMode::StackRelative),
            0x25 => Instruction::And(AddressingMode::Direct),
            0x27 => Instruction::And(AddressingMode::IndirectLong),
            0x29 => Instruction::And(AddressingMode::Immediate),
            0x2D => Instruction::And(AddressingMode::Absolute),
            0x2F => Instruction::And(AddressingMode::AbsoluteLong),
            0x31 => Instruction::And(AddressingMode::IndirectY),
            0x32 => Instruction::And(AddressingMode::Indirect),
            0x33 => Instruction::And(AddressingMode::StackRelativeY),
            0x35 => Instruction::And(AddressingMode::DirectX),
            0x37 => Instruction::And(AddressingMode::IndirectLongY),
            0x39 => Instruction::And(AddressingMode::AbsoluteY),
            0x3D => Instruction::And(AddressingMode::AbsoluteX),
            0x3F => Instruction::And(AddressingMode::AbsoluteLongX),
            0x06 => Instruction::Asl(AddressingMode::Direct),
            0x0A => Instruction::Asla,
            0x0E => Instruction::Asl(AddressingMode::Absolute),
            0x16 => Instruction::Asl(AddressingMode::DirectX),
            0x1E => Instruction::Asl(AddressingMode::AbsoluteX),
            0x90 => Instruction::Blt,
            0xB0 => Instruction::Bge,
            0xF0 => Instruction::Beq,
            0x24 => Instruction::Bit(AddressingMode::Direct),
            0x2C => Instruction::Bit(AddressingMode::Absolute),
            0x34 => Instruction::Bit(AddressingMode::DirectX),
            0x3C => Instruction::Bit(AddressingMode::AbsoluteX),
            0x89 => Instruction::Bit(AddressingMode::Immediate),
            0x30 => Instruction::Bmi,
            0xD0 => Instruction::Bne,
            0x10 => Instruction::Bpl,
            0x80 => Instruction::Bra,
            0x00 => Instruction::Brk,
            0x82 => Instruction::Brl,
            0x50 => Instruction::Bvc,
            0x70 => Instruction::Bvs,
            0x18 => Instruction::Clc,
            0xD8 => Instruction::Cld,
            0x58 => Instruction::Cli,
            0xB8 => Instruction::Clv,
            0xC1 => Instruction::Cmp(AddressingMode::IndirectX),
            0xC3 => Instruction::Cmp(AddressingMode::StackRelative),
            0xC5 => Instruction::Cmp(AddressingMode::Direct),
            0xC7 => Instruction::Cmp(AddressingMode::IndirectLong),
            0xC9 => Instruction::Cmp(AddressingMode::Immediate),
            0xCD => Instruction::Cmp(AddressingMode::Absolute),
            0xCF => Instruction::Cmp(AddressingMode::AbsoluteLong),
            0xD1 => Instruction::Cmp(AddressingMode::IndirectY),
            0xD2 => Instruction::Cmp(AddressingMode::Indirect),
            0xD3 => Instruction::Cmp(AddressingMode::StackRelativeY),
            0xD5 => Instruction::Cmp(AddressingMode::DirectX),
            0xD7 => Instruction::Cmp(AddressingMode::IndirectLongY),
            0xD9 => Instruction::Cmp(AddressingMode::AbsoluteY),
            0xDD => Instruction::Cmp(AddressingMode::AbsoluteX),
            0xDF => Instruction::Cmp(AddressingMode::AbsoluteLongX),
            0x02 => Instruction::Cop,
            0xE0 => Instruction::Cpx(AddressingMode::Immediate),
            0xE4 => Instruction::Cpx(AddressingMode::Direct),
            0xEC => Instruction::Cpx(AddressingMode::Absolute),
            0xC0 => Instruction::Cpy(AddressingMode::Immediate),
            0xC4 => Instruction::Cpy(AddressingMode::Direct),
            0xCC => Instruction::Cpy(AddressingMode::Absolute),
            0x3A => Instruction::Dea,
            0xC6 => Instruction::Dec(AddressingMode::Direct),
            0xCE => Instruction::Dec(AddressingMode::Absolute),
            0xD6 => Instruction::Dec(AddressingMode::DirectX),
            0xDE => Instruction::Dec(AddressingMode::AbsoluteX),
            0xCA => Instruction::Dex,
            0x88 => Instruction::Dey,
            0x41 => Instruction::Eor(AddressingMode::IndirectX),
            0x43 => Instruction::Eor(AddressingMode::StackRelative),
            0x45 => Instruction::Eor(AddressingMode::Direct),
            0x47 => Instruction::Eor(AddressingMode::IndirectLong),
            0x49 => Instruction::Eor(AddressingMode::Immediate),
            0x4D => Instruction::Eor(AddressingMode::Absolute),
            0x4F => Instruction::Eor(AddressingMode::AbsoluteLong),
            0x51 => Instruction::Eor(AddressingMode::IndirectY),
            0x52 => Instruction::Eor(AddressingMode::Indirect),
            0x53 => Instruction::Eor(AddressingMode::StackRelativeY),
            0x55 => Instruction::Eor(AddressingMode::DirectX),
            0x57 => Instruction::Eor(AddressingMode::IndirectLongY),
            0x59 => Instruction::Eor(AddressingMode::AbsoluteY),
            0x5D => Instruction::Eor(AddressingMode::AbsoluteX),
            0x5F => Instruction::Eor(AddressingMode::AbsoluteLongX),
            0x1A => Instruction::Ina,
            0xE6 => Instruction::Inc(AddressingMode::Direct),
            0xEE => Instruction::Inc(AddressingMode::Absolute),
            0xF6 => Instruction::Inc(AddressingMode::DirectX),
            0xFE => Instruction::Inc(AddressingMode::AbsoluteX),
            0xE8 => Instruction::Inx,
            0xC8 => Instruction::Iny,
            0x4C => Instruction::Jmp(AddressingMode::Absolute),
            0x5C => Instruction::Jml(AddressingMode::AbsoluteLong),
            0x6C => Instruction::Jmp(AddressingMode::Indirect),
            0x7C => Instruction::Jmp(AddressingMode::AbsoluteIndirectX),
            0xDC => Instruction::Jml(AddressingMode::AbsoluteIndirectLong),
            0x20 => Instruction::Jsr(AddressingMode::Absolute),
            0x22 => Instruction::Jsl,
            0xFC => Instruction::Jsr(AddressingMode::AbsoluteIndirectX),
            0xA1 => Instruction::Lda(AddressingMode::IndirectX),
            0xA3 => Instruction::Lda(AddressingMode::StackRelative),
            0xA5 => Instruction::Lda(AddressingMode::Direct),
            0xA7 => Instruction::Lda(AddressingMode::IndirectLong),
            0xA9 => Instruction::Lda(AddressingMode::Immediate),
            0xAD => Instruction::Lda(AddressingMode::Absolute),
            0xAF => Instruction::Lda(AddressingMode::AbsoluteLong),
            0xB1 => Instruction::Lda(AddressingMode::IndirectY),
            0xB2 => Instruction::Lda(AddressingMode::Indirect),
            0xB3 => Instruction::Lda(AddressingMode::StackRelativeY),
            0xB5 => Instruction::Lda(AddressingMode::DirectX),
            0xB7 => Instruction::Lda(AddressingMode::IndirectLongY),
            0xB9 => Instruction::Lda(AddressingMode::AbsoluteY),
            0xBD => Instruction::Lda(AddressingMode::AbsoluteX),
            0xBF => Instruction::Lda(AddressingMode::AbsoluteLongX),
            0xA2 => Instruction::Ldx(AddressingMode::Immediate),
            0xA6 => Instruction::Ldx(AddressingMode::Direct),
            0xAE => Instruction::Ldx(AddressingMode::Absolute),
            0xB6 => Instruction::Ldx(AddressingMode::DirectY),
            0xBE => Instruction::Ldx(AddressingMode::AbsoluteY),
            0xA0 => Instruction::Ldy(AddressingMode::Immediate),
            0xA4 => Instruction::Ldy(AddressingMode::Direct),
            0xAC => Instruction::Ldy(AddressingMode::Absolute),
            0xB4 => Instruction::Ldy(AddressingMode::DirectX),
            0xBC => Instruction::Ldy(AddressingMode::AbsoluteX),
            0x46 => Instruction::Lsr(AddressingMode::Direct),
            0x4A => Instruction::Lsra,
            0x4E => Instruction::Lsr(AddressingMode::Absolute),
            0x56 => Instruction::Lsr(AddressingMode::DirectX),
            0x5E => Instruction::Lsr(AddressingMode::AbsoluteX),
            0x54 => Instruction::Mvn,
            0x44 => Instruction::Mvp,
            0xEA => Instruction::Nop,
            0x01 => Instruction::Ora(AddressingMode::IndirectX),
            0x03 => Instruction::Ora(AddressingMode::StackRelative),
            0x05 => Instruction::Ora(AddressingMode::Direct),
            0x07 => Instruction::Ora(AddressingMode::IndirectLong),
            0x09 => Instruction::Ora(AddressingMode::Immediate),
            0x0D => Instruction::Ora(AddressingMode::Absolute),
            0x0F => Instruction::Ora(AddressingMode::AbsoluteLong),
            0x11 => Instruction::Ora(AddressingMode::IndirectY),
            0x12 => Instruction::Ora(AddressingMode::Indirect),
            0x13 => Instruction::Ora(AddressingMode::StackRelativeY),
            0x15 => Instruction::Ora(AddressingMode::DirectX),
            0x17 => Instruction::Ora(AddressingMode::IndirectLongY),
            0x19 => Instruction::Ora(AddressingMode::AbsoluteY),
            0x1D => Instruction::Ora(AddressingMode::AbsoluteX),
            0x1F => Instruction::Ora(AddressingMode::AbsoluteLongX),
            0xF4 => Instruction::Pea,
            0xD4 => Instruction::Pei,
            0x62 => Instruction::Per,
            0x48 => Instruction::Pha,
            0x8B => Instruction::Phb,
            0x0B => Instruction::Phd,
            0x4B => Instruction::Phk,
            0x08 => Instruction::Php,
            0xDA => Instruction::Phx,
            0x5A => Instruction::Phy,
            0x68 => Instruction::Pla,
            0xAB => Instruction::Plb,
            0x2B => Instruction::Pld,
            0x28 => Instruction::Plp,
            0xFA => Instruction::Plx,
            0x7A => Instruction::Ply,
            0xC2 => Instruction::Rep,
            0x26 => Instruction::Rol(AddressingMode::Direct),
            0x2A => Instruction::Rola,
            0x2E => Instruction::Rol(AddressingMode::Absolute),
            0x36 => Instruction::Rol(AddressingMode::DirectX),
            0x3E => Instruction::Rol(AddressingMode::AbsoluteX),
            0x66 => Instruction::Ror(AddressingMode::Direct),
            0x6A => Instruction::Rora,
            0x6E => Instruction::Ror(AddressingMode::Absolute),
            0x76 => Instruction::Ror(AddressingMode::DirectX),
            0x7E => Instruction::Ror(AddressingMode::AbsoluteX),
            0x40 => Instruction::Rti,
            0x6B => Instruction::Rtl,
            0x60 => Instruction::Rts,
            0xE1 => Instruction::Sbc(AddressingMode::IndirectX),
            0xE3 => Instruction::Sbc(AddressingMode::StackRelative),
            0xE5 => Instruction::Sbc(AddressingMode::Direct),
            0xE7 => Instruction::Sbc(AddressingMode::IndirectLong),
            0xE9 => Instruction::Sbc(AddressingMode::Immediate),
            0xED => Instruction::Sbc(AddressingMode::Absolute),
            0xEF => Instruction::Sbc(AddressingMode::AbsoluteLong),
            0xF1 => Instruction::Sbc(AddressingMode::IndirectY),
            0xF2 => Instruction::Sbc(AddressingMode::Indirect),
            0xF3 => Instruction::Sbc(AddressingMode::StackRelativeY),
            0xF5 => Instruction::Sbc(AddressingMode::DirectX),
            0xF7 => Instruction::Sbc(AddressingMode::IndirectLongY),
            0xF9 => Instruction::Sbc(AddressingMode::AbsoluteY),
            0xFD => Instruction::Sbc(AddressingMode::AbsoluteX),
            0xFF => Instruction::Sbc(AddressingMode::AbsoluteLongX),
            0x38 => Instruction::Sec,
            0xF8 => Instruction::Sed,
            0x78 => Instruction::Sei,
            0xE2 => Instruction::Sep,
            0x81 => Instruction::Sta(AddressingMode::IndirectX),
            0x83 => Instruction::Sta(AddressingMode::StackRelative),
            0x85 => Instruction::Sta(AddressingMode::Direct),
            0x87 => Instruction::Sta(AddressingMode::IndirectLong),
            0x8D => Instruction::Sta(AddressingMode::Absolute),
            0x8F => Instruction::Sta(AddressingMode::AbsoluteLong),
            0x91 => Instruction::Sta(AddressingMode::IndirectY),
            0x92 => Instruction::Sta(AddressingMode::Indirect),
            0x93 => Instruction::Sta(AddressingMode::StackRelativeY),
            0x95 => Instruction::Sta(AddressingMode::DirectX),
            0x97 => Instruction::Sta(AddressingMode::IndirectLongY),
            0x99 => Instruction::Sta(AddressingMode::AbsoluteY),
            0x9D => Instruction::Sta(AddressingMode::AbsoluteX),
            0x9F => Instruction::Sta(AddressingMode::AbsoluteLongX),
            0xDB => Instruction::Stp,
            0x86 => Instruction::Stx(AddressingMode::Direct),
            0x8E => Instruction::Stx(AddressingMode::Absolute),
            0x96 => Instruction::Stx(AddressingMode::DirectY),
            0x84 => Instruction::Sty(AddressingMode::Direct),
            0x8C => Instruction::Sty(AddressingMode::Absolute),
            0x94 => Instruction::Sty(AddressingMode::DirectX),
            0x64 => Instruction::Stz(AddressingMode::Direct),
            0x74 => Instruction::Stz(AddressingMode::DirectX),
            0x9C => Instruction::Stz(AddressingMode::Absolute),
            0x9E => Instruction::Stz(AddressingMode::AbsoluteX),
            0xAA => Instruction::Tax,
            0xA8 => Instruction::Tay,
            0x5B => Instruction::Tcd,
            0x1B => Instruction::Tcs,
            0x7B => Instruction::Tdc,
            0x14 => Instruction::Trb(AddressingMode::Direct),
            0x1C => Instruction::Trb(AddressingMode::Absolute),
            0x04 => Instruction::Tsb(AddressingMode::Direct),
            0x0C => Instruction::Tsb(AddressingMode::Absolute),
            0x3B => Instruction::Tsc,
            0xBA => Instruction::Tsx,
            0x8A => Instruction::Txa,
            0x9A => Instruction::Txs,
            0x9B => Instruction::Txy,
            0x98 => Instruction::Tya,
            0xBB => Instruction::Tyx,
            0xCB => Instruction::Wai,
            0x42 => Instruction::Wdm,
            0xEB => Instruction::Xba,
            0xFB => Instruction::Xce,
        }
    }
}
