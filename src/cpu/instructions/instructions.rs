pub enum Instruction {
    /// Add With Carry (DP Indexed Indirect,X)
    AdcDPIX,
    /// Add With Carry (Stack Relative)
    AdcSR,
    /// Add With Carry (Direct Page)
    AdcDP,
    /// Add With Carry (DP Indirect Long)
    AdcDPLong,
    /// Add With Carry (Immediate)
    AdcImm,
    /// Add With Carry (Absolute)
    AdcAbs,
    /// Add With Carry (Absolute Long)
    AdcAbsLong,
    /// Add With Carry (DP Indirect Indexed, Y)
    AdcDPIY,
    /// Add With Carry (DP Indirect)
    AdcDPI,
    /// Add With Carry (SR Indirect Indexed,Y)
    AdcSRY,
    /// Add With Carry (DP Indexed,X)
    AdcDPX,
    /// Add With Carry (DP Indirect Long Indexed, Y)
    AdcDPIYLong,
    /// Add With Carry (Absolute Indexed,Y)
    AdcAbsY,
    /// Add With Carry (Absolute Indexed,X)
    AdcAbsX,
    /// Add With Carry (Absolute Long Indexed,X)
    AdcAbsXLong,
    /// AND Accumulator with Memory (DP Indexed Indirect,X)
    AndDPIX,
    /// AND Accumulator with Memory (Stack Relative)
    AndSR,
    /// AND Accumulator with Memory (Direct Page)
    AndDP,
    /// AND Accumulator with Memory (DP Indirect Long)
    AndDPLong,
    /// AND Accumulator with Memory (Immediate)
    AndImm,
    /// AND Accumulator with Memory (Absolute)
    AndAbs,
    /// AND Accumulator with Memory (Absolute Long)
    AndAbsLong,
    /// AND Accumulator with Memory (DP Indirect Indexed, Y)
    AndDPIY,
    /// AND Accumulator with Memory (DP Indirect)
    AndDPI,
    /// AND Accumulator with Memory (SR Indirect Indexed,Y)
    AndSRY,
    /// AND Accumulator with Memory (DP Indexed,X)
    AndDPX,
    /// AND Accumulator with Memory (DP Indirect Long Indexed, Y)
    AndDPIYLong,
    /// AND Accumulator with Memory (Absolute Indexed,Y)
    AndAbsY,
    /// AND Accumulator with Memory (Absolute Indexed,X)
    AndAbsX,
    /// AND Accumulator with Memory (Absolute Long Indexed,X)
    AndAbsXLong,
    /// Arithmetic Shift Left (Direct Page)
    AslDP,
    /// Arithmetic Shift Left (Accumulator)
    Asl,
    /// Arithmetic Shift Left (Absolute)
    AslAbs,
    /// Arithmetic Shift Left (DP Indexed,X)
    AslDPX,
    /// Arithmetic Shift Left (Absolute Indexed,X)
    AslAbsX,
    /// Branch if Carry Clear (Program Counter Relative)
    Blt,
    /// Branch if Carry Set (Program Counter Relative)
    Bge,
    /// Branch if Equal (Program Counter Relative)
    Beq,
    /// Test Bits (Direct Page)
    BitDP,
    /// Test Bits (Absolute)
    BitAbs,
    /// Test Bits (DP Indexed,X)
    BitDPX,
    /// Test Bits (Absolute Indexed,X)
    BitAbsX,
    /// Test Bits (Immediate)
    BitImm,
    /// Branch if Minus (Program Counter Relative)
    Bmi,
    /// Branch if Not Equal (Program Counter Relative)
    Bne,
    /// Branch if Plus (Program Counter Relative)
    Bpl,
    /// Branch Always (Program Counter Relative)
    Bra,
    /// Break (Stack/Interrupt)
    Brk,
    /// Branch Long Always (Program Counter Relative Long)
    Brl,
    /// Branch if Overflow Clear (Program Counter Relative)
    Bvc,
    /// Branch if Overflow Set (Program Counter Relative)
    Bvs,
    /// Clear Carry (Implied)
    Clc,
    /// Clear Decimal Mode Flag (Implied)
    Cld,
    /// Clear Interrupt Disable Flag (Implied)
    Cli,
    /// Clear Overflow Flag (Implied)
    Clv,
    /// Compare Accumulator with Memory (DP Indexed Indirect,X)
    CmpDPIX,
    /// Compare Accumulator with Memory (Stack Relative)
    CmpSR,
    /// Compare Accumulator with Memory (Direct Page)
    CmpDP,
    /// Compare Accumulator with Memory (DP Indirect Long)
    CmpDPLong,
    /// Compare Accumulator with Memory (Immediate)
    CmpImm,
    /// Compare Accumulator with Memory (Absolute)
    CmpAbs,
    /// Compare Accumulator with Memory (Absolute Long)
    CmpAbsLong,
    /// Compare Accumulator with Memory (DP Indirect Indexed, Y)
    CmpDPIY,
    /// Compare Accumulator with Memory (DP Indirect)
    CmpDPI,
    /// Compare Accumulator with Memory (SR Indirect Indexed,Y)
    CmpSRY,
    /// Compare Accumulator with Memory (DP Indexed,X)
    CmpDPX,
    /// Compare Accumulator with Memory (DP Indirect Long Indexed, Y)
    CmpDPIYLong,
    /// Compare Accumulator with Memory (Absolute Indexed,Y)
    CmpAbsY,
    /// Compare Accumulator with Memory (Absolute Indexed,X)
    CmpAbsX,
    /// Compare Accumulator with Memory (Absolute Long Indexed,X)
    CmpAbsXLong,
    /// Co-Processor (Stack/Interrupt)
    Cop,
    /// Compare Index Register X with Memory (Immediate)
    CpxImm,
    /// Compare Index Register X with Memory (Direct Page)
    CpxDP,
    /// Compare Index Register X with Memory (Absolute)
    CpxAbs,
    /// Compare Index Register Y with Memory (Immediate)
    CpyImm,
    /// Compare Index Register Y with Memory (Direct Page)
    CpyDP,
    /// Compare Index Register Y with Memory (Absolute)
    CpyAbs,
    /// Decrement (Accumulator)
    Dea,
    /// Decrement (Direct Page)
    DecDP,
    /// Decrement (Absolute)
    DecAbs,
    /// Decrement (DP Indexed,X)
    DecDPX,
    /// Decrement (Absolute Indexed,X)
    DecAbsX,
    /// Decrement Index Register X (Implied)
    Dex,
    /// Decrement Index Register Y (Implied)
    Dey,
    /// Exclusive-OR Accumulator with Memory (DP Indexed Indirect,X)
    EorDPIX,
    /// Exclusive-OR Accumulator with Memory (Stack Relative)
    EorSR,
    /// Exclusive-OR Accumulator with Memory (Direct Page)
    EorDP,
    /// Exclusive-OR Accumulator with Memory (DP Indirect Long)
    EorDPLong,
    /// Exclusive-OR Accumulator with Memory (Immediate)
    EorImm,
    /// Exclusive-OR Accumulator with Memory (Absolute)
    EorAbs,
    /// Exclusive-OR Accumulator with Memory (Absolute Long)
    EorAbsLong,
    /// Exclusive-OR Accumulator with Memory (DP Indirect Indexed, Y)
    EorDPIY,
    /// Exclusive-OR Accumulator with Memory (DP Indirect)
    EorDPI,
    /// Exclusive-OR Accumulator with Memory (SR Indirect Indexed,Y)
    EorSRY,
    /// Exclusive-OR Accumulator with Memory (DP Indexed,X)
    EorDPX,
    /// Exclusive-OR Accumulator with Memory (DP Indirect Long Indexed, Y)
    EorDPIYLong,
    /// Exclusive-OR Accumulator with Memory (Absolute Indexed,Y)
    EorAbsY,
    /// Exclusive-OR Accumulator with Memory (Absolute Indexed,X)
    EorAbsX,
    /// Exclusive-OR Accumulator with Memory (Absolute Long Indexed,X)
    EorAbsXLong,
    /// Increment (Accumulator)
    Ina,
    /// Increment (Direct Page)
    IncDP,
    /// Increment (Absolute)
    IncAbs,
    /// Increment (DP Indexed,X)
    IncDPX,
    /// Increment (Absolute Indexed,X)
    IncAbsX,
    /// Increment Index Register X (Implied)
    Inx,
    /// Increment Index Register Y (Implied)
    Iny,
    /// Jump (Absolute)
    JmpAbs,
    /// Jump (Absolute Long)
    JmlAbsLong,
    /// Jump (Absolute Indirect)
    Jmp,
    /// Jump (Absolute Indexed Indirect)
    JmpAbsIX,
    /// Jump (Absolute Indirect Long)
    Jml,
    /// Jump to Subroutine (Absolute)
    JsrAbs,
    /// Jump to Subroutine (Absolute Long)
    JslAbsLong,
    /// Jump to Subroutine (Absolute Indexed Indirect)
    JsrAbsIX,
    /// Load Accumulator from Memory (DP Indexed Indirect,X)
    LdaDPIX,
    /// Load Accumulator from Memory (Stack Relative)
    LdaSR,
    /// Load Accumulator from Memory (Direct Page)
    LdaDP,
    /// Load Accumulator from Memory (DP Indirect Long)
    LdaDPLong,
    /// Load Accumulator from Memory (Immediate)
    LdaImm,
    /// Load Accumulator from Memory (Absolute)
    LdaAbs,
    /// Load Accumulator from Memory (Absolute Long)
    LdaAbsLong,
    /// Load Accumulator from Memory (DP Indirect Indexed, Y)
    LdaDPIY,
    /// Load Accumulator from Memory (DP Indirect)
    LdaDPI,
    /// Load Accumulator from Memory (SR Indirect Indexed,Y)
    LdaSRY,
    /// Load Accumulator from Memory (DP Indexed,X)
    LdaDPX,
    /// Load Accumulator from Memory (DP Indirect Long Indexed, Y)
    LdaDPIYLong,
    /// Load Accumulator from Memory (Absolute Indexed,Y)
    LdaAbsY,
    /// Load Accumulator from Memory (Absolute Indexed,X)
    LdaAbsX,
    /// Load Accumulator from Memory (Absolute Long Indexed,X)
    LdaAbsXLong,
    /// Load Index Register X from Memory (Immediate)
    LdxImm,
    /// Load Index Register X from Memory (Direct Page)
    LdxDP,
    /// Load Index Register X from Memory (Absolute)
    LdxAbs,
    /// Load Index Register X from Memory (DP Indexed,Y)
    LdxDPY,
    /// Load Index Register X from Memory (Absolute Indexed,Y)
    LdxAbsY,
    /// Load Index Register Y from Memory (Immediate)
    LdyImm,
    /// Load Index Register Y from Memory (Direct Page)
    LdyDP,
    /// Load Index Register Y from Memory (Absolute)
    LdyAbs,
    /// Load Index Register Y from Memory (DP Indexed,X)
    LdyDPX,
    /// Load Index Register Y from Memory (Absolute Indexed,X)
    LdyAbsX,
    /// Logical Shift Memory or Accumulator Right (Direct Page)
    LsrDP,
    /// Logical Shift Memory or Accumulator Right (Accumulator)
    Lsr,
    /// Logical Shift Memory or Accumulator Right (Absolute)
    LsrAbs,
    /// Logical Shift Memory or Accumulator Right (DP Indexed,X)
    LsrDPX,
    /// Logical Shift Memory or Accumulator Right (Absolute Indexed,X)
    LsrAbsX,
    /// Block Move Negative (Block Move)
    Mvn,
    /// Block Move Positive (Block Move)
    Mvp,
    /// No Operation (Implied)
    Nop,
    /// OR Accumulator with Memory (DP Indexed Indirect,X)
    OraDPIX,
    /// OR Accumulator with Memory (Stack Relative)
    OraSR,
    /// OR Accumulator with Memory (Direct Page)
    OraDP,
    /// OR Accumulator with Memory (DP Indirect Long)
    OraDPLong,
    /// OR Accumulator with Memory (Immediate)
    OraImm,
    /// OR Accumulator with Memory (Absolute)
    OraAbs,
    /// OR Accumulator with Memory (Absolute Long)
    OraAbsLong,
    /// OR Accumulator with Memory (DP Indirect Indexed, Y)
    OraDPIY,
    /// OR Accumulator with Memory (DP Indirect)
    OraDPI,
    /// OR Accumulator with Memory (SR Indirect Indexed,Y)
    OraSRY,
    /// OR Accumulator with Memory (DP Indexed,X)
    OraDPX,
    /// OR Accumulator with Memory (DP Indirect Long Indexed, Y)
    OraDPIYLong,
    /// OR Accumulator with Memory (Absolute Indexed,Y)
    OraAbsY,
    /// OR Accumulator with Memory (Absolute Indexed,X)
    OraAbsX,
    /// OR Accumulator with Memory (Absolute Long Indexed,X)
    OraAbsXLong,
    /// Push Effective Absolute Address (Stack (Absolute))
    Pea,
    /// Push Effective Indirect Address (Stack (DP Indirect))
    Pei,
    /// Push Effective PC Relative Indirect Address (Stack (PC Relative Long))
    Per,
    /// Push Accumulator (Stack (Push))
    Pha,
    /// Push Data Bank Register (Stack (Push))
    Phb,
    /// Push Direct Page Register (Stack (Push))
    Phd,
    /// Push Program Bank Register (Stack (Push))
    Phk,
    /// Push Processor Status Register (Stack (Push))
    Php,
    /// Push Index Register X (Stack (Push))
    Phx,
    /// Push Index Register Y (Stack (Push))
    Phy,
    /// Pull Accumulator (Stack (Pull))
    Pla,
    /// Pull Data Bank Register (Stack (Pull))
    Plb,
    /// Pull Direct Page Register (Stack (Pull))
    Pld,
    /// Pull Processor Status Register (Stack (Pull))
    Plp,
    /// Pull Index Register X (Stack (Pull))
    Plx,
    /// Pull Index Register Y (Stack (Pull))
    Ply,
    /// Reset Processor Status Bits (Immediate)
    RepImm,
    /// Rotate Memory or Accumulator Left (Direct Page)
    RolDP,
    /// Rotate Memory or Accumulator Left (Accumulator)
    Rol,
    /// Rotate Memory or Accumulator Left (Absolute)
    RolAbs,
    /// Rotate Memory or Accumulator Left (DP Indexed,X)
    RolDPX,
    /// Rotate Memory or Accumulator Left (Absolute Indexed,X)
    RolAbsX,
    /// Rotate Memory or Accumulator Right (Direct Page)
    RorDP,
    /// Rotate Memory or Accumulator Right (Accumulator)
    Ror,
    /// Rotate Memory or Accumulator Right (Absolute)
    RorAbs,
    /// Rotate Memory or Accumulator Right (DP Indexed,X)
    RorDPX,
    /// Rotate Memory or Accumulator Right (Absolute Indexed,X)
    RorAbsX,
    /// Return from Interrupt (Stack (RTI))
    Rti,
    /// Return from Subroutine Long (Stack (RTL))
    Rtl,
    /// Return from Subroutine (Stack (RTS))
    Rts,
    /// Subtract with Borrow from Accumulator (DP Indexed Indirect,X)
    SbcDPIX,
    /// Subtract with Borrow from Accumulator (Stack Relative)
    SbcSR,
    /// Subtract with Borrow from Accumulator (Direct Page)
    SbcDP,
    /// Subtract with Borrow from Accumulator (DP Indirect Long)
    SbcDPLong,
    /// Subtract with Borrow from Accumulator (Immediate)
    SbcImm,
    /// Subtract with Borrow from Accumulator (Absolute)
    SbcAbs,
    /// Subtract with Borrow from Accumulator (Absolute Long)
    SbcAbsLong,
    /// Subtract with Borrow from Accumulator (DP Indirect Indexed, Y)
    SbcDPIY,
    /// Subtract with Borrow from Accumulator (DP Indirect)
    SbcDPI,
    /// Subtract with Borrow from Accumulator (SR Indirect Indexed,Y)
    SbcSRY,
    /// Subtract with Borrow from Accumulator (DP Indexed,X)
    SbcDPX,
    /// Subtract with Borrow from Accumulator (DP Indirect Long Indexed, Y)
    SbcDPIYLong,
    /// Subtract with Borrow from Accumulator (Absolute Indexed,Y)
    SbcAbsY,
    /// Subtract with Borrow from Accumulator (Absolute Indexed,X)
    SbcAbsX,
    /// Subtract with Borrow from Accumulator (Absolute Long Indexed,X)
    SbcAbsXLong,
    /// Set Carry Flag (Implied)
    Sec,
    /// Set Decimal Flag (Implied)
    Sed,
    /// Set Interrupt Disable Flag (Implied)
    Sei,
    /// Set Processor Status Bits (Immediate)
    SepImm,
    /// Store Accumulator to Memory (DP Indexed Indirect,X)
    StaDPIX,
    /// Store Accumulator to Memory (Stack Relative)
    StaSR,
    /// Store Accumulator to Memory (Direct Page)
    StaDP,
    /// Store Accumulator to Memory (DP Indirect Long)
    StaDPLong,
    /// Store Accumulator to Memory (Absolute)
    StaAbs,
    /// Store Accumulator to Memory (Absolute Long)
    StaAbsLong,
    /// Store Accumulator to Memory (DP Indirect Indexed, Y)
    StaDPIY,
    /// Store Accumulator to Memory (DP Indirect)
    StaDPI,
    /// Store Accumulator to Memory (SR Indirect Indexed,Y)
    StaSRY,
    /// Store Accumulator to Memory (DP Indexed,X)
    StaDPX,
    /// Store Accumulator to Memory (DP Indirect Long Indexed, Y)
    StaDPIYLong,
    /// Store Accumulator to Memory (Absolute Indexed,Y)
    StaAbsY,
    /// Store Accumulator to Memory (Absolute Indexed,X)
    StaAbsX,
    /// Store Accumulator to Memory (Absolute Long Indexed,X)
    StaAbsXLong,
    /// Stop Processor (Implied)
    Stp,
    /// Store Index Register X to Memory (Direct Page)
    StxDP,
    /// Store Index Register X to Memory (Absolute)
    StxAbs,
    /// Store Index Register X to Memory (DP Indexed,Y)
    StxDPY,
    /// Store Index Register Y to Memory (Direct Page)
    StyDP,
    /// Store Index Register Y to Memory (Absolute)
    StyAbs,
    /// Store Index Register Y to Memory (DP Indexed,X)
    StyDPX,
    /// Store Zero to Memory (Direct Page)
    StzDP,
    /// Store Zero to Memory (DP Indexed,X)
    StzDPX,
    /// Store Zero to Memory (Absolute)
    StzAbs,
    /// Store Zero to Memory (Absolute Indexed,X)
    StzAbsX,
    /// Transfer Accumulator to Index Register X (Implied)
    Tax,
    /// Transfer Accumulator to Index Register Y (Implied)
    Tay,
    /// Transfer 16-bit Accumulator to Direct Page Register (Implied)
    Tcd,
    /// Transfer 16-bit Accumulator to Stack Pointer (Implied)
    Tcs,
    /// Transfer Direct Page Register to 16-bit Accumulator (Implied)
    Tdc,
    /// Test and Reset Memory Bits Against Accumulator (Direct Page)
    TrbDP,
    /// Test and Reset Memory Bits Against Accumulator (Absolute)
    TrbAbs,
    /// Test and Set Memory Bits Against Accumulator (Direct Page)
    TsbDP,
    /// Test and Set Memory Bits Against Accumulator (Absolute)
    TsbAbs,
    /// Transfer Stack Pointer to 16-bit Accumulator (Implied)
    Tsc,
    /// Transfer Stack Pointer to Index Register X (Implied)
    Tsx,
    /// Transfer Index Register X to Accumulator (Implied)
    Txa,
    /// Transfer Index Register X to Stack Pointer (Implied)
    Txs,
    /// Transfer Index Register X to Index Register Y (Implied)
    Txy,
    /// Transfer Index Register Y to Accumulator (Implied)
    Tya,
    /// Transfer Index Register Y to Index Register X (Implied)
    Tyx,
    /// Wait for Interrupt (Implied)
    Wai,
    /// <em>Reserved for Future Expansion</em> ()
    Wdm,
    /// Exchange B and A 8-bit Accumulators (Implied)
    Xba,
    /// Exchange Carry and Emulation Flags (Implied)
    Xce,
}

/// Returns `Instruction` enum variant from op code
pub fn instruction_from_opcode(op: u8) -> Instruction {
    match op {
        0x61 => Instruction::AdcDPIX,
        0x63 => Instruction::AdcSR,
        0x65 => Instruction::AdcDP,
        0x67 => Instruction::AdcDPLong,
        0x69 => Instruction::AdcImm,
        0x6D => Instruction::AdcAbs,
        0x6F => Instruction::AdcAbsLong,
        0x71 => Instruction::AdcDPIY,
        0x72 => Instruction::AdcDPI,
        0x73 => Instruction::AdcSRY,
        0x75 => Instruction::AdcDPX,
        0x77 => Instruction::AdcDPIYLong,
        0x79 => Instruction::AdcAbsY,
        0x7D => Instruction::AdcAbsX,
        0x7F => Instruction::AdcAbsXLong,
        0x21 => Instruction::AndDPIX,
        0x23 => Instruction::AndSR,
        0x25 => Instruction::AndDP,
        0x27 => Instruction::AndDPLong,
        0x29 => Instruction::AndImm,
        0x2D => Instruction::AndAbs,
        0x2F => Instruction::AndAbsLong,
        0x31 => Instruction::AndDPIY,
        0x32 => Instruction::AndDPI,
        0x33 => Instruction::AndSRY,
        0x35 => Instruction::AndDPX,
        0x37 => Instruction::AndDPIYLong,
        0x39 => Instruction::AndAbsY,
        0x3D => Instruction::AndAbsX,
        0x3F => Instruction::AndAbsXLong,
        0x06 => Instruction::AslDP,
        0x0A => Instruction::Asl,
        0x0E => Instruction::AslAbs,
        0x16 => Instruction::AslDPX,
        0x1E => Instruction::AslAbsX,
        0x90 => Instruction::Blt,
        0xB0 => Instruction::Bge,
        0xF0 => Instruction::Beq,
        0x24 => Instruction::BitDP,
        0x2C => Instruction::BitAbs,
        0x34 => Instruction::BitDPX,
        0x3C => Instruction::BitAbsX,
        0x89 => Instruction::BitImm,
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
        0xC1 => Instruction::CmpDPIX,
        0xC3 => Instruction::CmpSR,
        0xC5 => Instruction::CmpDP,
        0xC7 => Instruction::CmpDPLong,
        0xC9 => Instruction::CmpImm,
        0xCD => Instruction::CmpAbs,
        0xCF => Instruction::CmpAbsLong,
        0xD1 => Instruction::CmpDPIY,
        0xD2 => Instruction::CmpDPI,
        0xD3 => Instruction::CmpSRY,
        0xD5 => Instruction::CmpDPX,
        0xD7 => Instruction::CmpDPIYLong,
        0xD9 => Instruction::CmpAbsY,
        0xDD => Instruction::CmpAbsX,
        0xDF => Instruction::CmpAbsXLong,
        0x02 => Instruction::Cop,
        0xE0 => Instruction::CpxImm,
        0xE4 => Instruction::CpxDP,
        0xEC => Instruction::CpxAbs,
        0xC0 => Instruction::CpyImm,
        0xC4 => Instruction::CpyDP,
        0xCC => Instruction::CpyAbs,
        0x3A => Instruction::Dea,
        0xC6 => Instruction::DecDP,
        0xCE => Instruction::DecAbs,
        0xD6 => Instruction::DecDPX,
        0xDE => Instruction::DecAbsX,
        0xCA => Instruction::Dex,
        0x88 => Instruction::Dey,
        0x41 => Instruction::EorDPIX,
        0x43 => Instruction::EorSR,
        0x45 => Instruction::EorDP,
        0x47 => Instruction::EorDPLong,
        0x49 => Instruction::EorImm,
        0x4D => Instruction::EorAbs,
        0x4F => Instruction::EorAbsLong,
        0x51 => Instruction::EorDPIY,
        0x52 => Instruction::EorDPI,
        0x53 => Instruction::EorSRY,
        0x55 => Instruction::EorDPX,
        0x57 => Instruction::EorDPIYLong,
        0x59 => Instruction::EorAbsY,
        0x5D => Instruction::EorAbsX,
        0x5F => Instruction::EorAbsXLong,
        0x1A => Instruction::Ina,
        0xE6 => Instruction::IncDP,
        0xEE => Instruction::IncAbs,
        0xF6 => Instruction::IncDPX,
        0xFE => Instruction::IncAbsX,
        0xE8 => Instruction::Inx,
        0xC8 => Instruction::Iny,
        0x4C => Instruction::JmpAbs,
        0x5C => Instruction::JmlAbsLong,
        0x6C => Instruction::Jmp,
        0x7C => Instruction::JmpAbsIX,
        0xDC => Instruction::Jml,
        0x20 => Instruction::JsrAbs,
        0x22 => Instruction::JslAbsLong,
        0xFC => Instruction::JsrAbsIX,
        0xA1 => Instruction::LdaDPIX,
        0xA3 => Instruction::LdaSR,
        0xA5 => Instruction::LdaDP,
        0xA7 => Instruction::LdaDPLong,
        0xA9 => Instruction::LdaImm,
        0xAD => Instruction::LdaAbs,
        0xAF => Instruction::LdaAbsLong,
        0xB1 => Instruction::LdaDPIY,
        0xB2 => Instruction::LdaDPI,
        0xB3 => Instruction::LdaSRY,
        0xB5 => Instruction::LdaDPX,
        0xB7 => Instruction::LdaDPIYLong,
        0xB9 => Instruction::LdaAbsY,
        0xBD => Instruction::LdaAbsX,
        0xBF => Instruction::LdaAbsXLong,
        0xA2 => Instruction::LdxImm,
        0xA6 => Instruction::LdxDP,
        0xAE => Instruction::LdxAbs,
        0xB6 => Instruction::LdxDPY,
        0xBE => Instruction::LdxAbsY,
        0xA0 => Instruction::LdyImm,
        0xA4 => Instruction::LdyDP,
        0xAC => Instruction::LdyAbs,
        0xB4 => Instruction::LdyDPX,
        0xBC => Instruction::LdyAbsX,
        0x46 => Instruction::LsrDP,
        0x4A => Instruction::Lsr,
        0x4E => Instruction::LsrAbs,
        0x56 => Instruction::LsrDPX,
        0x5E => Instruction::LsrAbsX,
        0x54 => Instruction::Mvn,
        0x44 => Instruction::Mvp,
        0xEA => Instruction::Nop,
        0x01 => Instruction::OraDPIX,
        0x03 => Instruction::OraSR,
        0x05 => Instruction::OraDP,
        0x07 => Instruction::OraDPLong,
        0x09 => Instruction::OraImm,
        0x0D => Instruction::OraAbs,
        0x0F => Instruction::OraAbsLong,
        0x11 => Instruction::OraDPIY,
        0x12 => Instruction::OraDPI,
        0x13 => Instruction::OraSRY,
        0x15 => Instruction::OraDPX,
        0x17 => Instruction::OraDPIYLong,
        0x19 => Instruction::OraAbsY,
        0x1D => Instruction::OraAbsX,
        0x1F => Instruction::OraAbsXLong,
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
        0xC2 => Instruction::RepImm,
        0x26 => Instruction::RolDP,
        0x2A => Instruction::Rol,
        0x2E => Instruction::RolAbs,
        0x36 => Instruction::RolDPX,
        0x3E => Instruction::RolAbsX,
        0x66 => Instruction::RorDP,
        0x6A => Instruction::Ror,
        0x6E => Instruction::RorAbs,
        0x76 => Instruction::RorDPX,
        0x7E => Instruction::RorAbsX,
        0x40 => Instruction::Rti,
        0x6B => Instruction::Rtl,
        0x60 => Instruction::Rts,
        0xE1 => Instruction::SbcDPIX,
        0xE3 => Instruction::SbcSR,
        0xE5 => Instruction::SbcDP,
        0xE7 => Instruction::SbcDPLong,
        0xE9 => Instruction::SbcImm,
        0xED => Instruction::SbcAbs,
        0xEF => Instruction::SbcAbsLong,
        0xF1 => Instruction::SbcDPIY,
        0xF2 => Instruction::SbcDPI,
        0xF3 => Instruction::SbcSRY,
        0xF5 => Instruction::SbcDPX,
        0xF7 => Instruction::SbcDPIYLong,
        0xF9 => Instruction::SbcAbsY,
        0xFD => Instruction::SbcAbsX,
        0xFF => Instruction::SbcAbsXLong,
        0x38 => Instruction::Sec,
        0xF8 => Instruction::Sed,
        0x78 => Instruction::Sei,
        0xE2 => Instruction::SepImm,
        0x81 => Instruction::StaDPIX,
        0x83 => Instruction::StaSR,
        0x85 => Instruction::StaDP,
        0x87 => Instruction::StaDPLong,
        0x8D => Instruction::StaAbs,
        0x8F => Instruction::StaAbsLong,
        0x91 => Instruction::StaDPIY,
        0x92 => Instruction::StaDPI,
        0x93 => Instruction::StaSRY,
        0x95 => Instruction::StaDPX,
        0x97 => Instruction::StaDPIYLong,
        0x99 => Instruction::StaAbsY,
        0x9D => Instruction::StaAbsX,
        0x9F => Instruction::StaAbsXLong,
        0xDB => Instruction::Stp,
        0x86 => Instruction::StxDP,
        0x8E => Instruction::StxAbs,
        0x96 => Instruction::StxDPY,
        0x84 => Instruction::StyDP,
        0x8C => Instruction::StyAbs,
        0x94 => Instruction::StyDPX,
        0x64 => Instruction::StzDP,
        0x74 => Instruction::StzDPX,
        0x9C => Instruction::StzAbs,
        0x9E => Instruction::StzAbsX,
        0xAA => Instruction::Tax,
        0xA8 => Instruction::Tay,
        0x5B => Instruction::Tcd,
        0x1B => Instruction::Tcs,
        0x7B => Instruction::Tdc,
        0x14 => Instruction::TrbDP,
        0x1C => Instruction::TrbAbs,
        0x04 => Instruction::TsbDP,
        0x0C => Instruction::TsbAbs,
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
