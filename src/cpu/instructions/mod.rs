use self::instructions::Instruction;

pub mod arithmetic;
pub mod r#move;
pub mod branch;
pub mod flag;
pub mod interrupt;
pub mod jump;
pub mod loadstore;
pub mod logical;
pub mod other;
pub mod stack;
pub mod transfer;

pub mod instructions;

#[derive(Debug, Clone, Copy)]
pub enum AddressingMode {
    /// Argument is absolute 2 bytes
    Absolute,
    /// Argument is absolute 2 bytes, indexed with X
    AbsoluteX,
    /// Argument is absolute 2 bytes, indexed with Y
    AbsoluteY,
    /// Argument is memory address specified with 2 bytes (same bank)
    AbsoluteIndirect,
    AbsoluteIndirectX,
    /// Argument is memory address specified with 3 bytes
    AbsoluteIndirectLong,
    /// Argument is memory address is zero-based memory address (ie `LDA $XX` would read `$0000XX`)
    Direct,
    /// Argument is memory address is zero-based memory address (ie `LDA $XX` would read `$0000XX`) indexed by X
    DirectX,
    /// Argument is memory address is zero-based memory address (ie `LDA $XX` would read `$0000XX`) indexed by Y
    DirectY,
    /// Argument is implied
    Implied,
    /// Argument is 1 byte constant
    Immediate,
    /// Argument is memory address that is inside memory address specified by argument (ie `LDA $XX` would read memory address in `$0000XX`)
    Indirect,
    /// Argument is memory address that is inside memory address specified by argument (ie `LDA $XX` would read memory address in `$0000XX`) indexed by X
    IndirectX,
    /// Argument is memory address that is inside memory address specified by argument (ie `LDA $XX` would read memory address in `$0000XX`) indexed by Y
    IndirectY,
    IndirectLong,
    IndirectLongY,
    /// Argument is absolute 3 bytes
    Long,
    /// Argument is absolute 3 bytes, indexed with X
    LongX,
    /// Argument is 1 byte address relative to pc
    Relative,
    /// Argument is 2 byte address relative to pc
    RelativeLong,
    /// Stack is used as lower 2 bytes of address, argument is address relative to stack. Eg, stack pointer is `$1FF0` then `LDA $05, s` loads `$001FF5`
    StackRelative,
    /// Stack is used as lower 2 bytes of address, argument is address relative to stack. Eg, stack pointer is `$1FF0` then `LDA $05, s` loads `$001FF5` plus Y
    StackRelativeY,
    /// Used only for `MVN` and `MVP` instructions, explained in those implementations in more detail
    Move,
}

impl Instruction {
    /// Length of this instruction in bytes, includes op code and all arguments
    /// 
    /// `ADC #$2133` would have a length of 3 for example
    pub fn get_length(&self) -> usize {
        use Instruction::*;
        use AddressingMode::*;
        
        match self.get_addressing_mode() {
            Implied => 1,
            
            Direct | DirectX | DirectY |
            Immediate |
            Indirect | IndirectX | IndirectY |
            IndirectLong | IndirectLongY |
            Relative | StackRelative | StackRelativeY => 2,
            
            Absolute | AbsoluteX | AbsoluteY |
            AbsoluteIndirect | AbsoluteIndirectX |
            AbsoluteIndirectLong | RelativeLong |
            Move => 3,
            
            Long | LongX => 4,
        }
    }
    
    /// Get cycle duration of this instruction. This cycle time is then added to the cpu after instruction completes to sync with ie PPU
    pub fn get_cycle_time(&self) -> usize {
        use Instruction::*;
        use AddressingMode::*;
        match self {
            WDM(Implied)  => 0,
            
            MVN(Move) | MVP(Move)  => 1,
            
            ADC(Immediate) | AND(Immediate) | ASL(Implied) | BEQ(Relative) | BGE(Relative) | BIT(Immediate) |
            BLT(Relative) | BMI(Relative) | BNE(Relative) | BPL(Relative) | BVC(Relative) |
            BVS(Relative) | CLC(Implied) | CLD(Implied) | CLI(Implied) | CLV(Implied) |
            CMP(Immediate) | CPX(Immediate) | CPY(Immediate) | DEA(Implied) | DEX(Implied) |
            DEY(Implied) | EOR(Immediate) | INA(Implied) | INX(Implied) | INY(Implied) |
            LDA(Immediate) | LDX(Immediate) | LDY(Immediate) | LSR(Implied) | NOP(Implied) |
            ORA(Immediate) | ROL(Implied) | ROR(Implied) | SBC(Immediate) | SEC(Implied) |
            SED(Implied) | SEI(Implied) | TAX(Implied) | TAY(Implied) | TCD(Implied) |
            TCS(Implied) | TDC(Implied) | TSC(Implied) | TSX(Implied) | TXA(Implied) |
            TXS(Implied) | TXY(Implied) | TYA(Implied) | TYX(Implied) | XCE(Implied)  => 2,
            
            ADC(Direct) | AND(Direct) | BIT(Direct) | BRA(Relative) | CMP(Direct) | CPX(Direct) |
            CPY(Direct) | EOR(Direct) | JMP(Absolute) | LDA(Direct) | LDX(Direct) |
            LDY(Direct) | ORA(Direct) | PHA(Implied) | PHB(Implied) | PHK(Implied) |
            PHP(Implied) | PHX(Implied) | PHY(Implied) | REP(Immediate) | SBC(Direct) |
            SEP(Immediate) | STA(Direct) | STP(Implied) | STX(Direct) | STY(Direct) |
            STZ(Direct) | WAI(Implied) | XBA(Implied)  => 3,
            
            ADC(StackRelative) | ADC(Absolute) | ADC(DirectX) | ADC(AbsoluteY) | ADC(AbsoluteX) | AND(StackRelative) |
            AND(Absolute) | AND(DirectX) | AND(AbsoluteY) | AND(AbsoluteX) | BIT(Absolute) |
            BIT(DirectX) | BIT(AbsoluteX) | BRL(RelativeLong) | CMP(StackRelative) | CMP(Absolute) |
            CMP(DirectX) | CMP(AbsoluteY) | CMP(AbsoluteX) | CPX(Absolute) | CPY(Absolute) |
            EOR(StackRelative) | EOR(Absolute) | EOR(DirectX) | EOR(AbsoluteY) | EOR(AbsoluteX) |
            JML(Long) | LDA(StackRelative) | LDA(Absolute) | LDA(DirectX) | LDA(AbsoluteY) |
            LDA(AbsoluteX) | LDX(Absolute) | LDX(DirectY) | LDX(AbsoluteY) | LDY(Absolute) |
            LDY(DirectX) | LDY(AbsoluteX) | ORA(StackRelative) | ORA(Absolute) | ORA(DirectX) |
            ORA(AbsoluteY) | ORA(AbsoluteX) | PHD(Implied) | PLA(Implied) | PLB(Implied) |
            PLP(Implied) | PLX(Implied) | PLY(Implied) | SBC(StackRelative) | SBC(Absolute) |
            SBC(DirectX) | SBC(AbsoluteY) | SBC(AbsoluteX) | STA(StackRelative) | STA(Absolute) |
            STA(DirectX) | STX(Absolute) | STX(DirectY) | STY(Absolute) | STY(DirectX) |
            STZ(DirectX) | STZ(Absolute)  => 4,
            
            ADC(Long) | ADC(IndirectY) | ADC(Indirect) | ADC(LongX) | AND(Long) | AND(IndirectY) |
            AND(Indirect) | AND(LongX) | ASL(Direct) | CMP(Long) | CMP(IndirectY) |
            CMP(Indirect) | CMP(LongX) | DEC(Direct) | EOR(Long) | EOR(IndirectY) |
            EOR(Indirect) | EOR(LongX) | INC(Direct) | JMP(AbsoluteIndirect) | LDA(Long) |
            LDA(IndirectY) | LDA(Indirect) | LDA(LongX) | LSR(Direct) | ORA(Long) |
            ORA(IndirectY) | ORA(Indirect) | ORA(LongX) | PEA(Implied) | PLD(Implied) |
            ROL(Direct) | ROR(Direct) | SBC(Long) | SBC(IndirectY) | SBC(Indirect) |
            SBC(LongX) | STA(Long) | STA(Indirect) | STA(AbsoluteY) | STA(AbsoluteX) |
            STA(LongX) | STZ(AbsoluteX) | TRB(Direct) | TSB(Direct)  => 5,
            
            ADC(IndirectX) | ADC(IndirectLong) | ADC(IndirectLongY) | AND(IndirectX) | AND(IndirectLong) | AND(IndirectLongY) |
            ASL(Absolute) | ASL(DirectX) | CMP(IndirectX) | CMP(IndirectLong) | CMP(IndirectLongY) |
            DEC(Absolute) | DEC(DirectX) | EOR(IndirectX) | EOR(IndirectLong) | EOR(IndirectLongY) |
            INC(Absolute) | INC(DirectX) | JML(AbsoluteIndirectLong) | JMP(AbsoluteIndirectX) | JSR(Absolute) |
            LDA(IndirectX) | LDA(IndirectLong) | LDA(IndirectLongY) | LSR(Absolute) | LSR(DirectX) |
            ORA(IndirectX) | ORA(IndirectLong) | ORA(IndirectLongY) | PEI(Implied) | PER(Implied) |
            ROL(Absolute) | ROL(DirectX) | ROR(Absolute) | ROR(DirectX) | RTI(Implied) |
            RTL(Implied) | RTS(Implied) | SBC(IndirectX) | SBC(IndirectLong) | SBC(IndirectLongY) |
            STA(IndirectX) | STA(IndirectLong) | STA(IndirectY) | STA(IndirectLongY) | TRB(Absolute) |
            TSB(Absolute)  => 6,
            
            ADC(StackRelativeY) | AND(StackRelativeY) | ASL(AbsoluteX) | BRK(Implied) | CMP(StackRelativeY) | COP(Implied) |
            DEC(AbsoluteX) | EOR(StackRelativeY) | INC(AbsoluteX) | LDA(StackRelativeY) | LSR(AbsoluteX) |
            ORA(StackRelativeY) | ROL(AbsoluteX) | ROR(AbsoluteX) | SBC(StackRelativeY) | STA(StackRelativeY)  => 7,
            
            JSL(Long) | JSR(AbsoluteIndirectX)  => 8,
            
            _ => panic!("Invalid instruction: {:?}", self),
        }
    }
    
    pub fn get_addressing_mode(&self) -> AddressingMode {
        use Instruction::*;
        match self {
            ADC(addr_mode) => *addr_mode,
            AND(addr_mode) => *addr_mode,
            ASL(addr_mode) => *addr_mode,
            ASLA(addr_mode) => *addr_mode,
            BEQ(addr_mode) => *addr_mode,
            BGE(addr_mode) => *addr_mode,
            BIT(addr_mode) => *addr_mode,
            BLT(addr_mode) => *addr_mode,
            BMI(addr_mode) => *addr_mode,
            BNE(addr_mode) => *addr_mode,
            BPL(addr_mode) => *addr_mode,
            BRA(addr_mode) => *addr_mode,
            BRK(addr_mode) => *addr_mode,
            BRL(addr_mode) => *addr_mode,
            BVC(addr_mode) => *addr_mode,
            BVS(addr_mode) => *addr_mode,
            CLC(addr_mode) => *addr_mode,
            CLD(addr_mode) => *addr_mode,
            CLI(addr_mode) => *addr_mode,
            CLV(addr_mode) => *addr_mode,
            CMP(addr_mode) => *addr_mode,
            COP(addr_mode) => *addr_mode,
            CPX(addr_mode) => *addr_mode,
            CPY(addr_mode) => *addr_mode,
            DEA(addr_mode) => *addr_mode,
            DEC(addr_mode) => *addr_mode,
            DEX(addr_mode) => *addr_mode,
            DEY(addr_mode) => *addr_mode,
            EOR(addr_mode) => *addr_mode,
            INA(addr_mode) => *addr_mode,
            INC(addr_mode) => *addr_mode,
            INX(addr_mode) => *addr_mode,
            INY(addr_mode) => *addr_mode,
            JML(addr_mode) => *addr_mode,
            JMP(addr_mode) => *addr_mode,
            JSL(addr_mode) => *addr_mode,
            JSR(addr_mode) => *addr_mode,
            LDA(addr_mode) => *addr_mode,
            LDX(addr_mode) => *addr_mode,
            LDY(addr_mode) => *addr_mode,
            LSR(addr_mode) => *addr_mode,
            LSRA(addr_mode) => *addr_mode,
            MVN(addr_mode) => *addr_mode,
            MVP(addr_mode) => *addr_mode,
            NOP(addr_mode) => *addr_mode,
            ORA(addr_mode) => *addr_mode,
            PEA(addr_mode) => *addr_mode,
            PEI(addr_mode) => *addr_mode,
            PER(addr_mode) => *addr_mode,
            PHA(addr_mode) => *addr_mode,
            PHB(addr_mode) => *addr_mode,
            PHD(addr_mode) => *addr_mode,
            PHK(addr_mode) => *addr_mode,
            PHP(addr_mode) => *addr_mode,
            PHX(addr_mode) => *addr_mode,
            PHY(addr_mode) => *addr_mode,
            PLA(addr_mode) => *addr_mode,
            PLB(addr_mode) => *addr_mode,
            PLD(addr_mode) => *addr_mode,
            PLP(addr_mode) => *addr_mode,
            PLX(addr_mode) => *addr_mode,
            PLY(addr_mode) => *addr_mode,
            REP(addr_mode) => *addr_mode,
            ROL(addr_mode) => *addr_mode,
            ROLA(addr_mode) => *addr_mode,
            ROR(addr_mode) => *addr_mode,
            RORA(addr_mode) => *addr_mode,
            RTI(addr_mode) => *addr_mode,
            RTL(addr_mode) => *addr_mode,
            RTS(addr_mode) => *addr_mode,
            SBC(addr_mode) => *addr_mode,
            SEC(addr_mode) => *addr_mode,
            SED(addr_mode) => *addr_mode,
            SEI(addr_mode) => *addr_mode,
            SEP(addr_mode) => *addr_mode,
            STA(addr_mode) => *addr_mode,
            STP(addr_mode) => *addr_mode,
            STX(addr_mode) => *addr_mode,
            STY(addr_mode) => *addr_mode,
            STZ(addr_mode) => *addr_mode,
            TAX(addr_mode) => *addr_mode,
            TAY(addr_mode) => *addr_mode,
            TCD(addr_mode) => *addr_mode,
            TCS(addr_mode) => *addr_mode,
            TDC(addr_mode) => *addr_mode,
            TRB(addr_mode) => *addr_mode,
            TSB(addr_mode) => *addr_mode,
            TSC(addr_mode) => *addr_mode,
            TSX(addr_mode) => *addr_mode,
            TXA(addr_mode) => *addr_mode,
            TXS(addr_mode) => *addr_mode,
            TXY(addr_mode) => *addr_mode,
            TYA(addr_mode) => *addr_mode,
            TYX(addr_mode) => *addr_mode,
            WAI(addr_mode) => *addr_mode,
            WDM(addr_mode) => *addr_mode,
            XBA(addr_mode) => *addr_mode,
            XCE(addr_mode) => *addr_mode,
            
        }
    }
    
    /// Returns true if this instruction writes back to memory during its execution
    /// 
    /// In that case, the instruction should get passed an address instead of the data
    pub fn writes_back(&self) -> bool {
        use Instruction::*;
        match self {
            
            ASL(_) | DEC(_) | INC(_) | LSR(_) | ROR(_) | ROL(_) | TRB(_) | TSB(_) => true,
            _ => false,
        }
    }
}
