
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Accumulator,
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

impl AddressingMode {
    /// Byte length of the ARGUMENT of an instruction with this addressing mode
    pub fn byte_length(&self, acc_8_bit: bool) -> usize {
        match self {
            Self::Accumulator => 0,
            Self::Implied => 0,

            Self::Direct
            | Self::DirectX
            | Self::DirectY=> 1,

            Self::Indirect
            | Self::IndirectX
            | Self::IndirectY => 1,

            Self::IndirectLong
            | Self::IndirectLongY => 1,

            Self::Absolute
            | Self::AbsoluteX
            | Self::AbsoluteY
            | Self::AbsoluteIndirect
            | Self::AbsoluteIndirectX => 2,

            Self::AbsoluteLong
            | Self::AbsoluteLongX
            | Self::AbsoluteIndirectLong => 3,

            Self::Immediate if acc_8_bit => 1,
            Self::Immediate => 2,
            Self::ImmediateLong if acc_8_bit => 2,
            Self::ImmediateLong => 3,

            Self::Relative => 1,
            Self::RelativeLong => 2,

            Self::StackRelative
            | Self::StackRelativeY => 1,
            Self::Move => 2,
        }
    }

    pub fn cycle_count(&self) -> usize {
        match self {
            Self::Accumulator => 2,
            Self::Absolute => 4,
            Self::AbsoluteX => 4,
            Self::AbsoluteY => 4,
            Self::AbsoluteIndirect => 1,
            Self::AbsoluteIndirectX => 2,

            Self::AbsoluteLong => 5,
            Self::AbsoluteLongX => 5,
            Self::AbsoluteIndirectLong => 6,

            Self::Direct => 3,
            Self::DirectX => 4,
            Self::DirectY => 4,

            Self::Indirect => 5,
            Self::IndirectX => 6,
            Self::IndirectY => 5,

            Self::IndirectLong => 6,
            Self::IndirectLongY => 6,

            Self::Implied => 2,
            Self::Immediate => 2,
            Self::ImmediateLong => 2,

            Self::Relative => 3,
            Self::RelativeLong => 4,

            Self::StackRelative => 4,
            Self::StackRelativeY => 7,
            // 7 per byte moved
            Self::Move => 2,
        }
    }
}