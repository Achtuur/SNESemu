use crate::apu::instructions::instructions::Instruction;

pub mod instructions;
pub mod branch;
pub mod jump;
pub mod interrupt;
pub mod stack;

#[derive(Debug, Clone, Copy)]
pub enum AddressingMode {
    /// 8 bit Immediate value
    Immediate,
    DPImmediate,
    Implied,
    /// 8 bit direct page offset, ($0000 + dp or $0100 + dp depending on P flag)
    DirectPage,
    DirectPageX,
    DirectPageY,
    /// 16 bit Absolute address
    Absolute,
    /// 16 bit absolute address + X
    AbsoluteX,
    /// 16 bit absolute address + Y
    AbsoluteY,
    /// 8 bit address with second argument being bit
    AbsoluteBit,
    /// Relative offset in two's complement
    Relative,
    /// DP as first argument, Relative offset as second
    DPRelative,
    /// DP + X as first argument, Relative offset as second
    DPXRelative,
    /// (X), x points to zero page address of data
    XIndirect,
    /// (dp + X)
    IndirectX,
    /// (dp) + Y
    IndirectY,
    /// dp, dp
    DPtoDP,
    XIndirectYIndirect,
    Word,
    SpecialPointer,
    IncrementX
}

impl Instruction {

    fn get_addressing_mode(&self) -> AddressingMode {
        match self {
            Instruction::MOV(a) => *a,
            Instruction::STA(a) => *a,
            Instruction::LDA(a) => *a,
            Instruction::STX(a) => *a,
            Instruction::LDX(a) => *a,
            Instruction::STY(a) => *a,
            Instruction::LDY(a) => *a,
            Instruction::ADC(a) => *a,
            Instruction::SBC(a) => *a,
            Instruction::CMP(a) => *a,
            Instruction::CPX(a) => *a,
            Instruction::CPY(a) => *a,
            Instruction::AND(a) => *a,
            Instruction::OR(a) => *a,
            Instruction::EOR(a) => *a,
            Instruction::ASL(a) => *a,
            Instruction::ASLA(a) => *a,
            Instruction::LSR(a) => *a,
            Instruction::LSRA(a) => *a,
            Instruction::ROL(a) => *a,
            Instruction::ROLA(a) => *a,
            Instruction::ROR(a) => *a,
            Instruction::RORA(a) => *a,
            Instruction::INC(a) => *a,
            Instruction::DEC(a) => *a,
            Instruction::STW(a) => *a,
            Instruction::LDW(a) => *a,
            Instruction::INCW(a) => *a,
            Instruction::DECW(a) => *a,
            Instruction::ADDW(a) => *a,
            Instruction::SUBW(a) => *a,
            Instruction::CMPW(a) => *a,
            Instruction::BRA(a) => *a,
            Instruction::BEQ(a) => *a,
            Instruction::BNE(a) => *a,
            Instruction::BCS(a) => *a,
            Instruction::BCC(a) => *a,
            Instruction::BVS(a) => *a,
            Instruction::BVC(a) => *a,
            Instruction::BMI(a) => *a,
            Instruction::BPL(a) => *a,
            Instruction::CBNE(a) => *a,
            Instruction::DBNZ(a) => *a,
            Instruction::JMP(a) => *a,
            Instruction::CALL(a) => *a,
            Instruction::PCALL(a) => *a,
            Instruction::TSET1(a) => *a,
            Instruction::TCLR1(a) => *a,
            Instruction::AND1(a) => *a,
            Instruction::OR1(a) => *a,
            Instruction::ANDNOT1(a) => *a,
            Instruction::ORNOT1(a) => *a,
            Instruction::EOR1(a) => *a,
            Instruction::NOT1(a) => *a,
            Instruction::ST1(a) => *a,
            Instruction::LD1(a) => *a,
            Instruction::XCN => AddressingMode::Implied,
            Instruction::INX => AddressingMode::Implied,
            Instruction::INY => AddressingMode::Implied,
            Instruction::INA => AddressingMode::Implied,
            Instruction::DEX => AddressingMode::Implied,
            Instruction::DEY => AddressingMode::Implied,
            Instruction::DEA => AddressingMode::Implied,
            Instruction::MUL => AddressingMode::Implied,
            Instruction::DIV => AddressingMode::Implied,
            Instruction::DAA => AddressingMode::Implied,
            Instruction::DAS => AddressingMode::Implied,
            Instruction::BBS(_, _) => AddressingMode::Implied,
            Instruction::BBC(_, _) => AddressingMode::Implied,
            Instruction::TCALL(_, _) => AddressingMode::Implied,
            Instruction::BRK => AddressingMode::Implied,
            Instruction::RET => AddressingMode::Implied,
            Instruction::RETI => AddressingMode::Implied,
            Instruction::PUSHP => AddressingMode::Implied,
            Instruction::PUSHA => AddressingMode::Implied,
            Instruction::PUSHX => AddressingMode::Implied,
            Instruction::PUSHY => AddressingMode::Implied,
            Instruction::POPP => AddressingMode::Implied,
            Instruction::POPA => AddressingMode::Implied,
            Instruction::POPX => AddressingMode::Implied,
            Instruction::POPY => AddressingMode::Implied,
            Instruction::SET1(_, _) => AddressingMode::Implied,
            Instruction::CLR1(_, _) => AddressingMode::Implied,
            Instruction::CLRC => AddressingMode::Implied,
            Instruction::SETC => AddressingMode::Implied,
            Instruction::NOTC => AddressingMode::Implied,
            Instruction::CLRV => AddressingMode::Implied,
            Instruction::SETP => AddressingMode::Implied,
            Instruction::CLRP => AddressingMode::Implied,
            Instruction::EI => AddressingMode::Implied,
            Instruction::DI => AddressingMode::Implied,
            Instruction::NOP => AddressingMode::Implied,
            Instruction::SLEEP => AddressingMode::Implied,
            Instruction::STOP => AddressingMode::Implied,
            Instruction::TAX => AddressingMode::Implied,
            Instruction::TXA => AddressingMode::Implied,
            Instruction::TAY => AddressingMode::Implied,
            Instruction::TYA => AddressingMode::Implied,
            Instruction::TSX => AddressingMode::Implied,
            Instruction::TXS => AddressingMode::Implied,
        }
    }

    pub fn get_length(&self) -> usize {
        use AddressingMode::*;
        
        match self.get_addressing_mode() {

            Implied | XIndirect | XIndirectYIndirect | IncrementX => 1,

            Immediate | DirectPage | DirectPageX | DirectPageY |
            Relative | IndirectX | IndirectY | Word | SpecialPointer => 2,

            Absolute | AbsoluteX | AbsoluteY | AbsoluteBit | 
            DPRelative | DPXRelative | DPtoDP | DPImmediate => 3,
        }
    }

    pub fn get_cycle_time(&self) -> usize {
        match self {
            _ => 2,
        }
    }
}