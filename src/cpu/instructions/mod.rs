use self::instructions::Instruction;

pub mod arithmetic;
pub mod block;
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
    /// Argument is implied (ie STX)
    Implied,
    /// Argument is implied to be accumulator
    ImpliedAcc,
    /// Argument is 1 byte constant
    Immediate,
    /// Argument is 2 byte constant
    ImmediateLong,
    /// Argument is 1 byte address relative to pc
    Relative,
    /// Argument is 2 byte address relative to pc
    RelativeLong,
    /// Argument is absolute 2 bytes
    Absolute,
    /// Argument is absolute 2 bytes, indexed with X
    AbsoluteX,
    /// Argument is absolute 2 bytes, indexed with Y
    AbsoluteY,
    /// Argument is absolute 3 bytes
    AbsoluteLong,
    /// Argument is absolute 3 bytes, indexed with X
    AbsoluteLongX,
    /// Argument is memory address specified with 2 bytes (same bank)
    AbsoluteIndirect,
    AbsoluteIndirectX,
    /// Argument is memory address specified with 3 bytes
    AbsoluteIndirectLong,
    /// Argument is memory address specified with 3 bytes, indexed by X
    AbsoluteIndirectLongX,
    /// Argument is memory address is zero-based memory address (ie `LDA $XX` would read `$0000XX`)
    DirectPage,
    /// Argument is memory address is zero-based memory address (ie `LDA $XX` would read `$0000XX`) indexed by X
    DirectPageX,
    /// Argument is memory address is zero-based memory address (ie `LDA $XX` would read `$0000XX`) indexed by Y
    DirectPageY,
    /// Argument is memory address that is inside memory address specified by argument (ie `LDA $XX` would read memory address in `$0000XX`)
    DirectIndirect,
    /// Argument is memory address that is inside memory address specified by argument (ie `LDA $XX` would read memory address in `$0000XX`) indexed by X
    DirectIndirectX,
    /// Argument is memory address that is inside memory address specified by argument (ie `LDA $XX` would read memory address in `$0000XX`) indexed by Y
    DirectIndirectY,
    DirectPageIndirectLong,
    DirectPageIndirectLongX,
    DirectPageIndirectLongY,
    /// Stack is used as lower 2 bytes of address, argument is address relative to stack. Eg, stack pointer is `$1FF0` then `LDA $05, s` loads `$001FF5`
    StackRelative,
    /// Stack is used as lower 2 bytes of address, argument is address relative to stack. Eg, stack pointer is `$1FF0` then `LDA $05, s` loads `$001FF5` plus Y
    StackRelativeY,
    /// Used only for `MVN` and `MVP` instructions, explained in those implementations in more detail
    BlockMove,

    None,
}

impl Instruction {
    /// Length of this instruction in bytes, includes op code and all arguments
    /// 
    /// ADC #$21 #$33 would have a length of 3
    fn get_length(&self) -> usize {
        use Instruction::*;
        match self {
            Asl | Clc | Cld | Cli | Clv | Dea | Dex | Dey | Ina | Inx | Iny | Lsr | Nop | Pha | Phb | Phd | Phk | Php | Phx | Phy | Pla | Plb | Pld | Plp | Plx | Ply | Rol | Ror | Rti | Rtl | Rts | Sec | Sed | Sei | Stp | Tax | Tay | Tcd | Tcs | Tdc | Tsc | Tsx | Txa | Txs | Txy | Tya | Tyx | Wai | Xba | Xce  => 1,
            AdcDPIX | AdcSR | AdcDP | AdcDPLong | AdcImm | AdcDPIY | AdcDPI | AdcSRY | AdcDPX | AdcDPIYLong | AndDPIX | AndSR | AndDP | AndDPLong | AndImm | AndDPIY | AndDPI | AndSRY | AndDPX | AndDPIYLong | AslDP | AslDPX | Blt | Bge | Beq | BitDP | BitDPX | BitImm | Bmi | Bne | Bpl | Bra | Brk | Bvc | Bvs | CmpDPIX | CmpSR | CmpDP | CmpDPLong | CmpImm | CmpDPIY | CmpDPI | CmpSRY | CmpDPX | CmpDPIYLong | Cop | CpxImm | CpxDP | CpyImm | CpyDP | DecDP | DecDPX | EorDPIX | EorSR | EorDP | EorDPLong | EorImm | EorDPIY | EorDPI | EorSRY | EorDPX | EorDPIYLong | IncDP | IncDPX | LdaDPIX | LdaSR | LdaDP | LdaDPLong | LdaImm | LdaDPIY | LdaDPI | LdaSRY | LdaDPX | LdaDPIYLong | LdxImm | LdxDP | LdxDPY | LdyImm | LdyDP | LdyDPX | LsrDP | LsrDPX | OraDPIX | OraSR | OraDP | OraDPLong | OraImm | OraDPIY | OraDPI | OraSRY | OraDPX | OraDPIYLong | Pei | RepImm | RolDP | RolDPX | RorDP | RorDPX | SbcDPIX | SbcSR | SbcDP | SbcDPLong | SbcImm | SbcDPIY | SbcDPI | SbcSRY | SbcDPX | SbcDPIYLong | SepImm | StaDPIX | StaSR | StaDP | StaDPLong | StaDPIY | StaDPI | StaSRY | StaDPX | StaDPIYLong | StxDP | StxDPY | StyDP | StyDPX | StzDP | StzDPX | TrbDP | TsbDP | Wdm  => 2,
            AdcAbs | AdcAbsY | AdcAbsX | AndAbs | AndAbsY | AndAbsX | AslAbs | AslAbsX | BitAbs | BitAbsX | Brl | CmpAbs | CmpAbsY | CmpAbsX | CpxAbs | CpyAbs | DecAbs | DecAbsX | EorAbs | EorAbsY | EorAbsX | IncAbs | IncAbsX | JmpAbs | Jmp | JmpAbsIX | Jml | JsrAbs | JsrAbsIX | LdaAbs | LdaAbsY | LdaAbsX | LdxAbs | LdxAbsY | LdyAbs | LdyAbsX | LsrAbs | LsrAbsX | Mvn | Mvp | OraAbs | OraAbsY | OraAbsX | Pea | Per | RolAbs | RolAbsX | RorAbs | RorAbsX | SbcAbs | SbcAbsY | SbcAbsX | StaAbs | StaAbsY | StaAbsX | StxAbs | StyAbs | StzAbs | StzAbsX | TrbAbs | TsbAbs  => 3,
            AdcAbsLong | AdcAbsXLong | AndAbsLong | AndAbsXLong | CmpAbsLong | CmpAbsXLong | EorAbsLong | EorAbsXLong | JmlAbsLong | JslAbsLong | LdaAbsLong | LdaAbsXLong | OraAbsLong | OraAbsXLong | SbcAbsLong | SbcAbsXLong | StaAbsLong | StaAbsXLong  => 4,
        }
    }
    
    /// Get op code of this instruction, always returns a single byte
    fn get_op_code() -> u8 {
        todo!()
    }
    
    /// Get cycle duration of this instruction. This cycle time is then added to the cpu after instruction completes to sync with ie PPU
    fn get_cycle_time(&self) -> usize {
        use Instruction::*;
        match self {
            Asl | Clc | Cld | Cli | Clv | Dea | Dex | Dey | Ina | Inx | Iny | Lsr | Nop | Pha | Phb | Phd | Phk | Php | Phx | Phy | Pla | Plb | Pld | Plp | Plx | Ply | Rol | Ror | Rti | Rtl | Rts | Sec | Sed | Sei | Stp | Tax | Tay | Tcd | Tcs | Tdc | Tsc | Tsx | Txa | Txs | Txy | Tya | Tyx | Wai | Xba | Xce  => 1,
            AdcDPIX | AdcSR | AdcDP | AdcDPLong | AdcImm | AdcDPIY | AdcDPI | AdcSRY | AdcDPX | AdcDPIYLong | AndDPIX | AndSR | AndDP | AndDPLong | AndImm | AndDPIY | AndDPI | AndSRY | AndDPX | AndDPIYLong | AslDP | AslDPX | Blt | Bge | Beq | BitDP | BitDPX | BitImm | Bmi | Bne | Bpl | Bra | Brk | Bvc | Bvs | CmpDPIX | CmpSR | CmpDP | CmpDPLong | CmpImm | CmpDPIY | CmpDPI | CmpSRY | CmpDPX | CmpDPIYLong | Cop | CpxImm | CpxDP | CpyImm | CpyDP | DecDP | DecDPX | EorDPIX | EorSR | EorDP | EorDPLong | EorImm | EorDPIY | EorDPI | EorSRY | EorDPX | EorDPIYLong | IncDP | IncDPX | LdaDPIX | LdaSR | LdaDP | LdaDPLong | LdaImm | LdaDPIY | LdaDPI | LdaSRY | LdaDPX | LdaDPIYLong | LdxImm | LdxDP | LdxDPY | LdyImm | LdyDP | LdyDPX | LsrDP | LsrDPX | OraDPIX | OraSR | OraDP | OraDPLong | OraImm | OraDPIY | OraDPI | OraSRY | OraDPX | OraDPIYLong | Pei | RepImm | RolDP | RolDPX | RorDP | RorDPX | SbcDPIX | SbcSR | SbcDP | SbcDPLong | SbcImm | SbcDPIY | SbcDPI | SbcSRY | SbcDPX | SbcDPIYLong | SepImm | StaDPIX | StaSR | StaDP | StaDPLong | StaDPIY | StaDPI | StaSRY | StaDPX | StaDPIYLong | StxDP | StxDPY | StyDP | StyDPX | StzDP | StzDPX | TrbDP | TsbDP | Wdm  => 2,
            AdcAbs | AdcAbsY | AdcAbsX | AndAbs | AndAbsY | AndAbsX | AslAbs | AslAbsX | BitAbs | BitAbsX | Brl | CmpAbs | CmpAbsY | CmpAbsX | CpxAbs | CpyAbs | DecAbs | DecAbsX | EorAbs | EorAbsY | EorAbsX | IncAbs | IncAbsX | JmpAbs | Jmp | JmpAbsIX | Jml | JsrAbs | JsrAbsIX | LdaAbs | LdaAbsY | LdaAbsX | LdxAbs | LdxAbsY | LdyAbs | LdyAbsX | LsrAbs | LsrAbsX | Mvn | Mvp | OraAbs | OraAbsY | OraAbsX | Pea | Per | RolAbs | RolAbsX | RorAbs | RorAbsX | SbcAbs | SbcAbsY | SbcAbsX | StaAbs | StaAbsY | StaAbsX | StxAbs | StyAbs | StzAbs | StzAbsX | TrbAbs | TsbAbs  => 3,
            AdcAbsLong | AdcAbsXLong | AndAbsLong | AndAbsXLong | CmpAbsLong | CmpAbsXLong | EorAbsLong | EorAbsXLong | JmlAbsLong | JslAbsLong | LdaAbsLong | LdaAbsXLong | OraAbsLong | OraAbsXLong | SbcAbsLong | SbcAbsXLong | StaAbsLong | StaAbsXLong  => 4,
        }
    }
    
    fn get_addressing_mode(&self) -> AddressingMode {
        use AddressingMode::*;
        use Instruction::*;
        match self {
            AdcDPIX | AndDPIX | CmpDPIX | EorDPIX | LdaDPIX | OraDPIX | SbcDPIX | StaDPIX  => DirectIndirectX,
            AdcSR | AndSR | CmpSR | EorSR | LdaSR | OraSR | SbcSR | StaSR  => StackRelative,
            AdcDP | AndDP | AslDP | BitDP | CmpDP | CpxDP | CpyDP | DecDP | EorDP | IncDP | LdaDP | LdxDP | LdyDP | LsrDP | OraDP | RolDP | RorDP | SbcDP | StaDP | StxDP | StyDP | StzDP | TrbDP | TsbDP  => DirectPage,
            AdcDPLong | AndDPLong | CmpDPLong | EorDPLong | LdaDPLong | OraDPLong | SbcDPLong | StaDPLong  => DirectPageIndirectLong,
            AdcImm | AndImm | BitImm | CmpImm | CpxImm | CpyImm | EorImm | LdaImm | LdxImm | LdyImm | OraImm | RepImm | SbcImm | SepImm  => Immediate,
            AdcAbs | AndAbs | AslAbs | BitAbs | CmpAbs | CpxAbs | CpyAbs | DecAbs | EorAbs | IncAbs | JmpAbs | JsrAbs | LdaAbs | LdxAbs | LdyAbs | LsrAbs | OraAbs | RolAbs | RorAbs | SbcAbs | StaAbs | StxAbs | StyAbs | StzAbs | TrbAbs | TsbAbs  => Absolute,
            AdcAbsLong | AndAbsLong | CmpAbsLong | EorAbsLong | JmlAbsLong | JslAbsLong | LdaAbsLong | OraAbsLong | SbcAbsLong | StaAbsLong  => AbsoluteLong,
            AdcDPIY | AndDPIY | CmpDPIY | EorDPIY | LdaDPIY | OraDPIY | SbcDPIY | StaDPIY  => DirectIndirectY,
            AdcDPI | AndDPI | CmpDPI | EorDPI | LdaDPI | OraDPI | SbcDPI | StaDPI  => DirectIndirect,
            AdcSRY | AndSRY | CmpSRY | EorSRY | LdaSRY | OraSRY | SbcSRY | StaSRY  => StackRelativeY,
            AdcDPX | AndDPX | AslDPX | BitDPX | CmpDPX | DecDPX | EorDPX | IncDPX | LdaDPX | LdyDPX | LsrDPX | OraDPX | RolDPX | RorDPX | SbcDPX | StaDPX | StyDPX | StzDPX  => DirectPageX,
            AdcDPIYLong | AndDPIYLong | CmpDPIYLong | EorDPIYLong | LdaDPIYLong | OraDPIYLong | SbcDPIYLong | StaDPIYLong  => DirectPageIndirectLongY,
            AdcAbsY | AndAbsY | CmpAbsY | EorAbsY | LdaAbsY | LdxAbsY | OraAbsY | SbcAbsY | StaAbsY  => AbsoluteY,
            AdcAbsX | AndAbsX | AslAbsX | BitAbsX | CmpAbsX | DecAbsX | EorAbsX | IncAbsX | LdaAbsX | LdyAbsX | LsrAbsX | OraAbsX | RolAbsX | RorAbsX | SbcAbsX | StaAbsX | StzAbsX  => AbsoluteX,
            AdcAbsXLong | AndAbsXLong | CmpAbsXLong | EorAbsXLong | LdaAbsXLong | OraAbsXLong | SbcAbsXLong | StaAbsXLong  => AbsoluteLongX,
            Asl | Brk | Clc | Cld | Cli | Clv | Cop | Dea | Dex | Dey | Ina | Inx | Iny | Lsr | Nop | Pea | Pei | Per | Pha | Phb | Phd | Phk | Php | Phx | Phy | Pla | Plb | Pld | Plp | Plx | Ply | Rol | Ror | Rti | Rtl | Rts | Sec | Sed | Sei | Stp | Tax | Tay | Tcd | Tcs | Tdc | Tsc | Tsx | Txa | Txs | Txy | Tya | Tyx | Wai | Wdm | Xba | Xce  => Implied,
            Blt | Bge | Beq | Bmi | Bne | Bpl | Bra | Bvc | Bvs  => Relative,
            Brl  => RelativeLong,
            Jmp  => AbsoluteIndirect,
            JmpAbsIX | JsrAbsIX  => AbsoluteIndirectX,
            Jml  => AbsoluteIndirectLong,
            LdxDPY | StxDPY  => DirectPageY,
            Mvn | Mvp  => BlockMove,
        }
    }
}
    