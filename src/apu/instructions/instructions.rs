use super::AddressingMode;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    /// Generic move
    MOV(AddressingMode),
    /// Mov instruction that loads from memory to register
    STA(AddressingMode),
    /// Mov instruction that stores register to memory
    LDA(AddressingMode),

    STX(AddressingMode),
    LDX(AddressingMode),

    STY(AddressingMode),
    LDY(AddressingMode),
    
    /// Add with carry
    ADC(AddressingMode),
    /// Subtract with borrow
    SBC(AddressingMode),
    /// Compare with accumulator, x, or y register
    CMP(AddressingMode),
    CPX(AddressingMode),
    CPY(AddressingMode),
    /// And with accumulator, x or dp address
    AND(AddressingMode),
    /// OR with accumulator, x or dp address
    OR(AddressingMode),
    /// XOR with accumulator, x or dp address
    EOR(AddressingMode),
    /// Shift left accumulator, x, y or dp address
    ASL(AddressingMode),
    ASLA(AddressingMode),
    /// Shift right accumulator, x, y or dp address
    LSR(AddressingMode),
    LSRA(AddressingMode),
    /// Rotate left accumulator, x, y or dp address
    ROL(AddressingMode),
    ROLA(AddressingMode),
    /// Rotate right accumulator, x, y or dp address
    ROR(AddressingMode),
    RORA(AddressingMode),
    /// Exchange upper and lower nybble of accumulator
    XCN,
    /// Increment accumulator, x, y or dp address
    INC(AddressingMode),
    INX,
    INY,
    INA,
    /// Decrement accumulator, x, y or dp address
    DEC(AddressingMode),
    DEX,
    DEY,
    DEA,
    /// Move YA to direct page
    STW(AddressingMode),
    /// Move direct page to YA
    LDW(AddressingMode),
    /// Increment dp as 16 bits
    INCW(AddressingMode),
    /// Decrement dp as 16 bits
    DECW(AddressingMode),
    /// Add 16 bit dp to YA
    ADDW(AddressingMode),
    /// Sub 16 bit dp from YA
    SUBW(AddressingMode),
    /// Compare 16 bit dp to YA
    CMPW(AddressingMode),
    /// Multiply, YA = Y * A
    /// 
    /// n/z flag based on Y register (high byte of multiplication)
    MUL,
    /// Divide, A = YA / X, Y = YA % X
    /// 
    /// bit 8 of quotient is stored in overflow flag
    /// 
    /// Output only valid if quotient <= 511
    /// 
    /// n/z flag based on A register (bits 0-7 of quotient)
    DIV,
    /// Apply carry/half-carry after BCD addition
    DAA,
    /// Apply carry/half-carry after BCD subtraction
    DAS,
    /// Branch always
    BRA(AddressingMode),
    /// Branch if Z = 1
    BEQ(AddressingMode),
    /// Branch if Z = 0
    BNE(AddressingMode),
    /// Branch if C = 1
    BCS(AddressingMode),
    /// Branch if C = 0
    BCC(AddressingMode),
    /// Branch if V = 0
    BVS(AddressingMode),
    /// Branch if V = 0
    BVC(AddressingMode),
    /// Branch if N = 1
    BMI(AddressingMode),
    /// Branch if N = 0
    BPL(AddressingMode),
    /// Branch if dp bit = 1
    /// 
    /// Second argument in tuple holds the bit to check
    BBS(AddressingMode, u8),
    /// Branch if dp bit = 1
    /// 
    /// Second argument in tuple holds the bit to check
    BBC(AddressingMode, u8),
    /// CMP then BNE
    CBNE(AddressingMode),
    /// DEC then BNE
    DBNZ(AddressingMode),
    /// Jump to address
    JMP(AddressingMode),
    /// Jump to subroutine
    CALL(AddressingMode),
    /// Equivalent to `CALL $FF00 arg`, where arg is argument byte
    PCALL(AddressingMode),
    /// Equivalent to `CALL [$FFDE-(2*bit)]`, where `bit` is dependent on opcode. Essentially a table lookup
    TCALL(AddressingMode, u8),
    /// Break, sets break flag and clears interrupt enable flag
    /// 
    /// Pushes PSW and PC to the stack
    BRK,
    /// Return from subroutine
    RET,
    /// Return from interrupt
    RETI,
    
    // Stack
    /// Push status word register
    PUSHP,
    /// Push acc to stack
    PUSHA,
    /// Push x to stack
    PUSHX,
    /// Push y to stack
    PUSHY,

    /// Pop Status word register
    POPP,
    /// Pop accumulator
    POPA,
    /// Pop x register
    POPX,
    /// Pop y register
    POPY,

    /// Set dp bit, x=(bit*2)
    ///
    /// Second argument in tuple holds the bit to check
    SET1(AddressingMode, u8),

    /// Set dp bit, y=(bit*2)+1
    /// 
    /// Second argument in tuple holds the bit to check
    CLR1(AddressingMode, u8),
    /// Test and set bits with Acc
    /// 
    /// Equality test (A - old_value)
    TSET1(AddressingMode),
    /// Test and clear bits with Acc
    /// 
    /// Equality test (A - old_value)
    TCLR1(AddressingMode),
    /// Carry &= [abs]_bit
    AND1(AddressingMode),
    /// Carry |= [abs]_bit
    OR1(AddressingMode),
    /// Carry &= !([abs]_bit)
    ANDNOT1(AddressingMode),
    /// Carry != !([abs]_bit)
    ORNOT1(AddressingMode),
    /// Carry ^= [abs]_bit
    EOR1(AddressingMode),
    /// ![abs]_bit ^= 1 (toggle 1 bit)
    NOT1(AddressingMode),
    /// [abs]_bit = C
    ST1(AddressingMode),
    /// C = [abs]_bit
    LD1(AddressingMode),
    /// Clear carry
    CLRC,
    /// Set carry
    SETC,
    /// Invert carry flag
    NOTC,
    /// Clear overflow
    CLRV,
    /// Set direct page flag
    SETP,
    /// Clear direct page flag
    CLRP,
    /// Set InterruptEnable
    EI,
    /// Clear InterruptEnable
    DI,
    /// No operation
    NOP,
    /// Sleeps processor, does same as STOP as there are no interrupts
    SLEEP,
    /// Stop processor
    STOP,
    
    /// Transfer accumulator -> x register
    TAX,
    /// Transfer X register -> accumulator
    TXA, 
    /// Transfer accumulator -> x register
    TAY,
    /// Transfer Y register -> accumulator
    TYA,
    /// Transfer Stack pointer -> X register
    TSX,
    /// Transfer X register -> Stack pointer
    TXS,

}

impl Instruction {
    pub fn from_op(op: u8) -> Self {
        use crate::apu::instructions::instructions::{Instruction::*, AddressingMode::*};

        match op {
            0x00 => NOP,
            0x01 => BPL(Relative),
            0x02 => CLRP,
            0x03 => BMI(Relative),
            0x04 => SETP,
            0x05 => BVC(Relative),
            0x06 => CLRC,
            0x07 => BVS(Relative),
            0x08 => SETC,
            0x09 => BCC(Relative),
            0x0A => EI,
            0x0B => BCS(Relative),
            0x0C => DI,
            0x0D => BNE(Relative),
            0x0E => CLRV,
            0x0F => BEQ(Relative),

            0x10 => TCALL(Implied, 0),
            0x11 => TCALL(Implied, 1),
            0x12 => TCALL(Implied, 2),
            0x13 => TCALL(Implied, 3),
            0x14 => TCALL(Implied, 4),
            0x15 => TCALL(Implied, 5),
            0x16 => TCALL(Implied, 6),
            0x17 => TCALL(Implied, 7),
            0x18 => TCALL(Implied, 8),
            0x19 => TCALL(Implied, 9),
            0x1A => TCALL(Implied, 0xA),
            0x1B => TCALL(Implied, 0xB),
            0x1C => TCALL(Implied, 0xC),
            0x1D => TCALL(Implied, 0xD),
            0x1E => TCALL(Implied, 0xE),
            0x1F => TCALL(Implied, 0xF),

            0x20 => SET1(DirectPage, 0),
            0x21 => CLR1(DirectPage, 1),
            0x22 => SET1(DirectPage, 2),
            0x23 => CLR1(DirectPage, 3),
            0x24 => SET1(DirectPage, 4),
            0x25 => CLR1(DirectPage, 5),
            0x26 => SET1(DirectPage, 6),
            0x27 => CLR1(DirectPage, 7),
            0x28 => SET1(DirectPage, 8),
            0x29 => CLR1(DirectPage, 9),
            0x2A => SET1(DirectPage, 0xA),
            0x2B => CLR1(DirectPage, 0xB),
            0x2C => SET1(DirectPage, 0xC),
            0x2D => CLR1(DirectPage, 0xD),
            0x2E => SET1(DirectPage, 0xE),
            0x2F => CLR1(DirectPage, 0xF),

            0x30 => BBS(Relative, 0),
            0x31 => BBC(Relative, 1),
            0x32 => BBS(Relative, 2),
            0x33 => BBC(Relative, 3),
            0x34 => BBS(Relative, 4),
            0x35 => BBC(Relative, 5),
            0x36 => BBS(Relative, 6),
            0x37 => BBC(Relative, 7),
            0x38 => BBS(Relative, 8),
            0x39 => BBC(Relative, 9),
            0x3A => BBS(Relative, 0xA),
            0x3B => BBC(Relative, 0xB),
            0x3C => BBS(Relative, 0xC),
            0x3D => BBC(Relative, 0xD),
            0x3E => BBS(Relative, 0xE),
            0x3F => BBC(Relative, 0xF),

            0x40 => OR(DirectPage),
            0x41 => OR(DirectPageX),
            0x42 => AND(DirectPage),
            0x43 => AND(DirectPageX),
            0x44 => EOR(DirectPage),
            0x45 => EOR(DirectPageX),
            0x46 => CMP(DirectPage),
            0x47 => CMP(DirectPageX),
            0x48 => ADC(DirectPage),
            0x49 => ADC(DirectPageX),
            0x4A => SBC(DirectPage),
            0x4B => SBC(DirectPageX),
            0x4C => STA(DirectPage),
            0x4D => STA(DirectPageX),
            0x4E => LDA(DirectPage),
            0x4F => LDA(DirectPageX),

            0x50 => OR(Absolute),
            0x51 => OR(AbsoluteX),
            0x52 => AND(Absolute),
            0x53 => AND(AbsoluteX),
            0x54 => EOR(Absolute),
            0x55 => EOR(AbsoluteX),
            0x56 => CMP(Absolute),
            0x57 => CMP(AbsoluteX),
            0x58 => ADC(Absolute),
            0x59 => ADC(AbsoluteX),
            0x5A => SBC(Absolute),
            0x5B => SBC(AbsoluteX),
            0x5C => STA(Absolute),
            0x5D => STA(AbsoluteX),
            0x5E => LDA(Absolute),
            0x5F => LDA(AbsoluteX),

            0x60 => OR(XIndirect),
            0x61 => OR(AbsoluteY),
            0x62 => AND(XIndirect),
            0x63 => AND(AbsoluteY),
            0x64 => EOR(XIndirect),
            0x65 => EOR(AbsoluteY),
            0x66 => CMP(XIndirect),
            0x67 => CMP(AbsoluteY),
            0x68 => ADC(XIndirect),
            0x69 => ADC(AbsoluteY),
            0x6A => SBC(XIndirect),
            0x6B => SBC(AbsoluteY),
            0x6C => STA(XIndirect),
            0x6D => STA(AbsoluteY),
            0x6E => LDA(XIndirect),
            0x6F => LDA(AbsoluteY),
            
            0x70 => OR(IndirectX),
            0x71 => OR(IndirectY),
            0x72 => AND(IndirectX),
            0x73 => AND(IndirectY),
            0x74 => EOR(IndirectX),
            0x75 => EOR(IndirectY),
            0x76 => CMP(IndirectX),
            0x77 => CMP(IndirectY),
            0x78 => ADC(IndirectX),
            0x79 => ADC(IndirectY),
            0x7A => SBC(IndirectX),
            0x7B => SBC(IndirectY),
            0x7C => STA(IndirectX),
            0x7D => STA(IndirectY),
            0x7E => LDA(IndirectX),
            0x7F => LDA(IndirectY),

            0x80 => OR(Immediate),
            0x81 => OR(DPImmediate),
            0x82 => AND(Immediate),
            0x83 => AND(DPImmediate),
            0x84 => EOR(Immediate),
            0x85 => EOR(DPImmediate),
            0x86 => CMP(Immediate),
            0x87 => CMP(DPImmediate),
            0x88 => ADC(Immediate),
            0x89 => ADC(DPImmediate),
            0x8A => SBC(Immediate),
            0x8B => SBC(DPImmediate),
            0x8C => CPX(Immediate),
            0x8D => STX(DirectPage),
            0x8E => LDA(Immediate),
            0x8F => LDX(Immediate),

            0x90 => OR(DPtoDP),
            0x91 => OR(XIndirectYIndirect),
            0x92 => AND(DPtoDP),
            0x93 => AND(XIndirectYIndirect),
            0x94 => EOR(DPtoDP),
            0x95 => EOR(XIndirectYIndirect),
            0x96 => CMP(DPtoDP),
            0x97 => CMP(XIndirectYIndirect),
            0x98 => ADC(DPtoDP),
            0x99 => ADC(XIndirectYIndirect),
            0x9A => SBC(DPtoDP),
            0x9B => SBC(XIndirectYIndirect),
            0x9C => STX(Absolute),
            0x9D => STX(DirectPageY),
            0x9E => LDX(Absolute),
            0x9F => LDX(DirectPageY),

            0xA0 => OR1(AbsoluteBit),
            0xA1 => DECW(Word),
            0xA2 => ORNOT1(AbsoluteBit),
            0xA3 => INCW(Word),
            0xA4 => AND1(AbsoluteBit),
            0xA5 => CMPW(Word),
            0xA6 => ANDNOT1(AbsoluteBit),
            0xA7 => ADDW(Word),
            0xA8 => EOR1(AbsoluteBit),
            0xA9 => SUBW(Word),
            0xAA => ST1(AbsoluteBit),
            0xAB => STW(Word),
            0xAC => LD1(AbsoluteBit),
            0xAD => LDW(Word),
            0xAE => NOT1(AbsoluteBit),
            0xAF => MOV(DPtoDP),

            0xB0 => ASL(DirectPage),
            0xB1 => ASL(DirectPageX),
            0xB2 => ROL(DirectPage),
            0xB3 => ROL(DirectPageX),
            0xB4 => LSR(DirectPage),
            0xB5 => LSR(DirectPageX),
            0xB6 => ROR(DirectPage),
            0xB7 => ROR(DirectPageX),
            0xB8 => DEC(DirectPage),
            0xB9 => DEC(DirectPageX),
            0xBA => INC(DirectPage),
            0xBB => INC(DirectPageX),
            0xBC => STY(DirectPage),
            0xBD => STY(DirectPageX),
            0xBE => LDY(DirectPage),
            0xBF => LDY(DirectPageX),

            0xC0 => ASL(Absolute),
            0xC1 => ASLA(Implied),
            0xC2 => ROL(Absolute),
            0xC3 => ROLA(Implied),
            0xC4 => LSR(Absolute),
            0xC5 => LSRA(Implied),
            0xC6 => ROR(Absolute),
            0xC7 => RORA(Implied),
            0xC8 => DEC(Absolute),
            0xC9 => DEA,
            0xCA => INC(Absolute),
            0xCB => INA,
            0xCC => STY(Absolute),
            0xCD => DEY,
            0xCE => LDY(Absolute),
            0xCF => INY,

            0xD0 => PUSHP,
            0xD1 => DEX,
            0xD2 => PUSHA,
            0xD3 => INX,
            0xD4 => PUSHX,
            0xD5 => TAX,
            0xD6 => PUSHY,
            0xD7 => TXA,
            0xD8 => LDY(Immediate),
            0xD9 => TSX,
            0xDA => CPY(Immediate),
            0xDB => TXS,
            0xDC => LDX(Immediate),
            0xDD => TYA,
            0xDE => NOTC,
            0xDF => TAY,

            0xE0 => TSET1(Absolute),
            0xE1 => CPX(Absolute),
            0xE2 => CBNE(DPRelative),
            0xE3 => CPX(DirectPage),
            0xE4 => TCLR1(Absolute),
            0xE5 => CPY(Absolute),
            0xE6 => DBNZ(DPRelative),
            0xE7 => CPY(DirectPage),
            0xE8 => POPP,
            0xE9 => DIV,
            0xEA => POPA,
            0xEB => DAS,
            0xEC => POPX,
            0xED => CBNE(DPXRelative),
            0xEE => POPY,
            0xEF => DBNZ(Relative),
            
            0xF0 => BRK,
            0xF1 => JMP(AbsoluteX),
            0xF2 => BRA(Relative),
            0xF3 => CALL(Absolute),
            0xF4 => PCALL(Absolute),
            0xF5 => JMP(Absolute),
            0xF6 => RET,
            0xF7 => RETI,
            0xF8 => MOV(DPImmediate),
            0xF9 => XCN,
            0xFA => LDX(IncrementX),
            0xFB => LDA(IncrementX),
            0xFC => MUL,
            0xFD => DAA,
            0xFE => SLEEP,
            0xFF => STOP,
        }
    }
}