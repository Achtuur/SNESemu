use crate::{nth_bit, bit_slice, to_word, bit_set, fv_blanking, ppu::sprite::Sprite};

pub struct Oam {
    bytes: [u8; 544],
    latch: u8,
    pointer: usize,

    /// Small object size determined by `$2101`,
    pub smallobj_size: (usize, usize),
    /// Big object size determined by `$2101`
    pub bigobj_size: (usize, usize),

    /// Address of page zero
    pub page0_addr: usize,
    /// Address of page 1 is page0 + page1_offs
    /// 
    /// page1_offs is in 8kB increments
    pub page1_offs: usize,

    pub highest_prio_obj: usize,

    pub oamaddl: u8, //$2102
    pub oamaddh: u8, //$2103

    /// When set to true, the ppu object pixels will be updated next tick
    /// 
    /// This is set to true in ppumem when there is a write to oam registers or any oam settings change
    pub update_pending: bool,
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
            update_pending: false,
        }
    }

    /// Return data inside OAM currently as a vector of 128 sprites
    /// 
    /// Vector is ordered in sprite priority, it takes in to account the priority index for priority rotation
    pub fn as_sprites(&self) -> Vec<Sprite> {
        let mut i = self.highest_prio_obj;
        (0..128).map(|_| {
            let mut s = Sprite::new();
            // Get 1st part of sprite info from first 512 bytes
            let idx = 4 * i;

            // low 8 bits of x position
            s.x = self.bytes[idx] as usize;
            // y position
            s.y = self.bytes[idx + 1] as usize;

            // Attribute byte
            let attr = self.bytes[idx + 3];
            // Set tile index using 3rd byte and 1st bit of 4th byte
            s.tile_index = self.bytes[idx + 2] as usize;
            let tile_page = nth_bit!(attr, 0);
            s.tile_index |= (tile_page as usize) << 8;

            // Other attribute bits
            s.palette = bit_slice!(attr, 1, 3) as usize;
            s.priority = bit_slice!(attr, 4, 5) as usize;
            s.flip_h = bit_set!(attr, 6);
            s.flip_v = bit_set!(attr, 7);

            // Get 2nd part of sprite info from last 512 bytes
            let idx2 = 512 + i / 4;
            let shift = (i % 4) * 2;
            let byte = self.bytes[idx2];

            s.x |= (nth_bit!(byte, shift) as usize) << 8;
            s.big_size = bit_set!(byte, shift + 1);

            i = (i + 1) % 128;

            return s;
        }).collect::<Vec<Sprite>>()
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
        if !fv_blanking!() {
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
        if !fv_blanking!() {
            return;
        }

        // Priority rotation
        if bit_set!(self.oamaddh, 7) {
            self.highest_prio_obj = bit_slice!(self.oamaddl, 1, 7) as usize;
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
        let val = if fv_blanking!() {
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
        if fv_blanking!() {
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