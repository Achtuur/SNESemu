pub struct Window {
    left: u8, // $2126 / $2127 for W1 / W2
    right: u8, // $2128 / $2129 for W1 / W2
    wbglog: u8, // $212A
    wobjlog: u8, // $212b
    
    bg1_enabled: bool,
    bg1_inverted: bool,
    bg2_enabled: bool,
    bg2_inverted: bool,
    bg3_inverted: bool,
    bg3_enabled: bool,
    bg4_inverted: bool,
    bg4_enabled: bool,

    obj_enabled: bool,
    obj_inverted: bool,
    clr_enabled: bool,
    clr_inverted: bool
}

impl Window {
    pub fn new() -> Window {
        Window {
            left: 0,
            right: 0,
            wbglog: 0,
            wobjlog: 0,
            bg1_enabled: false,
            bg1_inverted: false,
            bg2_enabled: false,
            bg2_inverted: false,
            bg3_enabled: false,
            bg3_inverted: false,
            bg4_enabled: false,
            bg4_inverted: false,
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
        self.bg1_inverted = ((byte >> 0) & 0x1) == 1;
        self.bg1_enabled = ((byte >> 1) & 0x1) == 1;
        self.bg2_enabled = ((byte >> 4) & 0x1) == 1;
        self.bg2_inverted = ((byte >> 5) & 0x1) == 1;
    }

    /// Reads bits marked by x: `--xx --xx`
    /// 
    /// Window 2 must have `byte` shifted by two to the right
    pub fn write_34sel(&mut self, byte: u8) {
        self.bg3_inverted = ((byte >> 0) & 0x1) == 1;
        self.bg3_enabled = ((byte >> 1) & 0x1) == 1;
        self.bg4_enabled = ((byte >> 4) & 0x1) == 1;
        self.bg4_inverted = ((byte >> 5) & 0x1) == 1;
    }

    pub fn write_objsel(&mut self, byte: u8) {
        
    }

    pub fn write_left_pos(&mut self, byte: u8) {
        self.left = byte;
    }

    pub fn write_right_pos(&mut self, byte: u8) {
        self.right = byte;
    }

    pub fn write_wbglog(&mut self, byte: u8) {
        self.wbglog = byte;
    }

    pub fn write_wobjlog(&mut self, byte: u8) {
        self.wobjlog = byte;
    }

}