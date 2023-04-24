use crate::{nth_bit, bit_set};

pub struct Window {
    pub left: u8, // $2126 / $2127 for W1 / W2
    pub right: u8, // $2128 / $2129 for W1 / W2
    
    /// Window enabled per background, bg_enabled[0] is bg1, bg_enabled[1] is bg2 etc.
    pub bg_enabled: [bool; 4],
    /// Window inverted per background, bg_enabled[0] is bg1, bg_enabled[1] is bg2 etc.
    pub bg_inverted: [bool; 4],
    
    pub obj_enabled: bool,
    pub obj_inverted: bool,
    pub clr_enabled: bool,
    pub clr_inverted: bool
}

impl Window {
    pub fn new() -> Window {
        Window {
            left: 0,
            right: 0,
            bg_enabled: [false; 4],
            bg_inverted: [false; 4],
            obj_enabled: false,
            obj_inverted: false,
            clr_enabled: false,
            clr_inverted: false,
            
        }
    }

    /// Reads bits marked by x: `--xx --xx`
    /// 
    /// Window 2 must have `byte` shifted by two to the right
    pub fn write_12sel(&mut self, byte: u8) {
        self.bg_inverted[0] = bit_set!(byte, 0);
        self.bg_enabled[0] = bit_set!(byte, 1);
        self.bg_enabled[1] = bit_set!(byte, 4);
        self.bg_inverted[1] = bit_set!(byte, 5);
    }

    /// Reads bits marked by x: `--xx --xx`
    /// 
    /// Window 2 must have `byte` shifted by two to the right
    pub fn write_34sel(&mut self, byte: u8) {
        self.bg_inverted[2] = bit_set!(byte, 0);
        self.bg_enabled[2] = bit_set!(byte, 1);
        self.bg_enabled[3] = bit_set!(byte, 4);
        self.bg_inverted[4] = bit_set!(byte, 5);
    }

    pub fn write_objsel(&mut self, byte: u8) {
        todo!()
    }

    pub fn write_left_pos(&mut self, byte: u8) {
        self.left = byte;
    }

    pub fn write_right_pos(&mut self, byte: u8) {
        self.right = byte;
    }
}