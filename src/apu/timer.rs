/// Abstraction for the inner timer T0, T1 and T2 present in the SPC700.
/// 
/// The `max_counter` can be written to using registers `$00FA` to `$00FC` per timer
/// 
/// The output can be read through `$00FD` to `$00FF` per timer
pub struct Timer {
    /// Hidden counter, these are incremented by one every divided clock cycle
    /// 
    /// For timers 0 and 1 the clockfreq is 8000 Hz
    /// 
    /// For timer 2 the clockfreq is 64000 Hz
    hidden_counter: u8,
    max_counter: u8,
    t_out: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            hidden_counter: 0,
            max_counter: 0,
            t_out: 0,
        }
    }

    pub fn tick(&mut self) {
        self.hidden_counter += 1;
        if self.hidden_counter == self.max_counter {
            self.hidden_counter = 0;
            self.t_out = (self.t_out + 1) % 0xF; // Counters roll back to 0 after reaching 0xF
        }
    }

    pub fn read_tout(&mut self) -> u8 {
        let t_out = self.t_out;
        self.t_out = 0;
        t_out
    }

    pub fn write_tdiv(&mut self, byte: u8) {
        self.max_counter = byte;
    }
}