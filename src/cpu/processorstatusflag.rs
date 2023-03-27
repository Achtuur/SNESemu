use super::ProcessorStatusFlags;

impl ProcessorStatusFlags {
    pub fn new() -> Self {
        ProcessorStatusFlags::from_bits(0).unwrap()
    }

    pub fn clear_all(&mut self) {
        *self.0.bits_mut() = 0;
    }
}
