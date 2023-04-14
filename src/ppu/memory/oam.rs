use crate::{nth_bit, bit_slice, to_word, bit_set};

pub struct Oam {
    bytes: [u8; 544],
    latch: u8,
    pointer: usize,

    /// Small object size determined by `$2101`,
    smallobj_size: (usize, usize),
    /// Big object size determined by `$2101`
    bigobj_size: (usize, usize),

    /// Address of page zero
    page0_addr: usize,
    /// Address of page 1 is page0 + page1_offs
    page1_offs: usize,

    highest_prio_obj: usize,

    oamaddl: u8, //$2102
    oamaddh: u8, //$2103

    is_fvblanking: bool,
}

impl Oam {
    pub fn new() -> Oam {
        Oam {
            bytes: [0; 544],
            latch: 0,
            pointer: 0,
            oamaddh: 0,
            oamaddl: 0,
            smallobj_size: (8, 8),
            bigobj_size: (16, 16),
            page0_addr: 0,
            page1_offs: 0,
            highest_prio_obj: 0,
            is_fvblanking: false,            
        }
    }

    pub fn start_fv_blank(&mut self) {
        self.is_fvblanking = true;
    }
    
    pub fn stop_fv_blank(&mut self) {
        self.is_fvblanking = false;
    }

    pub fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            // OAMDATAREAD
            0x2138 => self.read_data(),
            _ => None,
        }
    }


    pub fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            // OBJSEL
            0x2101 => self.write_objsel(byte),
            // OAMADDL
            0x2102 => {
                self.oamaddl = byte;
                self.process_oamadd();
            },
            // OAMADDH
            0x2103 => {
                self.oamaddh = byte;
                self.process_oamadd();
            },
            // OAMDATA
            0x2104 => self.write_data(byte),
            
            _ => unreachable!(),
        }
    }

    /// 7  bit  0
    /// ---- ----
    /// SSSN NbBB
    /// |||| ||||
    /// |||| |+++- Name base address (word address = bBB << 13)
    /// |||+-+---- Name select (word offset = (NN+1) << 12)
    /// +++------- Object size:
    ///             0:  8x8  and 16x16
    ///             1:  8x8  and 32x32
    ///             2:  8x8  and 64x64
    ///             3: 16x16 and 32x32
    ///             4: 16x16 and 64x64
    ///             5: 32x32 and 64x64
    ///             6: 16x32 and 32x64
    ///             7: 16x32 and 32x32
    fn write_objsel(&mut self, byte: u8) {
        if !self.is_fvblanking {
            return;
        }

        self.smallobj_size = match (byte >> 5) & 0x3 {
            0..=2 => (8, 8),
            3..=4 => (16, 16),
            5 => (32, 32),
            6..=7 => (16, 32),
            _ => unreachable!(),
        };

        self.bigobj_size = match (byte >> 5) & 0x3 {
            0 => (16, 16),
            1 | 3 | 7 => (32, 32),
            2 | 4 | 5 => (64, 64),
            6 => (32, 64),
            _ => unreachable!(),
        };

        self.page1_offs = bit_slice!(byte, 4, 5) as usize + 1;
        self.page1_offs <<= 12;

        // Ignore 3rd bit of base address since it is unused
        self.page0_addr = bit_slice!(byte, 0, 1) as usize;
        self.page0_addr <<= 13;
    }

    ///  OAMADDH     OAMADDL
    ///   $2103       $2102
    /// 7  bit  0   7  bit  0
    /// ---- ----   ---- ----
    /// P... ...B   AAAA AAAA
    /// |       |   |||| ||||
    /// |       |   ++++-++++- OAM word address
    /// |       |   ++++-+++0- OAM priority rotation index
    /// |       +------------- OAM table select (0 = 256 word table, 1 = 16 word table)
    /// +--------------------- OAM priority rotation (1 = enable)
    /// 
    /// On write: Update OAMADD
    ///           internal_oamadd = (OAMADD & $1FF) << 1
    fn process_oamadd(&mut self) {
        if !self.is_fvblanking {
            return;
        }

        // Priority rotation
        if bit_set!(self.oamaddh, 7) {
            self.highest_prio_obj = bit_slice!(self.oamaddl, 0, 6) as usize;
        } else {
            self.highest_prio_obj = 0;
            self.update_pointer();
        }
    }

    fn update_pointer(&mut self) {
        let pointer = to_word!(self.oamaddh, self.oamaddl) as usize;
        self.pointer = (pointer & 0x1FF) << 1;
    }
    
    fn read_data(&mut self) -> Option<u8> {
        // Read val if blanking, else read nothing (emulate open bus)
        let val = if self.is_fvblanking {
            let val = self.bytes[self.pointer];
            Some(val)
        } else {
            None
        };

        self.pointer = self.pointer.wrapping_add(1);
        val
    }

    /// [Source](https://snes.nesdev.org/wiki/PPU_registers#OAMDATA)
    fn write_data(&mut self, byte: u8) {
        if self.is_fvblanking {
            // Write in B part of OAM (write is instant)
            if self.pointer >= 0x200 {
                self.bytes[self.pointer] = byte;
            }
            // Write first byte in A part of OAM
            else if !bit_set!(self.pointer, 0) {
                self.latch = byte;
            }
            // Write second byte in A part of OAM
            else if self.pointer < 0x200 && bit_set!(self.pointer, 1) {
                self.bytes[self.pointer.wrapping_sub(1)] = self.latch;
                self.bytes[self.pointer] = byte;
            }
        }
        self.pointer = self.pointer.wrapping_add(1);
    }
}