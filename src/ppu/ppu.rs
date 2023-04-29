use std::{cmp::Ordering, time::Instant};

use crate::{ppu::{SCREEN_WIDTH, tile::Tile}, bit_set, low_byte, high_byte, to_word, nth_bit, main};

use super::{rgb::Rgba, SPpu, sprite::Sprite, memory::PpuMemory, components::{background::Background, colormath::Addend}, layer::{LayerStruct, Layer}, NTSC_SCREEN_HEIGHT, scanline::{HOR_SCANLINES, NTSC_VER_SCANLINES}};


macro_rules! invert_if {
    ($b: expr, $condition: expr) => {{
        if $condition {
            !($b)
        } else {
            $b
        }
    }};
}

enum TileMapQuadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}


impl SPpu {

    pub fn reset(&mut self) {
        self.pixels = vec![Rgba::default(); SCREEN_WIDTH * NTSC_SCREEN_HEIGHT];
        self.scanline.reset();
    }

    /// Single clock cycle of ppu
    pub fn tick(&mut self) {
        // draw pixel at current scanline position
        if self.scanline.x < SCREEN_WIDTH && self.scanline.y < NTSC_SCREEN_HEIGHT {
            let pixel_color = self.draw_pixel();
            self.set_pixel(self.scanline.x, self.scanline.y, pixel_color);
        }
        // move scanline to next position
        self.scanline.goto_next();
    }
    
    /// Draw pixel on position denoted by current state of `self.scanline`
    fn draw_pixel(&mut self) -> Rgba {
        let mem = self.memory.lock().unwrap();
        
        let x_in_w1 = self.scanline.x >= mem.w1.left as usize && self.scanline.x <= mem.w1.right as usize;
        let x_in_w2 = self.scanline.x >= mem.w2.left as usize && self.scanline.x <= mem.w2.right as usize;
        
        // Determine if background, obj and color are treated as being inside window
        let bgx_w = self.bg_layers_in_window(&mem, x_in_w1, x_in_w2);
        let obj_w = self.obj_layer_in_window(&mem, x_in_w1, x_in_w2);
        let clr_w = self.clr_layer_in_window(&mem, x_in_w1, x_in_w2);
        
        // Get main and sub screen layers (they are copies of each other)
        let mut main_layers = self.get_layers(&mem);
        let mut sub_layers = main_layers;

        // Enable and disable main screen layers based on enable flags set by TM and window set by TMW
        main_layers.set_bg1_enabled(mem.bg1.enable_main, mem.ppustate.enable_window_bg_main[0], bgx_w[0]);
        main_layers.set_bg2_enabled(mem.bg2.enable_main, mem.ppustate.enable_window_bg_main[1], bgx_w[1]);
        main_layers.set_bg3_enabled(mem.bg3.enable_main, mem.ppustate.enable_window_bg_main[2], bgx_w[2]);
        main_layers.set_bg4_enabled(mem.bg4.enable_main, mem.ppustate.enable_window_bg_main[3], bgx_w[3]);
        main_layers.set_obj_enabled(mem.ppustate.enable_obj_main, mem.ppustate.enabled_window_obj_main, obj_w);

        // Get highest priority layer that should be drawn on this pixel
        let mut main_screen_layer = main_layers.get_highest_priority_layer(mem.ppustate.background_mode, mem.ppustate.bg3_prio, Rgba::default());

        // Use either fixed color or subscreen as subscreen based on $2130.1
        let mut sub_screen_layer = if matches!(mem.colormath.addend, Addend::FixedColor) {
            Layer::FallBack(mem.colormath.fixed_color)
        } else {  
            // Enable and disable sub screen layers based on enable flags set by TS register and window options set by TSW register
            sub_layers.set_bg1_enabled(mem.bg1.enable_sub, mem.ppustate.enable_window_bg_sub[0], bgx_w[0]);
            sub_layers.set_bg2_enabled(mem.bg2.enable_sub, mem.ppustate.enable_window_bg_sub[1], bgx_w[1]);
            sub_layers.set_bg3_enabled(mem.bg3.enable_sub, mem.ppustate.enable_window_bg_sub[2], bgx_w[2]);
            sub_layers.set_bg4_enabled(mem.bg4.enable_sub, mem.ppustate.enable_window_bg_sub[3], bgx_w[3]);
            sub_layers.set_obj_enabled(mem.ppustate.enable_obj_sub, mem.ppustate.enabled_window_obj_sub,  obj_w);
            
            // Get layers for main and sub screen that have the highest priority and should thus be drawn
            sub_layers.get_highest_priority_layer(mem.ppustate.background_mode, mem.ppustate.bg3_prio, mem.colormath.fixed_color)
        };
        

        // apply math
        if !mem.colormath.apply_color_switch_main(clr_w) {
            main_screen_layer = Layer::FallBack(Rgba::BLACK);
        }

        if !mem.colormath.apply_color_switch_sub(clr_w) {
            sub_screen_layer = Layer::FallBack(Rgba::default());
        }

        let pixel_color = mem.colormath.apply_math(main_screen_layer, sub_screen_layer);

        pixel_color
    }

    fn get_layers(&self, mem: &PpuMemory) -> LayerStruct {
        let mut layers = LayerStruct::new();

        match self.get_bg1_color(&mem) {
            (c, false) => layers.bg1low = c,
            (c, true) => layers.bg1high = c,
        };
        
        match self.get_bg2_color(&mem) {
            (c, false) => layers.bg2low = c,
            (c, true) => layers.bg2high = c,
        };
        
        match self.get_bg3_color(&mem) {
            (c, false) => layers.bg3low = c,
            (c, true) => layers.bg3high = c,
        };
        
        match self.get_bg4_color(&mem) {
            (c, false) => layers.bg4low = c,
            (c, true) => layers.bg4high = c,
        };
        
        let sprites = mem.oam.as_sprites();
        

        (layers.sprite0, layers.sprite0_palette) = self.get_obj_color(&mem, &sprites, 0);
        (layers.sprite1, layers.sprite1_palette) = self.get_obj_color(&mem, &sprites, 1);
        (layers.sprite2, layers.sprite2_palette) = self.get_obj_color(&mem, &sprites, 2);
        (layers.sprite3, layers.sprite3_palette) = self.get_obj_color(&mem, &sprites, 3);

        layers
    }
    
    /// Determine whether the object layer is inside window on this pixel
    fn obj_layer_in_window(&self, mem: &PpuMemory, x_in_w1: bool, x_in_w2: bool) -> bool {
        let obj_w1 = x_in_w1 && mem.w1.obj_enabled;
        let obj_w1 = invert_if!(obj_w1, mem.w1.obj_inverted);
        
        let obj_w2 = x_in_w2 && mem.w2.obj_enabled;
        let obj_w2 = invert_if!(obj_w2, mem.w2.obj_inverted);
        
        mem.ppustate.window_obj_masklogic.mask(obj_w1, obj_w2)
    }
    
    /// Determine whether the color layer is inside window on this pixel
    fn clr_layer_in_window(&self, mem: &PpuMemory, x_in_w1: bool, x_in_w2: bool) -> bool {
        let clr_w1 = x_in_w1 && mem.w1.clr_enabled;
        let clr_w1 = invert_if!(clr_w1, mem.w1.clr_inverted);
        
        let clr_w2 = x_in_w2 && mem.w2.clr_enabled;
        let clr_w2 = invert_if!(clr_w2, mem.w2.clr_inverted);
        
        mem.ppustate.window_clr_masklogic.mask(clr_w1, clr_w2)
    }
    
    /// Determine whether the background layers are inside window on this pixel
    fn bg_layers_in_window(&self, mem: &PpuMemory, x_in_w1: bool, x_in_w2: bool) -> [bool; 4] {
        let mut bgx_w1 = [false; 4];
        let mut bgx_w2 = [false; 4];
        
        // Determine for each of the windows whether the window is enabled for this pixel
        for i in 0..4 {
            bgx_w1[i] = mem.w1.bg_enabled[i] && x_in_w1;
            bgx_w1[i] = invert_if!(bgx_w1[i], mem.w1.bg_inverted[i]);
            
            bgx_w2[i] = mem.w2.bg_enabled[i] && x_in_w2;
            bgx_w2[i] = invert_if!(bgx_w2[i], mem.w2.bg_inverted[i]);
        }
        
        [
        mem.bg1.mask_logic.mask(bgx_w1[0], bgx_w2[0]),
        mem.bg2.mask_logic.mask(bgx_w1[1], bgx_w2[2]),
        mem.bg3.mask_logic.mask(bgx_w1[2], bgx_w2[3]),
        mem.bg4.mask_logic.mask(bgx_w1[3], bgx_w2[3]),
        ]
    }
    
    /// Gets color for `Sprite0..3` layers, where `0..3` denotes the priority
    fn get_obj_color(&self, mem: &PpuMemory, sprites: &[Sprite], priority: usize) -> (Rgba, usize) {
        // Get first sprite that should be currently visible on scanline x and y position
        // iterator returns (Rgba, prio) tuple
        for s in sprites {
            // Priority difference -> wrong sprite
            if s.priority != priority {
                continue;
            }

            // Get sprite tile size          
            let (x_size, y_size) = if s.big_size {
                mem.oam.bigobj_size
            } else {
                mem.oam.smallobj_size
            };
            
            // Check if x and y are within range of current pixel being drawn
            let in_x_range: bool = self.scanline.x >= s.x && self.scanline.x <= s.x + x_size;
            let in_y_range: bool = self.scanline.y >= s.y && self.scanline.y <= s.y + y_size;
            
            if !(in_x_range && in_y_range) {
                continue;
            }
            
            // From here on -> correct sprite found, now finding color of this pixel

            // get x and y coordinates on current sprite
            let sx = s.x + x_size - self.scanline.x;
            let sy = s.y + y_size - self.scanline.y;
            
            let mut tile_index_offset = s.tile_index;
            
            if sx >= 8 {
                tile_index_offset += 0x20;
            }
            
            if sy >= 8 {
                tile_index_offset += 0x200;
            }
            
            // Select from first or second sprite page based on tile index
            let addr = if s.tile_index < 0x100 {
                mem.oam.page0_addr + tile_index_offset * 0x20
            } else {
                mem.oam.page0_addr + mem.oam.page1_offs + (tile_index_offset - 0x100) * 0x20
            };
            
            // read tile form memory
            let color = self.get_4bpp_color(mem, addr as u16, sx % 8, sx % 8, s.palette as u16);
            
            if color != Rgba::default() {
                return (color, s.palette as usize);
            }
        }

        (Rgba::default(), 0)
    }
    
    /// Returns color and priority for current pixel for bg1 layer as a tuple
    fn get_bg1_color(&self, mem: &PpuMemory) -> (Rgba, bool) {
        
        let tile = self.get_tile(mem, &mem.bg1);
        let char_addr = mem.bg1.chr_base_addr + tile.tile_num;
        let (x, y) = self.get_tile_xy(&mem.bg1, tile.h_flip, tile.v_flip, mem.ppustate.mosaic_size);

        
        let c = match mem.ppustate.background_mode {
            // 2 bpp
            0 => self.get_2bpp_color(mem, char_addr, x, y, tile.palette as u16),
            // 4 bpp
            1 | 2 | 5 | 6 => self.get_4bpp_color(mem, char_addr, x, y, tile.palette as u16),
            // 8 bpp (direct color)
            3 | 4 | 7 => {
                if mem.colormath.direct_color_mode {
                    self.get_direct_color(mem, char_addr, x, y)
                } else {
                    self.get_8bpp_color(mem, char_addr, x, y)
                }
            },
            _ => Rgba::default(),
        };
        
        (c, tile.prio)
    }
    
    /// Returns color and priority for current pixel for bg2 layer as a tuple
    fn get_bg2_color(&self, mem: &PpuMemory) -> (Rgba, bool) {
        
        
        let tile = self.get_tile(mem, &mem.bg2);
        let char_addr = mem.bg2.chr_base_addr + tile.tile_num;
        let (x, y) = self.get_tile_xy(&mem.bg2, tile.h_flip, tile.v_flip, mem.ppustate.mosaic_size);
        
        let c = match mem.ppustate.background_mode {
            // mode 0 = 2bpp with offset based on layer
            0 => self.get_2bpp_color(mem, char_addr, x, y, tile.palette as u16 + 0x10),
            // 2 bpp
            4 | 5 => self.get_2bpp_color(mem, char_addr, x, y, tile.palette as u16),
            // 4 bpp
            1 | 2 | 3 => self.get_4bpp_color(mem, char_addr, x, y, tile.palette as u16),
            
            _ => Rgba::default(),
        };
        
        (c, tile.prio)
    }
    
    /// Returns color and priority for current pixel for bg3 layer as a tuple
    fn get_bg3_color(&self, mem: &PpuMemory) -> (Rgba, bool) {
        
        let tile = self.get_tile(mem, &mem.bg3);
        let char_addr = mem.bg3.chr_base_addr + tile.tile_num;
        let (x, y) = self.get_tile_xy(&mem.bg3, tile.h_flip, tile.v_flip, mem.ppustate.mosaic_size);
        
        let c = match mem.ppustate.background_mode {
            // 2 bpp
            0 => self.get_2bpp_color(mem, char_addr, x, y, tile.palette as u16 + 0x20),
            // 4 bpp
            1 => self.get_2bpp_color(mem, char_addr, x, y, tile.palette as u16),
            
            _ => Rgba::default(),
        };
        
        (c, tile.prio)
    }
    
    /// Returns color and priority for current pixel for bg4 layer as a tuple
    fn get_bg4_color(&self, mem: &PpuMemory) -> (Rgba, bool) {
        
        let tile = self.get_tile(mem, &mem.bg4);
        let char_addr = mem.bg4.chr_base_addr + tile.tile_num;
        let (x, y) = self.get_tile_xy(&mem.bg4, tile.h_flip, tile.v_flip, mem.ppustate.mosaic_size);
        
        let c = match mem.ppustate.background_mode {
            // 2 bpp
            0 => self.get_2bpp_color(mem, char_addr, x, y, tile.palette as u16 + 0x30),
            _ => Rgba::default(),
        };
        
        (c, tile.prio)
    }
    
    /// Get x and y coordinate of this pixel on tile, also handles horizontal and vertical flip
    fn get_tile_xy(&self, bg: &Background, h_flip: bool, v_flip: bool, mosaic_size: usize) -> (usize, usize) {
        let (mut x, mut y) = if bg.mosaic {
            // The previous tile is selected in case mosaic size causes tile boundary crossing
            // Meaning that this x/y should check what mosaic block it is and then check where
            // that is on the current tile
            let x = ((self.scanline.x / mosaic_size) * mosaic_size) % bg.char_size as usize;
            let y = ((self.scanline.y / mosaic_size) * mosaic_size) % bg.char_size as usize;
            (x, y)
        } else {
            (self.scanline.x % bg.char_size as usize, self.scanline.y % bg.char_size as usize)
        };

        
        if h_flip {
            x = bg.char_size as usize - x;
        }
        
        if v_flip {
            y = bg.char_size as usize - y;
        }
        
        (x, y)
    }
    
    /// Get address of tilemap entry corresponding to current scanline x and y position for this background
    fn get_tile(&self, mem: &PpuMemory, bg: &Background) -> Tile {
        // Get t_xth and t_yth tile on tilemap
        // These are based on screen coordinates, and will have to be translated to the proper tilemap quadrant
        
        // mosaic effect
        let (bx, by) = if mem.bg1.mosaic {
            let x = ((self.scanline.x + bg.scroll_x as usize) / mem.ppustate.mosaic_size) * mem.ppustate.mosaic_size;
            let y = ((self.scanline.y + bg.scroll_y as usize) / mem.ppustate.mosaic_size) * mem.ppustate.mosaic_size;
            (x, y)
        } else {
            ((self.scanline.x + bg.scroll_x as usize), self.scanline.y + bg.scroll_y as usize)
        };
        
        let tx = bx / bg.char_size as usize;
        let ty = by / bg.char_size as usize;
        
        // Tilemaps are always 32x32, the top left of each quadrant is tile (0, 0) + quad_offset
        // where quad_offset increment by 0x800 for quadrants topleft, topright, bottomleft, bottomright
        // in that order
        let addr = bg.tilemap_vram_addr + match self.get_quadrant(tx, ty, bg) {
            // Top left is 
            TileMapQuadrant::TopLeft => ty * 32 + tx,
            TileMapQuadrant::TopRight => ty * 32 + (tx - 32) + 0x800,
            TileMapQuadrant::BottomLeft => (ty - 32) * 32 + tx + 0x1000,
            TileMapQuadrant::BottomRight => (ty - 32) * 32 + (tx - 32) + 0x1800,
        } as u16;
        
        let tiledata = mem.vram.read(addr);
        Tile::new(tiledata)
    }
    
    /// Returns quadrant in memory of tile in tilemap based on tile x and y position and 
    /// current horizontal/vertical tilemap size settings
    fn get_quadrant(&self, tx: usize, ty: usize, bg: &Background) -> TileMapQuadrant {
        match (bg.horizontal_tilemap_count, bg.vertical_tilemap_count) {
            (0, 0) => TileMapQuadrant::TopLeft,
            (0, 1) => {
                if tx < 32 {
                    return TileMapQuadrant::TopLeft;
                }
                return TileMapQuadrant::BottomLeft;
            },
            (1, 0) => {
                if ty < 32 {
                    return TileMapQuadrant::TopLeft;
                }
                return TileMapQuadrant::BottomRight;
            },
            (1, 1) => {
                match (tx < 32, ty < 32) {
                    (false, false) => TileMapQuadrant::TopLeft,
                    (false, true) => TileMapQuadrant::TopRight,
                    (true, false) => TileMapQuadrant::BottomLeft,
                    (true, true) => TileMapQuadrant::BottomRight,
                }
            },
            _ => unreachable!(),
        }
    }
    
    /// Get color of character at char_addr with pixel coord `x` and `y` using 2bpp palette
    fn get_2bpp_color(&self, mem: &PpuMemory, char_addr: u16, x: usize, y: usize, palette: u16) -> Rgba {
        // In 2bpp, characters are stored as [row0plane1, row0plane0, row1plane1, row1plane0, ...]
        // To get the correct row, 2*y should be added
        let addr_row = char_addr + 2*y as u16;
        // Read plane 1 and 0 at the same time since vram reads a word in this implemenation
        let plane10 = mem.vram.read(addr_row as u16);
        
        // Get 'column' from plane1 and plane0
        let palette_offset = nth_bit!(plane10, x + 8) << 1 | nth_bit!(plane10, x);
        let palette_addr = palette << 2 + palette_offset;
        let color = mem.cgram.read(palette_addr);
        Rgba::from_snes_palette(color)
    }
    
    /// Get color of character at char_addr with pixel coord `x` and `y` using 4bpp palette
    fn get_4bpp_color(&self, mem: &PpuMemory, char_addr: u16, x: usize, y: usize, palette: u16) -> Rgba {
        // In 2bpp, characters are stored as [row0plane1, row0plane0, row1plane1, row1plane0, ...]
        // To get the correct row, 2*y should be added
        let addr_row = char_addr + 2*y as u16;
        
        let plane10 = mem.vram.read(addr_row);
        let plane32 = mem.vram.read(addr_row.wrapping_add(8));
        
        let palette_offset = nth_bit!(plane32, x + 8) << 3 | nth_bit!(plane32, x) << 2 | 
        nth_bit!(plane10, x + 8) << 1 | nth_bit!(plane10, x);
        
        let palette_addr = palette << 4 + palette_offset;
        let color = mem.cgram.read(palette_addr);
        Rgba::from_snes_palette(color)
    }
    
    /// Get color of character at char_addr with pixel coord `x` and `y` using 8bpp palette
    fn get_8bpp_color(&self, mem: &PpuMemory, char_addr: u16, x: usize, y: usize) -> Rgba {
        // In 2bpp, characters are stored as [row0plane1, row0plane0, row1plane1, row1plane0, ...]
        // To get the correct row, 2*y should be added
        let addr_row = char_addr + 2*y as u16;
        
        // Read bitplanes from memory
        let plane10 = mem.vram.read(addr_row);
        let plane32 = mem.vram.read(addr_row.wrapping_add(0x08));
        let plane54 = mem.vram.read(addr_row.wrapping_add(0x10));
        let plane76 = mem.vram.read(addr_row.wrapping_add(0x18));
        
        // Get column for all bitplanes
        let plane10_col = nth_bit!(plane10, x + 8) << 1 | nth_bit!(plane10, x);
        let plane32_col = nth_bit!(plane32, x + 8) << 1 | nth_bit!(plane32, x);
        let plane54_col = nth_bit!(plane54, x + 8) << 1 | nth_bit!(plane54, x);
        let plane76_col = nth_bit!(plane76, x + 8) << 1 | nth_bit!(plane76, x);
        let palette_offset = plane76_col << 6 | plane54_col << 4 | plane32_col << 2 | plane10_col;
        
        let color = mem.cgram.read(palette_offset);
        Rgba::from_snes_palette(color)
    }
    
    /// Direct color mode, similar to 8bpp, 
    /// the difference being that the bitplane is interpreted as a color instead of a palette index
    fn get_direct_color(&self, mem: &PpuMemory, char_addr: u16, x: usize, y: usize) -> Rgba {
        // In 2bpp, characters are stored as [row0plane1, row0plane0, row1plane1, row1plane0, ...]
        // To get the correct row, 2*y should be added
        let addr_row = char_addr + 2*y as u16;
        
        // Read bitplanes from memory
        let plane10 = mem.vram.read(addr_row);
        let plane32 = mem.vram.read(addr_row.wrapping_add(0x08));
        let plane54 = mem.vram.read(addr_row.wrapping_add(0x10));
        let plane76 = mem.vram.read(addr_row.wrapping_add(0x18));
        
        // Get column for all bitplanes
        let plane10_col = nth_bit!(plane10, x + 8) << 1 | nth_bit!(plane10, x);
        let plane32_col = nth_bit!(plane32, x + 8) << 1 | nth_bit!(plane32, x);
        let plane54_col = nth_bit!(plane54, x + 8) << 1 | nth_bit!(plane54, x);
        let plane76_col = nth_bit!(plane76, x + 8) << 1 | nth_bit!(plane76, x);
        
        let color_word = plane76_col << 6 | plane54_col << 4 | plane32_col << 2 | plane10_col;
        Rgba::from_snes_palette(color_word)
    }
    
}
