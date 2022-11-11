
pub struct Cpu {
    //some registers and pc and stuff

    // Registers
    sp: u16,        // Stack pointer         (Points to the next available(unused) location on the stack.)
    pc: u16,        // Program counter       (Holds the address of the current instruction to execute.)
    acc: u16,       // Accumulator           (This is the math register. It stores one of two operands or the result of most arithmetic and logical operations.)
    p_reg: u8,      // Processor status      (Holds various important flags, see below.)
    d_reg: u16,     // Direct page register  (Used for direct page addressing modes.)
    reg_x: u16,     // Index register X      (Can be used to reference memory, to pass data to memory, or as counters for loops.)
    reg_y: u16,     // Index register Y      (Can be used to reference memory, to pass data to memory, or as counters for loops.)   
    dbr_reg: u8,    // Data bank register    (Holds the default bank for memory transfers.)
    pbr_reg: u8,    // Program bank register (Holds the bank address of all instruction fetches.)
    

    /////////////////////////////////////////////////////
    /// p_reg flag bits
    /// bit 7: N (Negative)
    /// bit 6: V (Overflow)
    /// bit 5: M (Accumulator register size (native mode only) (0 = 16-bit, 1 = 8-bit))
    /// bit 5: B (Break (emulation mode only))
    /// bit 4: X (Index register size (native mode only) (0 = 16-bit, 1 = 8-bit))
    /// bit 3: D (Decimal)
    /// bit 2: I (IRQ disable)
    /// bit 1: Z (Zero)
    /// bit 0: C (Carry)
    /////////////////////////////////////////////////////

    // 128 KB RAM, addresses $7E0000-$7FFFFF
    ram: [u8; 0x1FFF],
}

impl Cpu {

}