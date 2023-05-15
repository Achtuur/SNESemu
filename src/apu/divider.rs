const INPUT_CLK_FREQ_HZ: usize = 2048000;

pub const MAX_COUNTER_8000_HZ: usize = INPUT_CLK_FREQ_HZ / 8000;

pub const MAX_COUNTER_64000_HZ: usize = INPUT_CLK_FREQ_HZ / 64000;

/// Clock divider, `max_counter` (and thus reduced clk freq) can only be set upon instantiating divider
/// 
/// Input clock frequency is [INPUT_CLK_FREQ_HZ] (= 2048000 Hz), meaning that a value of `256` for `max_counter` will give an output clock freq of `8000 Hz`
pub struct Divider {
    /// Interal counter that counts the 2048000Hz clock from the SNES
    counter: usize,
    /// Maximum value of counter, when `counter == max_counter` then `clk_out = ~clk_out`
    max_counter: usize,
    /// Output of divider, true if clk signal high, false if clock signal low
    pub clk_out: bool,
}

impl Divider {
    pub fn new(max_counter: usize) -> Divider {
        Divider {
            counter: 0,
            max_counter,
            clk_out: false,
        }
    }

    /// Reset internal counter, does not reset `max_counter`
    pub fn reset(&mut self) {
        self.counter = 0;
        self.clk_out = false;
    }

    /// Tick this divider, returns true if rising/falling edge, false otherwise
    pub fn tick(&mut self) -> bool {
        self.counter += 1;

        if self.counter == self.max_counter {
            self.counter = 0;
            self.clk_out = !self.clk_out;
            return true;
        }

        false
    }
}

