use crate::cpu::instructions::AddressingMode;
#[derive(Debug)]
pub enum Instruction {
    
    /// `ADC`, Add With Carry (DP Indexed Indirect,X)
    ADC(AddressingMode),
    
    /// `AND`, AND Accumulator with Memory (DP Indexed Indirect,X)
    AND(AddressingMode),
    
    /// `ASL`, Arithmetic Shift Left on accumulator
    ASLA(AddressingMode),

    /// `ASL`, Arithmetic Shift Left (Direct Page)
    ASL(AddressingMode),
    
    /// `BEQ`, Branch if Equal (Program Counter Relative)
    BEQ(AddressingMode),
    
    /// `BGE`, Branch if Carry Set (Program Counter Relative)
    BGE(AddressingMode),
    
    /// `BIT`, Test Bits (Direct Page)
    BIT(AddressingMode),
    
    /// `BLT`, Branch if Carry Clear (Program Counter Relative)
    BLT(AddressingMode),
    
    /// `BMI`, Branch if Minus (Program Counter Relative)
    BMI(AddressingMode),
    
    /// `BNE`, Branch if Not Equal (Program Counter Relative)
    BNE(AddressingMode),
    
    /// `BPL`, Branch if Plus (Program Counter Relative)
    BPL(AddressingMode),
    
    /// `BRA`, Branch Always (Program Counter Relative)
    BRA(AddressingMode),
    
    /// `BRK`, Break (Stack/Interrupt)
    BRK(AddressingMode),
    
    /// `BRL`, Branch Long Always (Program Counter Relative Long)
    BRL(AddressingMode),
    
    /// `BVC`, Branch if Overflow Clear (Program Counter Relative)
    BVC(AddressingMode),
    
    /// `BVS`, Branch if Overflow Set (Program Counter Relative)
    BVS(AddressingMode),
    
    /// `CLC`, Clear Carry (Implied)
    CLC(AddressingMode),
    
    /// `CLD`, Clear Decimal Mode Flag (Implied)
    CLD(AddressingMode),
    
    /// `CLI`, Clear Interrupt Disable Flag (Implied)
    CLI(AddressingMode),
    
    /// `CLV`, Clear Overflow Flag (Implied)
    CLV(AddressingMode),
    
    /// `CMP`, Compare Accumulator with Memory (DP Indexed Indirect,X)
    CMP(AddressingMode),
    
    /// `COP`, Co-Processor (Stack/Interrupt)
    COP(AddressingMode),
    
    /// `CPX`, Compare Index Register X with Memory (Immediate)
    CPX(AddressingMode),
    
    /// `CPY`, Compare Index Register Y with Memory (Immediate)
    CPY(AddressingMode),
    
    /// `DEA`, Decrement (Accumulator)
    DEA(AddressingMode),
    
    /// `DEC`, Decrement (Direct Page)
    DEC(AddressingMode),
    
    /// `DEX`, Decrement Index Register X (Implied)
    DEX(AddressingMode),
    
    /// `DEY`, Decrement Index Register Y (Implied)
    DEY(AddressingMode),
    
    /// `EOR`, Exclusive-OR Accumulator with Memory (DP Indexed Indirect,X)
    EOR(AddressingMode),
    
    /// `INA`, Increment (Accumulator)
    INA(AddressingMode),
    
    /// `INC`, Increment (Direct Page)
    INC(AddressingMode),
    
    /// `INX`, Increment Index Register X (Implied)
    INX(AddressingMode),
    
    /// `INY`, Increment Index Register Y (Implied)
    INY(AddressingMode),
    
    /// `JML`, Jump (Absolute Long)
    JML(AddressingMode),
    
    /// `JMP`, Jump (Absolute)
    JMP(AddressingMode),
    
    /// `JSL`, Jump to Subroutine (Absolute Long)
    JSL(AddressingMode),
    
    /// `JSR`, Jump to Subroutine (Absolute)
    JSR(AddressingMode),
    
    /// `LDA`, Load Accumulator from Memory (DP Indexed Indirect,X)
    LDA(AddressingMode),
    
    /// `LDX`, Load Index Register X from Memory (Immediate)
    LDX(AddressingMode),
    
    /// `LDY`, Load Index Register Y from Memory (Immediate)
    LDY(AddressingMode),
    
    LSRA(AddressingMode),

    /// `LSR`, Logical Shift Memory or Accumulator Right (Direct Page)
    LSR(AddressingMode),
    
    /// `MVN`, Block Move Negative (Block Move)
    MVN(AddressingMode),
    
    /// `MVP`, Block Move Positive (Block Move)
    MVP(AddressingMode),
    
    /// `NOP`, No Operation (Implied)
    NOP(AddressingMode),
    
    /// `ORA`, OR Accumulator with Memory (DP Indexed Indirect,X)
    ORA(AddressingMode),
    
    /// `PEA`, Push Effective Absolute Address (Stack (Absolute))
    PEA(AddressingMode),
    
    /// `PEI`, Push Effective Indirect Address (Stack (DP Indirect))
    PEI(AddressingMode),
    
    /// `PER`, Push Effective PC Relative Indirect Address (Stack (PC Relative Long))
    PER(AddressingMode),
    
    /// `PHA`, Push Accumulator (Stack (Push))
    PHA(AddressingMode),
    
    /// `PHB`, Push Data Bank Register (Stack (Push))
    PHB(AddressingMode),
    
    /// `PHD`, Push Direct Page Register (Stack (Push))
    PHD(AddressingMode),
    
    /// `PHK`, Push Program Bank Register (Stack (Push))
    PHK(AddressingMode),
    
    /// `PHP`, Push Processor Status Register (Stack (Push))
    PHP(AddressingMode),
    
    /// `PHX`, Push Index Register X (Stack (Push))
    PHX(AddressingMode),
    
    /// `PHY`, Push Index Register Y (Stack (Push))
    PHY(AddressingMode),
    
    /// `PLA`, Pull Accumulator (Stack (Pull))
    PLA(AddressingMode),
    
    /// `PLB`, Pull Data Bank Register (Stack (Pull))
    PLB(AddressingMode),
    
    /// `PLD`, Pull Direct Page Register (Stack (Pull))
    PLD(AddressingMode),
    
    /// `PLP`, Pull Processor Status Register (Stack (Pull))
    PLP(AddressingMode),
    
    /// `PLX`, Pull Index Register X (Stack (Pull))
    PLX(AddressingMode),
    
    /// `PLY`, Pull Index Register Y (Stack (Pull))
    PLY(AddressingMode),
    
    /// `REP`, Reset Processor Status Bits (Immediate)
    REP(AddressingMode),
    
    ROLA(AddressingMode),

    /// `ROL`, Rotate Memory or Accumulator Left (Direct Page)
    ROL(AddressingMode),

    RORA(AddressingMode),
    
    /// `ROR`, Rotate Memory or Accumulator Right (Direct Page)
    ROR(AddressingMode),
    
    /// `RTI`, Return from Interrupt (Stack (RTI))
    RTI(AddressingMode),
    
    /// `RTL`, Return from Subroutine Long (Stack (RTL))
    RTL(AddressingMode),
    
    /// `RTS`, Return from Subroutine (Stack (RTS))
    RTS(AddressingMode),
    
    /// `SBC`, Subtract with Borrow from Accumulator (DP Indexed Indirect,X)
    SBC(AddressingMode),
    
    /// `SEC`, Set Carry Flag (Implied)
    SEC(AddressingMode),
    
    /// `SED`, Set Decimal Flag (Implied)
    SED(AddressingMode),
    
    /// `SEI`, Set Interrupt Disable Flag (Implied)
    SEI(AddressingMode),
    
    /// `SEP`, Set Processor Status Bits (Immediate)
    SEP(AddressingMode),
    
    /// `STA`, Store Accumulator to Memory (DP Indexed Indirect,X)
    STA(AddressingMode),
    
    /// `STP`, Stop Processor (Implied)
    STP(AddressingMode),
    
    /// `STX`, Store Index Register X to Memory (Direct Page)
    STX(AddressingMode),
    
    /// `STY`, Store Index Register Y to Memory (Direct Page)
    STY(AddressingMode),
    
    /// `STZ`, Store Zero to Memory (Direct Page)
    STZ(AddressingMode),
    
    /// `TAX`, Transfer Accumulator to Index Register X (Implied)
    TAX(AddressingMode),
    
    /// `TAY`, Transfer Accumulator to Index Register Y (Implied)
    TAY(AddressingMode),
    
    /// `TCD`, Transfer 16-bit Accumulator to Direct Page Register (Implied)
    TCD(AddressingMode),
    
    /// `TCS`, Transfer 16-bit Accumulator to Stack Pointer (Implied)
    TCS(AddressingMode),
    
    /// `TDC`, Transfer Direct Page Register to 16-bit Accumulator (Implied)
    TDC(AddressingMode),
    
    /// `TRB`, Test and Reset Memory Bits Against Accumulator (Direct Page)
    TRB(AddressingMode),
    
    /// `TSB`, Test and Set Memory Bits Against Accumulator (Direct Page)
    TSB(AddressingMode),
    
    /// `TSC`, Transfer Stack Pointer to 16-bit Accumulator (Implied)
    TSC(AddressingMode),
    
    /// `TSX`, Transfer Stack Pointer to Index Register X (Implied)
    TSX(AddressingMode),
    
    /// `TXA`, Transfer Index Register X to Accumulator (Implied)
    TXA(AddressingMode),
    
    /// `TXS`, Transfer Index Register X to Stack Pointer (Implied)
    TXS(AddressingMode),
    
    /// `TXY`, Transfer Index Register X to Index Register Y (Implied)
    TXY(AddressingMode),
    
    /// `TYA`, Transfer Index Register Y to Accumulator (Implied)
    TYA(AddressingMode),
    
    /// `TYX`, Transfer Index Register Y to Index Register X (Implied)
    TYX(AddressingMode),
    
    /// `WAI`, Wait for Interrupt (Implied)
    WAI(AddressingMode),
    
    /// `WDM`, <em>Reserved for Future Expansion</em> ()
    WDM(AddressingMode),
    
    /// `XBA`, Exchange B and A 8-bit Accumulators (Implied)
    XBA(AddressingMode),
    
    /// `XCE`, Exchange Carry and Emulation Flags (Implied)
    XCE(AddressingMode),
}
impl Instruction {
    /// Returns `Instruction` enum variant from op code
    pub fn from_op(op: u8) -> Instruction {
        use Instruction::*;
        match op {
            0x00 => BRK(AddressingMode::Implied),
            0x01 => ORA(AddressingMode::IndirectX),
            0x02 => COP(AddressingMode::Implied),
            0x03 => ORA(AddressingMode::StackRelative),
            0x04 => TSB(AddressingMode::Direct),
            0x05 => ORA(AddressingMode::Direct),
            0x06 => ASL(AddressingMode::Direct),
            0x07 => ORA(AddressingMode::IndirectLong),
            0x08 => PHP(AddressingMode::Implied),
            0x09 => ORA(AddressingMode::Immediate),
            0x0A => ASLA(AddressingMode::Implied),
            0x0B => PHD(AddressingMode::Implied),
            0x0C => TSB(AddressingMode::Absolute),
            0x0D => ORA(AddressingMode::Absolute),
            0x0E => ASL(AddressingMode::Absolute),
            0x0F => ORA(AddressingMode::Long),
            0x10 => BPL(AddressingMode::Relative),
            0x11 => ORA(AddressingMode::IndirectY),
            0x12 => ORA(AddressingMode::Indirect),
            0x13 => ORA(AddressingMode::StackRelativeY),
            0x14 => TRB(AddressingMode::Direct),
            0x15 => ORA(AddressingMode::DirectX),
            0x16 => ASL(AddressingMode::DirectX),
            0x17 => ORA(AddressingMode::IndirectLongY),
            0x18 => CLC(AddressingMode::Implied),
            0x19 => ORA(AddressingMode::AbsoluteY),
            0x1A => INA(AddressingMode::Implied),
            0x1B => TCS(AddressingMode::Implied),
            0x1C => TRB(AddressingMode::Absolute),
            0x1D => ORA(AddressingMode::AbsoluteX),
            0x1E => ASL(AddressingMode::AbsoluteX),
            0x1F => ORA(AddressingMode::LongX),
            0x20 => JSR(AddressingMode::Absolute),
            0x21 => AND(AddressingMode::IndirectX),
            0x22 => JSL(AddressingMode::Long),
            0x23 => AND(AddressingMode::StackRelative),
            0x24 => BIT(AddressingMode::Direct),
            0x25 => AND(AddressingMode::Direct),
            0x26 => ROL(AddressingMode::Direct),
            0x27 => AND(AddressingMode::IndirectLong),
            0x28 => PLP(AddressingMode::Implied),
            0x29 => AND(AddressingMode::Immediate),
            0x2A => ROLA(AddressingMode::Implied),
            0x2B => PLD(AddressingMode::Implied),
            0x2C => BIT(AddressingMode::Absolute),
            0x2D => AND(AddressingMode::Absolute),
            0x2E => ROL(AddressingMode::Absolute),
            0x2F => AND(AddressingMode::Long),
            0x30 => BMI(AddressingMode::Relative),
            0x31 => AND(AddressingMode::IndirectY),
            0x32 => AND(AddressingMode::Indirect),
            0x33 => AND(AddressingMode::StackRelativeY),
            0x34 => BIT(AddressingMode::DirectX),
            0x35 => AND(AddressingMode::DirectX),
            0x36 => ROL(AddressingMode::DirectX),
            0x37 => AND(AddressingMode::IndirectLongY),
            0x38 => SEC(AddressingMode::Implied),
            0x39 => AND(AddressingMode::AbsoluteY),
            0x3A => DEA(AddressingMode::Implied),
            0x3B => TSC(AddressingMode::Implied),
            0x3C => BIT(AddressingMode::AbsoluteX),
            0x3D => AND(AddressingMode::AbsoluteX),
            0x3E => ROL(AddressingMode::AbsoluteX),
            0x3F => AND(AddressingMode::LongX),
            0x40 => RTI(AddressingMode::Implied),
            0x41 => EOR(AddressingMode::IndirectX),
            0x42 => WDM(AddressingMode::Implied),
            0x43 => EOR(AddressingMode::StackRelative),
            0x44 => MVP(AddressingMode::Move),
            0x45 => EOR(AddressingMode::Direct),
            0x46 => LSR(AddressingMode::Direct),
            0x47 => EOR(AddressingMode::IndirectLong),
            0x48 => PHA(AddressingMode::Implied),
            0x49 => EOR(AddressingMode::Immediate),
            0x4A => LSRA(AddressingMode::Implied),
            0x4B => PHK(AddressingMode::Implied),
            0x4C => JMP(AddressingMode::Absolute),
            0x4D => EOR(AddressingMode::Absolute),
            0x4E => LSR(AddressingMode::Absolute),
            0x4F => EOR(AddressingMode::Long),
            0x50 => BVC(AddressingMode::Relative),
            0x51 => EOR(AddressingMode::IndirectY),
            0x52 => EOR(AddressingMode::Indirect),
            0x53 => EOR(AddressingMode::StackRelativeY),
            0x54 => MVN(AddressingMode::Move),
            0x55 => EOR(AddressingMode::DirectX),
            0x56 => LSR(AddressingMode::DirectX),
            0x57 => EOR(AddressingMode::IndirectLongY),
            0x58 => CLI(AddressingMode::Implied),
            0x59 => EOR(AddressingMode::AbsoluteY),
            0x5A => PHY(AddressingMode::Implied),
            0x5B => TCD(AddressingMode::Implied),
            0x5C => JML(AddressingMode::Long),
            0x5D => EOR(AddressingMode::AbsoluteX),
            0x5E => LSR(AddressingMode::AbsoluteX),
            0x5F => EOR(AddressingMode::LongX),
            0x60 => RTS(AddressingMode::Implied),
            0x61 => ADC(AddressingMode::IndirectX),
            0x62 => PER(AddressingMode::Implied),
            0x63 => ADC(AddressingMode::StackRelative),
            0x64 => STZ(AddressingMode::Direct),
            0x65 => ADC(AddressingMode::Direct),
            0x66 => ROR(AddressingMode::Direct),
            0x67 => ADC(AddressingMode::IndirectLong),
            0x68 => PLA(AddressingMode::Implied),
            0x69 => ADC(AddressingMode::Immediate),
            0x6A => RORA(AddressingMode::Implied),
            0x6B => RTL(AddressingMode::Implied),
            0x6C => JMP(AddressingMode::AbsoluteIndirect),
            0x6D => ADC(AddressingMode::Absolute),
            0x6E => ROR(AddressingMode::Absolute),
            0x6F => ADC(AddressingMode::Long),
            0x70 => BVS(AddressingMode::Relative),
            0x71 => ADC(AddressingMode::IndirectY),
            0x72 => ADC(AddressingMode::Indirect),
            0x73 => ADC(AddressingMode::StackRelativeY),
            0x74 => STZ(AddressingMode::DirectX),
            0x75 => ADC(AddressingMode::DirectX),
            0x76 => ROR(AddressingMode::DirectX),
            0x77 => ADC(AddressingMode::IndirectLongY),
            0x78 => SEI(AddressingMode::Implied),
            0x79 => ADC(AddressingMode::AbsoluteY),
            0x7A => PLY(AddressingMode::Implied),
            0x7B => TDC(AddressingMode::Implied),
            0x7C => JMP(AddressingMode::AbsoluteIndirectX),
            0x7D => ADC(AddressingMode::AbsoluteX),
            0x7E => ROR(AddressingMode::AbsoluteX),
            0x7F => ADC(AddressingMode::LongX),
            0x80 => BRA(AddressingMode::Relative),
            0x81 => STA(AddressingMode::IndirectX),
            0x82 => BRL(AddressingMode::RelativeLong),
            0x83 => STA(AddressingMode::StackRelative),
            0x84 => STY(AddressingMode::Direct),
            0x85 => STA(AddressingMode::Direct),
            0x86 => STX(AddressingMode::Direct),
            0x87 => STA(AddressingMode::IndirectLong),
            0x88 => DEY(AddressingMode::Implied),
            0x89 => BIT(AddressingMode::Immediate),
            0x8A => TXA(AddressingMode::Implied),
            0x8B => PHB(AddressingMode::Implied),
            0x8C => STY(AddressingMode::Absolute),
            0x8D => STA(AddressingMode::Absolute),
            0x8E => STX(AddressingMode::Absolute),
            0x8F => STA(AddressingMode::Long),
            0x90 => BLT(AddressingMode::Relative),
            0x91 => STA(AddressingMode::IndirectY),
            0x92 => STA(AddressingMode::Indirect),
            0x93 => STA(AddressingMode::StackRelativeY),
            0x94 => STY(AddressingMode::DirectX),
            0x95 => STA(AddressingMode::DirectX),
            0x96 => STX(AddressingMode::DirectY),
            0x97 => STA(AddressingMode::IndirectLongY),
            0x98 => TYA(AddressingMode::Implied),
            0x99 => STA(AddressingMode::AbsoluteY),
            0x9A => TXS(AddressingMode::Implied),
            0x9B => TXY(AddressingMode::Implied),
            0x9C => STZ(AddressingMode::Absolute),
            0x9D => STA(AddressingMode::AbsoluteX),
            0x9E => STZ(AddressingMode::AbsoluteX),
            0x9F => STA(AddressingMode::LongX),
            0xA0 => LDY(AddressingMode::Immediate),
            0xA1 => LDA(AddressingMode::IndirectX),
            0xA2 => LDX(AddressingMode::Immediate),
            0xA3 => LDA(AddressingMode::StackRelative),
            0xA4 => LDY(AddressingMode::Direct),
            0xA5 => LDA(AddressingMode::Direct),
            0xA6 => LDX(AddressingMode::Direct),
            0xA7 => LDA(AddressingMode::IndirectLong),
            0xA8 => TAY(AddressingMode::Implied),
            0xA9 => LDA(AddressingMode::Immediate),
            0xAA => TAX(AddressingMode::Implied),
            0xAB => PLB(AddressingMode::Implied),
            0xAC => LDY(AddressingMode::Absolute),
            0xAD => LDA(AddressingMode::Absolute),
            0xAE => LDX(AddressingMode::Absolute),
            0xAF => LDA(AddressingMode::Long),
            0xB0 => BGE(AddressingMode::Relative),
            0xB1 => LDA(AddressingMode::IndirectY),
            0xB2 => LDA(AddressingMode::Indirect),
            0xB3 => LDA(AddressingMode::StackRelativeY),
            0xB4 => LDY(AddressingMode::DirectX),
            0xB5 => LDA(AddressingMode::DirectX),
            0xB6 => LDX(AddressingMode::DirectY),
            0xB7 => LDA(AddressingMode::IndirectLongY),
            0xB8 => CLV(AddressingMode::Implied),
            0xB9 => LDA(AddressingMode::AbsoluteY),
            0xBA => TSX(AddressingMode::Implied),
            0xBB => TYX(AddressingMode::Implied),
            0xBC => LDY(AddressingMode::AbsoluteX),
            0xBD => LDA(AddressingMode::AbsoluteX),
            0xBE => LDX(AddressingMode::AbsoluteY),
            0xBF => LDA(AddressingMode::LongX),
            0xC0 => CPY(AddressingMode::Immediate),
            0xC1 => CMP(AddressingMode::IndirectX),
            0xC2 => REP(AddressingMode::Immediate),
            0xC3 => CMP(AddressingMode::StackRelative),
            0xC4 => CPY(AddressingMode::Direct),
            0xC5 => CMP(AddressingMode::Direct),
            0xC6 => DEC(AddressingMode::Direct),
            0xC7 => CMP(AddressingMode::IndirectLong),
            0xC8 => INY(AddressingMode::Implied),
            0xC9 => CMP(AddressingMode::Immediate),
            0xCA => DEX(AddressingMode::Implied),
            0xCB => WAI(AddressingMode::Implied),
            0xCC => CPY(AddressingMode::Absolute),
            0xCD => CMP(AddressingMode::Absolute),
            0xCE => DEC(AddressingMode::Absolute),
            0xCF => CMP(AddressingMode::Long),
            0xD0 => BNE(AddressingMode::Relative),
            0xD1 => CMP(AddressingMode::IndirectY),
            0xD2 => CMP(AddressingMode::Indirect),
            0xD3 => CMP(AddressingMode::StackRelativeY),
            0xD4 => PEI(AddressingMode::Implied),
            0xD5 => CMP(AddressingMode::DirectX),
            0xD6 => DEC(AddressingMode::DirectX),
            0xD7 => CMP(AddressingMode::IndirectLongY),
            0xD8 => CLD(AddressingMode::Implied),
            0xD9 => CMP(AddressingMode::AbsoluteY),
            0xDA => PHX(AddressingMode::Implied),
            0xDB => STP(AddressingMode::Implied),
            0xDC => JML(AddressingMode::AbsoluteIndirectLong),
            0xDD => CMP(AddressingMode::AbsoluteX),
            0xDE => DEC(AddressingMode::AbsoluteX),
            0xDF => CMP(AddressingMode::LongX),
            0xE0 => CPX(AddressingMode::Immediate),
            0xE1 => SBC(AddressingMode::IndirectX),
            0xE2 => SEP(AddressingMode::Immediate),
            0xE3 => SBC(AddressingMode::StackRelative),
            0xE4 => CPX(AddressingMode::Direct),
            0xE5 => SBC(AddressingMode::Direct),
            0xE6 => INC(AddressingMode::Direct),
            0xE7 => SBC(AddressingMode::IndirectLong),
            0xE8 => INX(AddressingMode::Implied),
            0xE9 => SBC(AddressingMode::Immediate),
            0xEA => NOP(AddressingMode::Implied),
            0xEB => XBA(AddressingMode::Implied),
            0xEC => CPX(AddressingMode::Absolute),
            0xED => SBC(AddressingMode::Absolute),
            0xEE => INC(AddressingMode::Absolute),
            0xEF => SBC(AddressingMode::Long),
            0xF0 => BEQ(AddressingMode::Relative),
            0xF1 => SBC(AddressingMode::IndirectY),
            0xF2 => SBC(AddressingMode::Indirect),
            0xF3 => SBC(AddressingMode::StackRelativeY),
            0xF4 => PEA(AddressingMode::Implied),
            0xF5 => SBC(AddressingMode::DirectX),
            0xF6 => INC(AddressingMode::DirectX),
            0xF7 => SBC(AddressingMode::IndirectLongY),
            0xF8 => SED(AddressingMode::Implied),
            0xF9 => SBC(AddressingMode::AbsoluteY),
            0xFA => PLX(AddressingMode::Implied),
            0xFB => XCE(AddressingMode::Implied),
            0xFC => JSR(AddressingMode::AbsoluteIndirectX),
            0xFD => SBC(AddressingMode::AbsoluteX),
            0xFE => INC(AddressingMode::AbsoluteX),
            0xFF => SBC(AddressingMode::LongX),
            
            
        }
    }
}
