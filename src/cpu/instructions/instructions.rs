pub enum Instruction {
    
/// ($00) BRK, Break (Stack/Interrupt)
/// 
/// * Byte length: 2
/// * Cycles: 7
Brk,

/// ($01) ORA, OR Accumulator with Memory (DP Indexed Indirect,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
OraDPIX,

/// ($02) COP, Co-Processor (Stack/Interrupt)
/// 
/// * Byte length: 2
/// * Cycles: 7
Cop,

/// ($03) ORA, OR Accumulator with Memory (Stack Relative)
/// 
/// * Byte length: 2
/// * Cycles: 4
OraSR,

/// ($04) TSB, Test and Set Memory Bits Against Accumulator (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 5
TsbDP,

/// ($05) ORA, OR Accumulator with Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
OraDP,

/// ($06) ASL, Arithmetic Shift Left (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 5
AslDP,

/// ($07) ORA, OR Accumulator with Memory (DP Indirect Long)
/// 
/// * Byte length: 2
/// * Cycles: 6
OraDPLong,

/// ($08) PHP, Push Processor Status Register (Stack (Push))
/// 
/// * Byte length: 1
/// * Cycles: 3
Php,

/// ($09) ORA, OR Accumulator with Memory (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
OraImm,

/// ($0A) ASL, Arithmetic Shift Left (Accumulator)
/// 
/// * Byte length: 1
/// * Cycles: 2
Asl,

/// ($0B) PHD, Push Direct Page Register (Stack (Push))
/// 
/// * Byte length: 1
/// * Cycles: 4
Phd,

/// ($0C) TSB, Test and Set Memory Bits Against Accumulator (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 6
TsbAbs,

/// ($0D) ORA, OR Accumulator with Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
OraAbs,

/// ($0E) ASL, Arithmetic Shift Left (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 6
AslAbs,

/// ($0F) ORA, OR Accumulator with Memory (Absolute Long)
/// 
/// * Byte length: 4
/// * Cycles: 5
OraAbsLong,

/// ($10) BPL, Branch if Plus (Program Counter Relative)
/// 
/// * Byte length: 2
/// * Cycles: 2
Bpl,

/// ($11) ORA, OR Accumulator with Memory (DP Indirect Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 5
OraDPIY,

/// ($12) ORA, OR Accumulator with Memory (DP Indirect)
/// 
/// * Byte length: 2
/// * Cycles: 5
OraDPI,

/// ($13) ORA, OR Accumulator with Memory (SR Indirect Indexed,Y)
/// 
/// * Byte length: 2
/// * Cycles: 7
OraSRY,

/// ($14) TRB, Test and Reset Memory Bits Against Accumulator (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 5
TrbDP,

/// ($15) ORA, OR Accumulator with Memory (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
OraDPX,

/// ($16) ASL, Arithmetic Shift Left (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
AslDPX,

/// ($17) ORA, OR Accumulator with Memory (DP Indirect Long Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 6
OraDPIYLong,

/// ($18) CLC, Clear Carry (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Clc,

/// ($19) ORA, OR Accumulator with Memory (Absolute Indexed,Y)
/// 
/// * Byte length: 3
/// * Cycles: 4
OraAbsY,

/// ($1A) INA, Increment (Accumulator)
/// 
/// * Byte length: 1
/// * Cycles: 2
Ina,

/// ($1B) TCS, Transfer 16-bit Accumulator to Stack Pointer (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Tcs,

/// ($1C) TRB, Test and Reset Memory Bits Against Accumulator (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 6
TrbAbs,

/// ($1D) ORA, OR Accumulator with Memory (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 4
OraAbsX,

/// ($1E) ASL, Arithmetic Shift Left (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 7
AslAbsX,

/// ($1F) ORA, OR Accumulator with Memory (Absolute Long Indexed,X)
/// 
/// * Byte length: 4
/// * Cycles: 5
OraAbsXLong,

/// ($20) JSR, Jump to Subroutine (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 6
JsrAbs,

/// ($21) AND, AND Accumulator with Memory (DP Indexed Indirect,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
AndDPIX,

/// ($22) JSL, Jump to Subroutine (Absolute Long)
/// 
/// * Byte length: 4
/// * Cycles: 8
JslAbsLong,

/// ($23) AND, AND Accumulator with Memory (Stack Relative)
/// 
/// * Byte length: 2
/// * Cycles: 4
AndSR,

/// ($24) BIT, Test Bits (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
BitDP,

/// ($25) AND, AND Accumulator with Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
AndDP,

/// ($26) ROL, Rotate Memory or Accumulator Left (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 5
RolDP,

/// ($27) AND, AND Accumulator with Memory (DP Indirect Long)
/// 
/// * Byte length: 2
/// * Cycles: 6
AndDPLong,

/// ($28) PLP, Pull Processor Status Register (Stack (Pull))
/// 
/// * Byte length: 1
/// * Cycles: 4
Plp,

/// ($29) AND, AND Accumulator with Memory (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
AndImm,

/// ($2A) ROL, Rotate Memory or Accumulator Left (Accumulator)
/// 
/// * Byte length: 1
/// * Cycles: 2
Rol,

/// ($2B) PLD, Pull Direct Page Register (Stack (Pull))
/// 
/// * Byte length: 1
/// * Cycles: 5
Pld,

/// ($2C) BIT, Test Bits (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
BitAbs,

/// ($2D) AND, AND Accumulator with Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
AndAbs,

/// ($2E) ROL, Rotate Memory or Accumulator Left (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 6
RolAbs,

/// ($2F) AND, AND Accumulator with Memory (Absolute Long)
/// 
/// * Byte length: 4
/// * Cycles: 5
AndAbsLong,

/// ($30) BMI, Branch if Minus (Program Counter Relative)
/// 
/// * Byte length: 2
/// * Cycles: 2
Bmi,

/// ($31) AND, AND Accumulator with Memory (DP Indirect Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 5
AndDPIY,

/// ($32) AND, AND Accumulator with Memory (DP Indirect)
/// 
/// * Byte length: 2
/// * Cycles: 5
AndDPI,

/// ($33) AND, AND Accumulator with Memory (SR Indirect Indexed,Y)
/// 
/// * Byte length: 2
/// * Cycles: 7
AndSRY,

/// ($34) BIT, Test Bits (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
BitDPX,

/// ($35) AND, AND Accumulator with Memory (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
AndDPX,

/// ($36) ROL, Rotate Memory or Accumulator Left (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
RolDPX,

/// ($37) AND, AND Accumulator with Memory (DP Indirect Long Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 6
AndDPIYLong,

/// ($38) SEC, Set Carry Flag (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Sec,

/// ($39) AND, AND Accumulator with Memory (Absolute Indexed,Y)
/// 
/// * Byte length: 3
/// * Cycles: 4
AndAbsY,

/// ($3A) DEA, Decrement (Accumulator)
/// 
/// * Byte length: 1
/// * Cycles: 2
Dea,

/// ($3B) TSC, Transfer Stack Pointer to 16-bit Accumulator (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Tsc,

/// ($3C) BIT, Test Bits (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 4
BitAbsX,

/// ($3D) AND, AND Accumulator with Memory (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 4
AndAbsX,

/// ($3E) ROL, Rotate Memory or Accumulator Left (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 7
RolAbsX,

/// ($3F) AND, AND Accumulator with Memory (Absolute Long Indexed,X)
/// 
/// * Byte length: 4
/// * Cycles: 5
AndAbsXLong,

/// ($40) RTI, Return from Interrupt (Stack (RTI))
/// 
/// * Byte length: 1
/// * Cycles: 6
Rti,

/// ($41) EOR, Exclusive-OR Accumulator with Memory (DP Indexed Indirect,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
EorDPIX,

/// ($42) WDM, <em>Reserved for Future Expansion</em> ()
/// 
/// * Byte length: 2
/// * Cycles: 0
Wdm,

/// ($43) EOR, Exclusive-OR Accumulator with Memory (Stack Relative)
/// 
/// * Byte length: 2
/// * Cycles: 4
EorSR,

/// ($44) MVP, Block Move Positive (Block Move)
/// 
/// * Byte length: 3
/// * Cycles: 1
Mvp,

/// ($45) EOR, Exclusive-OR Accumulator with Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
EorDP,

/// ($46) LSR, Logical Shift Memory or Accumulator Right (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 5
LsrDP,

/// ($47) EOR, Exclusive-OR Accumulator with Memory (DP Indirect Long)
/// 
/// * Byte length: 2
/// * Cycles: 6
EorDPLong,

/// ($48) PHA, Push Accumulator (Stack (Push))
/// 
/// * Byte length: 1
/// * Cycles: 3
Pha,

/// ($49) EOR, Exclusive-OR Accumulator with Memory (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
EorImm,

/// ($4A) LSR, Logical Shift Memory or Accumulator Right (Accumulator)
/// 
/// * Byte length: 1
/// * Cycles: 2
Lsr,

/// ($4B) PHK, Push Program Bank Register (Stack (Push))
/// 
/// * Byte length: 1
/// * Cycles: 3
Phk,

/// ($4C) JMP, Jump (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 3
JmpAbs,

/// ($4D) EOR, Exclusive-OR Accumulator with Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
EorAbs,

/// ($4E) LSR, Logical Shift Memory or Accumulator Right (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 6
LsrAbs,

/// ($4F) EOR, Exclusive-OR Accumulator with Memory (Absolute Long)
/// 
/// * Byte length: 4
/// * Cycles: 5
EorAbsLong,

/// ($50) BVC, Branch if Overflow Clear (Program Counter Relative)
/// 
/// * Byte length: 2
/// * Cycles: 2
Bvc,

/// ($51) EOR, Exclusive-OR Accumulator with Memory (DP Indirect Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 5
EorDPIY,

/// ($52) EOR, Exclusive-OR Accumulator with Memory (DP Indirect)
/// 
/// * Byte length: 2
/// * Cycles: 5
EorDPI,

/// ($53) EOR, Exclusive-OR Accumulator with Memory (SR Indirect Indexed,Y)
/// 
/// * Byte length: 2
/// * Cycles: 7
EorSRY,

/// ($54) MVN, Block Move Negative (Block Move)
/// 
/// * Byte length: 3
/// * Cycles: 1
Mvn,

/// ($55) EOR, Exclusive-OR Accumulator with Memory (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
EorDPX,

/// ($56) LSR, Logical Shift Memory or Accumulator Right (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
LsrDPX,

/// ($57) EOR, Exclusive-OR Accumulator with Memory (DP Indirect Long Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 6
EorDPIYLong,

/// ($58) CLI, Clear Interrupt Disable Flag (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Cli,

/// ($59) EOR, Exclusive-OR Accumulator with Memory (Absolute Indexed,Y)
/// 
/// * Byte length: 3
/// * Cycles: 4
EorAbsY,

/// ($5A) PHY, Push Index Register Y (Stack (Push))
/// 
/// * Byte length: 1
/// * Cycles: 3
Phy,

/// ($5B) TCD, Transfer 16-bit Accumulator to Direct Page Register (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Tcd,

/// ($5C) JML, Jump (Absolute Long)
/// 
/// * Byte length: 4
/// * Cycles: 4
JmlAbsLong,

/// ($5D) EOR, Exclusive-OR Accumulator with Memory (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 4
EorAbsX,

/// ($5E) LSR, Logical Shift Memory or Accumulator Right (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 7
LsrAbsX,

/// ($5F) EOR, Exclusive-OR Accumulator with Memory (Absolute Long Indexed,X)
/// 
/// * Byte length: 4
/// * Cycles: 5
EorAbsXLong,

/// ($60) RTS, Return from Subroutine (Stack (RTS))
/// 
/// * Byte length: 1
/// * Cycles: 6
Rts,

/// ($61) ADC, Add With Carry (DP Indexed Indirect,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
AdcDPIX,

/// ($62) PER, Push Effective PC Relative Indirect Address (Stack (PC Relative Long))
/// 
/// * Byte length: 3
/// * Cycles: 6
Per,

/// ($63) ADC, Add With Carry (Stack Relative)
/// 
/// * Byte length: 2
/// * Cycles: 4
AdcSR,

/// ($64) STZ, Store Zero to Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
StzDP,

/// ($65) ADC, Add With Carry (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
AdcDP,

/// ($66) ROR, Rotate Memory or Accumulator Right (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 5
RorDP,

/// ($67) ADC, Add With Carry (DP Indirect Long)
/// 
/// * Byte length: 2
/// * Cycles: 6
AdcDPLong,

/// ($68) PLA, Pull Accumulator (Stack (Pull))
/// 
/// * Byte length: 1
/// * Cycles: 4
Pla,

/// ($69) ADC, Add With Carry (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
AdcImm,

/// ($6A) ROR, Rotate Memory or Accumulator Right (Accumulator)
/// 
/// * Byte length: 1
/// * Cycles: 2
Ror,

/// ($6B) RTL, Return from Subroutine Long (Stack (RTL))
/// 
/// * Byte length: 1
/// * Cycles: 6
Rtl,

/// ($6C) JMP, Jump (Absolute Indirect)
/// 
/// * Byte length: 3
/// * Cycles: 5
JmpIndirect,

/// ($6D) ADC, Add With Carry (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
AdcAbs,

/// ($6E) ROR, Rotate Memory or Accumulator Right (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 6
RorAbs,

/// ($6F) ADC, Add With Carry (Absolute Long)
/// 
/// * Byte length: 4
/// * Cycles: 5
AdcAbsLong,

/// ($70) BVS, Branch if Overflow Set (Program Counter Relative)
/// 
/// * Byte length: 2
/// * Cycles: 2
Bvs,

/// ($71) ADC, Add With Carry (DP Indirect Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 5
AdcDPIY,

/// ($72) ADC, Add With Carry (DP Indirect)
/// 
/// * Byte length: 2
/// * Cycles: 5
AdcDPI,

/// ($73) ADC, Add With Carry (SR Indirect Indexed,Y)
/// 
/// * Byte length: 2
/// * Cycles: 7
AdcSRY,

/// ($74) STZ, Store Zero to Memory (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
StzDPX,

/// ($75) ADC, Add With Carry (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
AdcDPX,

/// ($76) ROR, Rotate Memory or Accumulator Right (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
RorDPX,

/// ($77) ADC, Add With Carry (DP Indirect Long Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 6
AdcDPIYLong,

/// ($78) SEI, Set Interrupt Disable Flag (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Sei,

/// ($79) ADC, Add With Carry (Absolute Indexed,Y)
/// 
/// * Byte length: 3
/// * Cycles: 4
AdcAbsY,

/// ($7A) PLY, Pull Index Register Y (Stack (Pull))
/// 
/// * Byte length: 1
/// * Cycles: 4
Ply,

/// ($7B) TDC, Transfer Direct Page Register to 16-bit Accumulator (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Tdc,

/// ($7C) JMP, Jump (Absolute Indexed Indirect)
/// 
/// * Byte length: 3
/// * Cycles: 6
JmpAbsIX,

/// ($7D) ADC, Add With Carry (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 4
AdcAbsX,

/// ($7E) ROR, Rotate Memory or Accumulator Right (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 7
RorAbsX,

/// ($7F) ADC, Add With Carry (Absolute Long Indexed,X)
/// 
/// * Byte length: 4
/// * Cycles: 5
AdcAbsXLong,

/// ($80) BRA, Branch Always (Program Counter Relative)
/// 
/// * Byte length: 2
/// * Cycles: 3
Bra,

/// ($81) STA, Store Accumulator to Memory (DP Indexed Indirect,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
StaDPIX,

/// ($82) BRL, Branch Long Always (Program Counter Relative Long)
/// 
/// * Byte length: 3
/// * Cycles: 4
Brl,

/// ($83) STA, Store Accumulator to Memory (Stack Relative)
/// 
/// * Byte length: 2
/// * Cycles: 4
StaSR,

/// ($84) STY, Store Index Register Y to Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
StyDP,

/// ($85) STA, Store Accumulator to Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
StaDP,

/// ($86) STX, Store Index Register X to Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
StxDP,

/// ($87) STA, Store Accumulator to Memory (DP Indirect Long)
/// 
/// * Byte length: 2
/// * Cycles: 6
StaDPLong,

/// ($88) DEY, Decrement Index Register Y (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Dey,

/// ($89) BIT, Test Bits (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
BitImm,

/// ($8A) TXA, Transfer Index Register X to Accumulator (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Txa,

/// ($8B) PHB, Push Data Bank Register (Stack (Push))
/// 
/// * Byte length: 1
/// * Cycles: 3
Phb,

/// ($8C) STY, Store Index Register Y to Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
StyAbs,

/// ($8D) STA, Store Accumulator to Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
StaAbs,

/// ($8E) STX, Store Index Register X to Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
StxAbs,

/// ($8F) STA, Store Accumulator to Memory (Absolute Long)
/// 
/// * Byte length: 4
/// * Cycles: 5
StaAbsLong,

/// ($90) BLT, Branch if Carry Clear (Program Counter Relative)
/// 
/// * Byte length: 2
/// * Cycles: 2
Blt,

/// ($91) STA, Store Accumulator to Memory (DP Indirect Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 6
StaDPIY,

/// ($92) STA, Store Accumulator to Memory (DP Indirect)
/// 
/// * Byte length: 2
/// * Cycles: 5
StaDPI,

/// ($93) STA, Store Accumulator to Memory (SR Indirect Indexed,Y)
/// 
/// * Byte length: 2
/// * Cycles: 7
StaSRY,

/// ($94) STY, Store Index Register Y to Memory (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
StyDPX,

/// ($95) STA, Store Accumulator to Memory (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
StaDPX,

/// ($96) STX, Store Index Register X to Memory (DP Indexed,Y)
/// 
/// * Byte length: 2
/// * Cycles: 4
StxDPY,

/// ($97) STA, Store Accumulator to Memory (DP Indirect Long Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 6
StaDPIYLong,

/// ($98) TYA, Transfer Index Register Y to Accumulator (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Tya,

/// ($99) STA, Store Accumulator to Memory (Absolute Indexed,Y)
/// 
/// * Byte length: 3
/// * Cycles: 5
StaAbsY,

/// ($9A) TXS, Transfer Index Register X to Stack Pointer (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Txs,

/// ($9B) TXY, Transfer Index Register X to Index Register Y (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Txy,

/// ($9C) STZ, Store Zero to Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
StzAbs,

/// ($9D) STA, Store Accumulator to Memory (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 5
StaAbsX,

/// ($9E) STZ, Store Zero to Memory (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 5
StzAbsX,

/// ($9F) STA, Store Accumulator to Memory (Absolute Long Indexed,X)
/// 
/// * Byte length: 4
/// * Cycles: 5
StaAbsXLong,

/// ($A0) LDY, Load Index Register Y from Memory (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
LdyImm,

/// ($A1) LDA, Load Accumulator from Memory (DP Indexed Indirect,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
LdaDPIX,

/// ($A2) LDX, Load Index Register X from Memory (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
LdxImm,

/// ($A3) LDA, Load Accumulator from Memory (Stack Relative)
/// 
/// * Byte length: 2
/// * Cycles: 4
LdaSR,

/// ($A4) LDY, Load Index Register Y from Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
LdyDP,

/// ($A5) LDA, Load Accumulator from Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
LdaDP,

/// ($A6) LDX, Load Index Register X from Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
LdxDP,

/// ($A7) LDA, Load Accumulator from Memory (DP Indirect Long)
/// 
/// * Byte length: 2
/// * Cycles: 6
LdaDPLong,

/// ($A8) TAY, Transfer Accumulator to Index Register Y (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Tay,

/// ($A9) LDA, Load Accumulator from Memory (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
LdaImm,

/// ($AA) TAX, Transfer Accumulator to Index Register X (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Tax,

/// ($AB) PLB, Pull Data Bank Register (Stack (Pull))
/// 
/// * Byte length: 1
/// * Cycles: 4
Plb,

/// ($AC) LDY, Load Index Register Y from Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
LdyAbs,

/// ($AD) LDA, Load Accumulator from Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
LdaAbs,

/// ($AE) LDX, Load Index Register X from Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
LdxAbs,

/// ($AF) LDA, Load Accumulator from Memory (Absolute Long)
/// 
/// * Byte length: 4
/// * Cycles: 5
LdaAbsLong,

/// ($B0) BGE, Branch if Carry Set (Program Counter Relative)
/// 
/// * Byte length: 2
/// * Cycles: 2
Bge,

/// ($B1) LDA, Load Accumulator from Memory (DP Indirect Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 5
LdaDPIY,

/// ($B2) LDA, Load Accumulator from Memory (DP Indirect)
/// 
/// * Byte length: 2
/// * Cycles: 5
LdaDPI,

/// ($B3) LDA, Load Accumulator from Memory (SR Indirect Indexed,Y)
/// 
/// * Byte length: 2
/// * Cycles: 7
LdaSRY,

/// ($B4) LDY, Load Index Register Y from Memory (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
LdyDPX,

/// ($B5) LDA, Load Accumulator from Memory (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
LdaDPX,

/// ($B6) LDX, Load Index Register X from Memory (DP Indexed,Y)
/// 
/// * Byte length: 2
/// * Cycles: 4
LdxDPY,

/// ($B7) LDA, Load Accumulator from Memory (DP Indirect Long Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 6
LdaDPIYLong,

/// ($B8) CLV, Clear Overflow Flag (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Clv,

/// ($B9) LDA, Load Accumulator from Memory (Absolute Indexed,Y)
/// 
/// * Byte length: 3
/// * Cycles: 4
LdaAbsY,

/// ($BA) TSX, Transfer Stack Pointer to Index Register X (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Tsx,

/// ($BB) TYX, Transfer Index Register Y to Index Register X (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Tyx,

/// ($BC) LDY, Load Index Register Y from Memory (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 4
LdyAbsX,

/// ($BD) LDA, Load Accumulator from Memory (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 4
LdaAbsX,

/// ($BE) LDX, Load Index Register X from Memory (Absolute Indexed,Y)
/// 
/// * Byte length: 3
/// * Cycles: 4
LdxAbsY,

/// ($BF) LDA, Load Accumulator from Memory (Absolute Long Indexed,X)
/// 
/// * Byte length: 4
/// * Cycles: 5
LdaAbsXLong,

/// ($C0) CPY, Compare Index Register Y with Memory (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
CpyImm,

/// ($C1) CMP, Compare Accumulator with Memory (DP Indexed Indirect,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
CmpDPIX,

/// ($C2) REP, Reset Processor Status Bits (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 3
RepImm,

/// ($C3) CMP, Compare Accumulator with Memory (Stack Relative)
/// 
/// * Byte length: 2
/// * Cycles: 4
CmpSR,

/// ($C4) CPY, Compare Index Register Y with Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
CpyDP,

/// ($C5) CMP, Compare Accumulator with Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
CmpDP,

/// ($C6) DEC, Decrement (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 5
DecDP,

/// ($C7) CMP, Compare Accumulator with Memory (DP Indirect Long)
/// 
/// * Byte length: 2
/// * Cycles: 6
CmpDPLong,

/// ($C8) INY, Increment Index Register Y (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Iny,

/// ($C9) CMP, Compare Accumulator with Memory (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
CmpImm,

/// ($CA) DEX, Decrement Index Register X (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Dex,

/// ($CB) WAI, Wait for Interrupt (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 3
Wai,

/// ($CC) CPY, Compare Index Register Y with Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
CpyAbs,

/// ($CD) CMP, Compare Accumulator with Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
CmpAbs,

/// ($CE) DEC, Decrement (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 6
DecAbs,

/// ($CF) CMP, Compare Accumulator with Memory (Absolute Long)
/// 
/// * Byte length: 4
/// * Cycles: 5
CmpAbsLong,

/// ($D0) BNE, Branch if Not Equal (Program Counter Relative)
/// 
/// * Byte length: 2
/// * Cycles: 2
Bne,

/// ($D1) CMP, Compare Accumulator with Memory (DP Indirect Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 5
CmpDPIY,

/// ($D2) CMP, Compare Accumulator with Memory (DP Indirect)
/// 
/// * Byte length: 2
/// * Cycles: 5
CmpDPI,

/// ($D3) CMP, Compare Accumulator with Memory (SR Indirect Indexed,Y)
/// 
/// * Byte length: 2
/// * Cycles: 7
CmpSRY,

/// ($D4) PEI, Push Effective Indirect Address (Stack (DP Indirect))
/// 
/// * Byte length: 2
/// * Cycles: 6
Pei,

/// ($D5) CMP, Compare Accumulator with Memory (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
CmpDPX,

/// ($D6) DEC, Decrement (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
DecDPX,

/// ($D7) CMP, Compare Accumulator with Memory (DP Indirect Long Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 6
CmpDPIYLong,

/// ($D8) CLD, Clear Decimal Mode Flag (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Cld,

/// ($D9) CMP, Compare Accumulator with Memory (Absolute Indexed,Y)
/// 
/// * Byte length: 3
/// * Cycles: 4
CmpAbsY,

/// ($DA) PHX, Push Index Register X (Stack (Push))
/// 
/// * Byte length: 1
/// * Cycles: 3
Phx,

/// ($DB) STP, Stop Processor (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 3
Stp,

/// ($DC) JML, Jump (Absolute Indirect Long)
/// 
/// * Byte length: 3
/// * Cycles: 6
Jml,

/// ($DD) CMP, Compare Accumulator with Memory (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 4
CmpAbsX,

/// ($DE) DEC, Decrement (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 7
DecAbsX,

/// ($DF) CMP, Compare Accumulator with Memory (Absolute Long Indexed,X)
/// 
/// * Byte length: 4
/// * Cycles: 5
CmpAbsXLong,

/// ($E0) CPX, Compare Index Register X with Memory (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
CpxImm,

/// ($E1) SBC, Subtract with Borrow from Accumulator (DP Indexed Indirect,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
SbcDPIX,

/// ($E2) SEP, Set Processor Status Bits (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 3
SepImm,

/// ($E3) SBC, Subtract with Borrow from Accumulator (Stack Relative)
/// 
/// * Byte length: 2
/// * Cycles: 4
SbcSR,

/// ($E4) CPX, Compare Index Register X with Memory (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
CpxDP,

/// ($E5) SBC, Subtract with Borrow from Accumulator (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 3
SbcDP,

/// ($E6) INC, Increment (Direct Page)
/// 
/// * Byte length: 2
/// * Cycles: 5
IncDP,

/// ($E7) SBC, Subtract with Borrow from Accumulator (DP Indirect Long)
/// 
/// * Byte length: 2
/// * Cycles: 6
SbcDPLong,

/// ($E8) INX, Increment Index Register X (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Inx,

/// ($E9) SBC, Subtract with Borrow from Accumulator (Immediate)
/// 
/// * Byte length: 2
/// * Cycles: 2
SbcImm,

/// ($EA) NOP, No Operation (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Nop,

/// ($EB) XBA, Exchange B and A 8-bit Accumulators (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 3
Xba,

/// ($EC) CPX, Compare Index Register X with Memory (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
CpxAbs,

/// ($ED) SBC, Subtract with Borrow from Accumulator (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 4
SbcAbs,

/// ($EE) INC, Increment (Absolute)
/// 
/// * Byte length: 3
/// * Cycles: 6
IncAbs,

/// ($EF) SBC, Subtract with Borrow from Accumulator (Absolute Long)
/// 
/// * Byte length: 4
/// * Cycles: 5
SbcAbsLong,

/// ($F0) BEQ, Branch if Equal (Program Counter Relative)
/// 
/// * Byte length: 2
/// * Cycles: 2
Beq,

/// ($F1) SBC, Subtract with Borrow from Accumulator (DP Indirect Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 5
SbcDPIY,

/// ($F2) SBC, Subtract with Borrow from Accumulator (DP Indirect)
/// 
/// * Byte length: 2
/// * Cycles: 5
SbcDPI,

/// ($F3) SBC, Subtract with Borrow from Accumulator (SR Indirect Indexed,Y)
/// 
/// * Byte length: 2
/// * Cycles: 7
SbcSRY,

/// ($F4) PEA, Push Effective Absolute Address (Stack (Absolute))
/// 
/// * Byte length: 3
/// * Cycles: 5
Pea,

/// ($F5) SBC, Subtract with Borrow from Accumulator (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 4
SbcDPX,

/// ($F6) INC, Increment (DP Indexed,X)
/// 
/// * Byte length: 2
/// * Cycles: 6
IncDPX,

/// ($F7) SBC, Subtract with Borrow from Accumulator (DP Indirect Long Indexed, Y)
/// 
/// * Byte length: 2
/// * Cycles: 6
SbcDPIYLong,

/// ($F8) SED, Set Decimal Flag (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Sed,

/// ($F9) SBC, Subtract with Borrow from Accumulator (Absolute Indexed,Y)
/// 
/// * Byte length: 3
/// * Cycles: 4
SbcAbsY,

/// ($FA) PLX, Pull Index Register X (Stack (Pull))
/// 
/// * Byte length: 1
/// * Cycles: 4
Plx,

/// ($FB) XCE, Exchange Carry and Emulation Flags (Implied)
/// 
/// * Byte length: 1
/// * Cycles: 2
Xce,

/// ($FC) JSR, Jump to Subroutine (Absolute Indexed Indirect)
/// 
/// * Byte length: 3
/// * Cycles: 8
JsrAbsIX,

/// ($FD) SBC, Subtract with Borrow from Accumulator (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 4
SbcAbsX,

/// ($FE) INC, Increment (Absolute Indexed,X)
/// 
/// * Byte length: 3
/// * Cycles: 7
IncAbsX,

/// ($FF) SBC, Subtract with Borrow from Accumulator (Absolute Long Indexed,X)
/// 
/// * Byte length: 4
/// * Cycles: 5
SbcAbsXLong,


}

impl Instruction {
    /// Returns `Instruction` enum variant from op code
    pub fn from_op(op: u8) -> Instruction {
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
            0x6C => Instruction::JmpIndirect,
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
}
