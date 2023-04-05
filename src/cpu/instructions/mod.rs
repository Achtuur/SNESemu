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
    /// Argument is 2 byte constant
    ImmediateLong,
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
    /// ADC #$21 #$33 would have a length of 3
    pub fn get_length(&self) -> usize {
        use Instruction::*;
        match self {
            Asl | Clc | Cld | Cli | Clv | Dea | Dex | Dey | Ina | Inx | Iny | Lsr | Nop | Pha | Phb | Phd | Phk | Php | Phx | Phy | Pla | Plb | Pld | Plp | Plx | Ply | Rol | Ror | Rti | Rtl | Rts | Sec | Sed | Sei | Stp | Tax | Tay | Tcd | Tcs | Tdc | Tsc | Tsx | Txa | Txs | Txy | Tya | Tyx | Wai | Xba | Xce  => 1,
            AdcDPIX | AdcSR | AdcDP | AdcDPLong | AdcImm | AdcDPIY | AdcDPI | AdcSRY | AdcDPX | AdcDPIYLong | AndDPIX | AndSR | AndDP | AndDPLong | AndImm | AndDPIY | AndDPI | AndSRY | AndDPX | AndDPIYLong | AslDP | AslDPX | Blt | Bge | Beq | BitDP | BitDPX | BitImm | Bmi | Bne | Bpl | Bra | Brk | Bvc | Bvs | CmpDPIX | CmpSR | CmpDP | CmpDPLong | CmpImm | CmpDPIY | CmpDPI | CmpSRY | CmpDPX | CmpDPIYLong | Cop | CpxImm | CpxDP | CpyImm | CpyDP | DecDP | DecDPX | EorDPIX | EorSR | EorDP | EorDPLong | EorImm | EorDPIY | EorDPI | EorSRY | EorDPX | EorDPIYLong | IncDP | IncDPX | LdaDPIX | LdaSR | LdaDP | LdaDPLong | LdaImm | LdaDPIY | LdaDPI | LdaSRY | LdaDPX | LdaDPIYLong | LdxImm | LdxDP | LdxDPY | LdyImm | LdyDP | LdyDPX | LsrDP | LsrDPX | OraDPIX | OraSR | OraDP | OraDPLong | OraImm | OraDPIY | OraDPI | OraSRY | OraDPX | OraDPIYLong | Pei | RepImm | RolDP | RolDPX | RorDP | RorDPX | SbcDPIX | SbcSR | SbcDP | SbcDPLong | SbcImm | SbcDPIY | SbcDPI | SbcSRY | SbcDPX | SbcDPIYLong | SepImm | StaDPIX | StaSR | StaDP | StaDPLong | StaDPIY | StaDPI | StaSRY | StaDPX | StaDPIYLong | StxDP | StxDPY | StyDP | StyDPX | StzDP | StzDPX | TrbDP | TsbDP | Wdm  => 2,
            AdcAbs | AdcAbsY | AdcAbsX | AndAbs | AndAbsY | AndAbsX | AslAbs | AslAbsX | BitAbs | BitAbsX | Brl | CmpAbs | CmpAbsY | CmpAbsX | CpxAbs | CpyAbs | DecAbs | DecAbsX | EorAbs | EorAbsY | EorAbsX | IncAbs | IncAbsX | JmpAbs | JmpIndirect | JmpAbsIX | Jml | JsrAbs | JsrAbsIX | LdaAbs | LdaAbsY | LdaAbsX | LdxAbs | LdxAbsY | LdyAbs | LdyAbsX | LsrAbs | LsrAbsX | Mvn | Mvp | OraAbs | OraAbsY | OraAbsX | Pea | Per | RolAbs | RolAbsX | RorAbs | RorAbsX | SbcAbs | SbcAbsY | SbcAbsX | StaAbs | StaAbsY | StaAbsX | StxAbs | StyAbs | StzAbs | StzAbsX | TrbAbs | TsbAbs  => 3,
            AdcAbsLong | AdcAbsXLong | AndAbsLong | AndAbsXLong | CmpAbsLong | CmpAbsXLong | EorAbsLong | EorAbsXLong | JmlAbsLong | JslAbsLong | LdaAbsLong | LdaAbsXLong | OraAbsLong | OraAbsXLong | SbcAbsLong | SbcAbsXLong | StaAbsLong | StaAbsXLong  => 4,
        }
    }
    
    /// Get op code of this instruction, always returns a single byte
    pub fn get_op_code() -> u8 {
        todo!()
    }
    
    /// Get cycle duration of this instruction. This cycle time is then added to the cpu after instruction completes to sync with ie PPU
    pub fn get_cycle_time(&self) -> usize {
        use Instruction::*;
        match self {
            Wdm  => 0,
            Mvn | Mvp  => 1,
            AdcImm | AndImm | Asl | Blt | Bge | Beq | BitImm | Bmi | Bne | Bpl | Bvc | Bvs | Clc | Cld | Cli | Clv | CmpImm | CpxImm | CpyImm | Dea | Dex | Dey | EorImm | Ina | Inx | Iny | LdaImm | LdxImm | LdyImm | Lsr | Nop | OraImm | Rol | Ror | SbcImm | Sec | Sed | Sei | Tax | Tay | Tcd | Tcs | Tdc | Tsc | Tsx | Txa | Txs | Txy | Tya | Tyx | Xce  => 2,
            AdcDP | AndDP | BitDP | Bra | CmpDP | CpxDP | CpyDP | EorDP | JmpAbs | LdaDP | LdxDP | LdyDP | OraDP | Pha | Phb | Phk | Php | Phx | Phy | RepImm | SbcDP | SepImm | StaDP | Stp | StxDP | StyDP | StzDP | Wai | Xba  => 3,
            AdcSR | AdcAbs | AdcDPX | AdcAbsY | AdcAbsX | AndSR | AndAbs | AndDPX | AndAbsY | AndAbsX | BitAbs | BitDPX | BitAbsX | Brl | CmpSR | CmpAbs | CmpDPX | CmpAbsY | CmpAbsX | CpxAbs | CpyAbs | EorSR | EorAbs | EorDPX | EorAbsY | EorAbsX | JmlAbsLong | LdaSR | LdaAbs | LdaDPX | LdaAbsY | LdaAbsX | LdxAbs | LdxDPY | LdxAbsY | LdyAbs | LdyDPX | LdyAbsX | OraSR | OraAbs | OraDPX | OraAbsY | OraAbsX | Phd | Pla | Plb | Plp | Plx | Ply | SbcSR | SbcAbs | SbcDPX | SbcAbsY | SbcAbsX | StaSR | StaAbs | StaDPX | StxAbs | StxDPY | StyAbs | StyDPX | StzDPX | StzAbs  => 4,
            AdcAbsLong | AdcDPIY | AdcDPI | AdcAbsXLong | AndAbsLong | AndDPIY | AndDPI | AndAbsXLong | AslDP | CmpAbsLong | CmpDPIY | CmpDPI | CmpAbsXLong | DecDP | EorAbsLong | EorDPIY | EorDPI | EorAbsXLong | IncDP | JmpIndirect | LdaAbsLong | LdaDPIY | LdaDPI | LdaAbsXLong | LsrDP | OraAbsLong | OraDPIY | OraDPI | OraAbsXLong | Pea | Pld | RolDP | RorDP | SbcAbsLong | SbcDPIY | SbcDPI | SbcAbsXLong | StaAbsLong | StaDPI | StaAbsY | StaAbsX | StaAbsXLong | StzAbsX | TrbDP | TsbDP  => 5,
            AdcDPIX | AdcDPLong | AdcDPIYLong | AndDPIX | AndDPLong | AndDPIYLong | AslAbs | AslDPX | CmpDPIX | CmpDPLong | CmpDPIYLong | DecAbs | DecDPX | EorDPIX | EorDPLong | EorDPIYLong | IncAbs | IncDPX | JmpAbsIX | Jml | JsrAbs | LdaDPIX | LdaDPLong | LdaDPIYLong | LsrAbs | LsrDPX | OraDPIX | OraDPLong | OraDPIYLong | Pei | Per | RolAbs | RolDPX | RorAbs | RorDPX | Rti | Rtl | Rts | SbcDPIX | SbcDPLong | SbcDPIYLong | StaDPIX | StaDPLong | StaDPIY | StaDPIYLong | TrbAbs | TsbAbs  => 6,
            AdcSRY | AndSRY | AslAbsX | Brk | CmpSRY | Cop | DecAbsX | EorSRY | IncAbsX | LdaSRY | LsrAbsX | OraSRY | RolAbsX | RorAbsX | SbcSRY | StaSRY  => 7,
            JslAbsLong | JsrAbsIX  => 8,
        }
    }
    
    pub fn get_addressing_mode(&self) -> AddressingMode {
        use AddressingMode::*;
        use Instruction::*;
        match self {
            AdcDPIX | AndDPIX | CmpDPIX | EorDPIX | LdaDPIX | OraDPIX | SbcDPIX | StaDPIX  
            => IndirectX,
            
            AdcSR | AndSR | CmpSR | EorSR | LdaSR | OraSR | SbcSR | StaSR  
            => StackRelative,
            
            AdcDP | AndDP | AslDP | BitDP | CmpDP | CpxDP | CpyDP | DecDP | EorDP | IncDP | 
            LdaDP | LdxDP | LdyDP | LsrDP | OraDP | RolDP | RorDP | SbcDP | StaDP | StxDP |
            StyDP | StzDP | TrbDP | TsbDP  
            => Direct,
            
            AdcDPLong | AndDPLong | CmpDPLong | EorDPLong | LdaDPLong | 
            OraDPLong | SbcDPLong | StaDPLong  
            => IndirectLong,
            
            AdcImm | AndImm | BitImm | CmpImm | CpxImm | CpyImm | EorImm | LdaImm | LdxImm |
            LdyImm | OraImm | RepImm | SbcImm | SepImm  
            => Immediate,
            
            AdcAbs | AndAbs | AslAbs | BitAbs | CmpAbs | CpxAbs | CpyAbs | DecAbs | EorAbs |
            IncAbs | JmpAbs | JsrAbs | LdaAbs | LdxAbs | LdyAbs | LsrAbs | OraAbs | RolAbs |
            RorAbs | SbcAbs | StaAbs | StxAbs | StyAbs | StzAbs | TrbAbs | TsbAbs  
            => Absolute,
            
            AdcAbsLong | AndAbsLong | CmpAbsLong | EorAbsLong | JmlAbsLong | JslAbsLong |
            LdaAbsLong | OraAbsLong | SbcAbsLong | StaAbsLong  
            => Long,
            
            AdcDPIY | AndDPIY | CmpDPIY | EorDPIY | LdaDPIY | OraDPIY |
            SbcDPIY | StaDPIY  
            => IndirectY,
            
            AdcDPI | AndDPI | CmpDPI | EorDPI | LdaDPI | OraDPI | SbcDPI | StaDPI  
            => Indirect,
            
            AdcSRY | AndSRY | CmpSRY | EorSRY | LdaSRY | OraSRY | SbcSRY | StaSRY  
            => StackRelativeY,
            
            AdcDPX | AndDPX | AslDPX | BitDPX | CmpDPX | DecDPX | EorDPX | IncDPX |
            LdaDPX | LdyDPX | LsrDPX | OraDPX | RolDPX | RorDPX | SbcDPX | StaDPX |
            StyDPX | StzDPX 
            => DirectX,
            
            AdcDPIYLong | AndDPIYLong | CmpDPIYLong | EorDPIYLong | LdaDPIYLong |
            OraDPIYLong | SbcDPIYLong | StaDPIYLong
            => IndirectLongY,
            
            AdcAbsY | AndAbsY | CmpAbsY | EorAbsY | LdaAbsY | LdxAbsY | OraAbsY |
            SbcAbsY | StaAbsY  
            => AbsoluteY,
            
            AdcAbsX | AndAbsX | AslAbsX | BitAbsX | CmpAbsX | DecAbsX | EorAbsX |
            IncAbsX | LdaAbsX | LdyAbsX | LsrAbsX | OraAbsX | RolAbsX | RorAbsX |
            SbcAbsX | StaAbsX | StzAbsX 
            => AbsoluteX,
            AdcAbsXLong | AndAbsXLong | CmpAbsXLong | EorAbsXLong | LdaAbsXLong |
            OraAbsXLong | SbcAbsXLong | StaAbsXLong 
            => LongX,
            
            Asl | Brk | Clc | Cld | Cli | Clv | Cop | Dea | Dex | Dey | Ina | Inx
            | Iny | Lsr | Nop | Pea | Pei | Per | Pha | Phb | Phd | Phk | Php | Phx
            | Phy | Pla | Plb | Pld | Plp | Plx | Ply | Rol | Ror | Rti | Rtl | Rts
            | Sec | Sed | Sei | Stp | Tax | Tay | Tcd | Tcs | Tdc | Tsc | Tsx | Txa
            | Txs | Txy | Tya | Tyx | Wai | Wdm | Xba | Xce  
            => Implied,
            
            Blt | Bge | Beq | Bmi | Bne | Bpl | Bra | Bvc | Bvs => Relative,
            
            Brl  => RelativeLong,
            
            JmpIndirect  => AbsoluteIndirect,
            
            JmpAbsIX | JsrAbsIX  => AbsoluteIndirectX,
            
            Jml  => AbsoluteIndirectLong,
            
            LdxDPY | StxDPY  => DirectY,
            
            Mvn | Mvp  => Move,
        }
    }
    
    /// Returns true if this instruction writes back to memory during its execution
    /// 
    /// In that case, the instruction should get passed an address instead of the data
    pub fn writes_back(&self) -> bool {
        use Instruction::*;
        match self {
            IncAbs | IncAbsX | IncDP | IncDPX | 
            DecAbs | DecAbsX | DecDP | DecDPX | 
            AslAbs | AslAbsX | AslDP | AslDPX | 
            LsrAbs | LsrAbsX | LsrDP | LsrDPX |
            RorAbs | RorAbsX | RorDP | RorDPX |
            RolAbs | RolAbsX | RolDP | RolDPX |
            TrbAbs | TrbDP | TsbAbs | TsbDP => true,
            _ => false,
        }
    }
}
