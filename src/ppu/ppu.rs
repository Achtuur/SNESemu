use std::{cmp::Ordering, time::Instant, sync::Mutex};

use crate::{ppu::{SCREEN_WIDTH, tile::Tile, NTSC_SCREEN_PIXELS}, bit_set, low_byte, high_byte, to_word, nth_bit, main, wrap_add_lowbyte};

use super::{rgb::Rgba, SPpu, sprite::Sprite, memory::PpuMemory, components::{background::{Background, BackgroundLayer}, colormath::Addend}, layer::{LayerStruct, Layer, BitDepth}, NTSC_SCREEN_HEIGHT, scanline::{HOR_SCANLINES, NTSC_VER_SCANLINES}};

use rayon::prelude::*;


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
        self.pixels = vec![Rgba::default(); NTSC_SCREEN_PIXELS];
        self.bg1_pixels = vec![(Rgba::default(), 0); NTSC_SCREEN_PIXELS];
        self.bg2_pixels = vec![(Rgba::default(), 0); NTSC_SCREEN_PIXELS];
        self.bg3_pixels = vec![(Rgba::default(), 0); NTSC_SCREEN_PIXELS];
        self.bg4_pixels = vec![(Rgba::default(), 0); NTSC_SCREEN_PIXELS];
        self.obj_pixels = vec![(Rgba::default(), 0, 0); NTSC_SCREEN_PIXELS];
        self.scanline.reset();
    }
    
    pub fn debug_tick(&mut self) {
        // rust pls why is this necessary
        let mem_refclone = self.memory.clone();
        let mem = mem_refclone.lock().unwrap();
        
        self.draw_bg(&mem, &mem.bg1);
        self.draw_bg(&mem, &mem.bg2);
        self.draw_bg(&mem, &mem.bg3);
        self.draw_bg(&mem, &mem.bg4);
        self.draw_obj(&mem);
    }
    
    /// Single clock cycle of ppu
    pub fn tick(&mut self) {
        
        let t = Instant::now();
        // rust pls why is this necessary
        let mem_refclone = self.memory.clone();
        let mut mem = mem_refclone.lock().unwrap();
        
        self.draw_bg(&mem, &mem.bg1);
        self.draw_bg(&mem, &mem.bg2);
        self.draw_bg(&mem, &mem.bg3);
        self.draw_bg(&mem, &mem.bg4);
        self.draw_obj(&mem);
        if mem.bg1.update_pending {
            mem.bg1.update_pending = false;
        }
        if mem.bg2.update_pending {
            mem.bg2.update_pending = false;
        }
        if mem.bg3.update_pending {
            mem.bg3.update_pending = false;
        }
        if mem.bg4.update_pending {
            mem.bg4.update_pending = false;
        }
        if mem.oam.update_pending {
            mem.oam.update_pending = false;
        }
        drop(mem); //free mutex for use witin draw_pixel function
        
        // draw pixel at current scanline position
        if self.scanline.x < SCREEN_WIDTH && self.scanline.y < NTSC_SCREEN_HEIGHT {
            let pixel_color = self.draw_pixel();
            self.set_pixel(self.scanline.x, self.scanline.y, pixel_color);
        }
        // move scanline to next position
        self.scanline.goto_next();
        
        println!("t: {0:?}", t.elapsed());
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
        let i = self.scanline.y * NTSC_SCREEN_HEIGHT + self.scanline.x;
        match self.get_bg_pixel(mem, &mem.bg1) {
            (c, 0) => layers.bg1low = c,
            (c, 1) => layers.bg1high = c,
            _ => unreachable!(),
        };
        
        match self.get_bg_pixel(mem, &mem.bg2) {
            (c, 0) => layers.bg2low = c,
            (c, 1) => layers.bg2high = c,
            _ => unreachable!(),
        };
        
        match self.get_bg_pixel(mem, &mem.bg3) {
            (c, 0) => layers.bg3low = c,
            (c, 1) => layers.bg3high = c,
            _ => unreachable!(),
        };
        
        match self.get_bg_pixel(mem, &mem.bg4) {
            (c, 0) => layers.bg4low = c,
            (c, 1) => layers.bg4high = c,
            _ => unreachable!(),
        };
        
        match self.obj_pixels[i] {
            (c, 0, p) => (layers.sprite0, layers.sprite0_palette) = (c, p),
            (c, 1, p) => (layers.sprite1, layers.sprite1_palette) = (c, p),
            (c, 2, p) => (layers.sprite2, layers.sprite2_palette) = (c, p),
            (c, 3, p) => (layers.sprite3, layers.sprite3_palette) = (c, p),
            _ => unreachable!(),
        }
        
        layers
    }
    
    /// Determine whether the object layer is inside window on this pixel
    fn obj_layer_in_window(&self, mem: &PpuMemory, x_in_w1: bool, x_in_w2: bool) -> bool {
        let mut obj_w1 = x_in_w1 && mem.w1.obj_enabled;
        obj_w1 = invert_if!(obj_w1, mem.w1.obj_inverted);
        
        let mut obj_w2 = x_in_w2 && mem.w2.obj_enabled;
        obj_w2 = invert_if!(obj_w2, mem.w2.obj_inverted);
        
        mem.ppustate.window_obj_masklogic.mask(obj_w1, obj_w2)
    }
    
    /// Determine whether the color layer is inside window on this pixel
    fn clr_layer_in_window(&self, mem: &PpuMemory, x_in_w1: bool, x_in_w2: bool) -> bool {
        let mut clr_w1 = x_in_w1 && mem.w1.clr_enabled;
        clr_w1 = invert_if!(clr_w1, mem.w1.clr_inverted);
        
        let mut clr_w2 = x_in_w2 && mem.w2.clr_enabled;
        clr_w2 = invert_if!(clr_w2, mem.w2.clr_inverted);
        
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

    /// Get pixel of current scanline position for background `bg`. Applies effects such as mosaic effect
    fn get_bg_pixel(&self, mem: &PpuMemory, bg: &Background) -> (Rgba, usize) {
        let i = self.scanline.y * SCREEN_WIDTH + self.scanline.x;
        match bg.layer {
            BackgroundLayer::Background1 => {
                if mem.bg1.mosaic {
                    return self.get_mosaic_pixel(&self.bg1_pixels, mem.ppustate.mosaic_size);
                }
                self.bg1_pixels[i]
            },
            BackgroundLayer::Background2 => {
                if mem.bg2.mosaic {
                    return self.get_mosaic_pixel(&self.bg2_pixels, mem.ppustate.mosaic_size);
                }
                self.bg2_pixels[i]
            },
            BackgroundLayer::Background3 => {
                if mem.bg3.mosaic {
                    return self.get_mosaic_pixel(&self.bg3_pixels, mem.ppustate.mosaic_size);
                }
                self.bg3_pixels[i]
            },
            BackgroundLayer::Background4 => {
                if mem.bg4.mosaic {
                    return self.get_mosaic_pixel(&self.bg4_pixels, mem.ppustate.mosaic_size);
                }
                self.bg4_pixels[i]
            },
        }

        // high resolution stuff ?
    }

    fn get_mosaic_pixel(&self, bg_pixels: &[(Rgba, usize)], mosaic_size: usize) -> (Rgba, usize) {
        // Get xth and yth mosaic_size x mosaic_size square based on scanline position
        let x_s = self.scanline.x / mosaic_size;
        let y_s = self.scanline.y / mosaic_size;

        // Get topleft color of that square
        let i_s = y_s * SCREEN_WIDTH + x_s;
        bg_pixels[i_s]
    }
    
    /// Draws all (visible) tiles for a background
    fn draw_bg(&mut self, mem: &PpuMemory, bg: &Background) {
        
        let bg_pixels = match bg.layer {
            BackgroundLayer::Background1 => &mut self.bg1_pixels,
            BackgroundLayer::Background2 => &mut self.bg2_pixels,
            BackgroundLayer::Background3 => &mut self.bg3_pixels,
            BackgroundLayer::Background4 => &mut self.bg4_pixels,
        };
        
        
        // Loop through every 8th or 16th pixel in y direction depending on char size
        (0..NTSC_SCREEN_HEIGHT).step_by(bg.char_size).for_each(|y| {
            (0..SCREEN_WIDTH).step_by(bg.char_size).for_each(|x| {
                // Get tile at current x and y position
                let tile = Self::get_tile(mem, &bg, x, y);
                let char_addr = bg.chr_base_addr + tile.tile_num;
                // Loop through pixels in tile and get colorss
                
                // If char_size is 16x16, then 'neighbouring' tiles need to be used
                // In case of 8x8 tiles, this for loop runs once and uses 'char_addr' as the char address
                for y_i in 0..bg.char_size / 8 {
                    for x_i in 0..bg.char_size / 8 {
                        // Current tile address is base char address or 'neighbours' of it depending on x_i and y_i
                        let mut t_addr = wrap_add_lowbyte!(char_addr, x_i * 0x20);
                        t_addr = wrap_add_lowbyte!(t_addr, y_i * 0x200);
                        
                        let char_rgba = Self::get_bg_tile_color(mem, bg, &tile, t_addr);
                        
                        // base index is current x/y position offset by neighbour tile position
                        let i_base = (y + 8*y_i) * SCREEN_WIDTH + x + 8*x_i;
                        
                        char_rgba.iter().enumerate().for_each(|(idx, clr)| {
                            let x = i_base + idx % 8;
                            let y = i_base + idx / 8;
                            if y < NTSC_SCREEN_HEIGHT && x < SCREEN_WIDTH {
                                let i = y * SCREEN_WIDTH + x;
                                bg_pixels[i] = (*clr, tile.priority);
                            }
                        });
                    }
                }
            });
        });
    }
    
    fn draw_obj(&mut self, mem: &PpuMemory) {
        
        let sprites = mem.oam.as_sprites();
        // Loop in reverse so each next sprite can overwrite the previous one
        // (sprites with lower index have higher priority over sprites with higher index)
        sprites.iter().rev().for_each(|sprite| {
            
            // Get position of sprite
            let x = sprite.x;
            let y = sprite.y;
            
            // Skip sprites that are not currently on screen
            if x > SCREEN_WIDTH || y > NTSC_SCREEN_HEIGHT {
                return;
            }
            
            // Get sprite tile size          
            let (x_size, y_size) = if sprite.big_size {
                mem.oam.bigobj_size
            } else {
                mem.oam.smallobj_size
            };
            // Select from first or second sprite page based on tile index
            let char_addr = if sprite.tile_index < 0x100 {
                mem.oam.page0_addr + sprite.tile_index * 0x20
            } else {
                mem.oam.page0_addr + mem.oam.page1_offs + (sprite.tile_index - 0x100) * 0x20
            };
            
            // If object size larger than 8, 'neighbouring' characters should be drawn as well
            for y_i in 0..y_size/8 {
                for x_i in 0..x_size/8 {
                    let mut t_addr = wrap_add_lowbyte!(char_addr, x_i * 0x20);
                    t_addr = wrap_add_lowbyte!(t_addr, y_i * 0x200);
                    
                    // Get colors of this character and base index in obj_pixels vector
                    // Sprites always use 4bpp
                    let char_rgba = Self::draw_character(
                        mem, 
                        t_addr as u16,
                         BitDepth::FourBpp, 
                         Some(8 + sprite.palette as u16), // Sprites use palettes 8-15, so offset of 8 is added
                         sprite.flip_h, 
                         sprite.flip_v
                    );


                    let i_base = (y + 8*y_i) * SCREEN_WIDTH + x + 8*x_i;
                    
                    
                    char_rgba.iter().enumerate().for_each(|(idx, clr)| {
                        let x = i_base + idx % 8;
                        let y = i_base + idx / 8;
                        if y < NTSC_SCREEN_HEIGHT && x < SCREEN_WIDTH {
                            let i = y * SCREEN_WIDTH + x;
                            self.obj_pixels[i] = (*clr, sprite.priority, sprite.palette);
                        }
                    });
                }
            }
            
        });
    } 
    
    /// Draws a single character at `$char_addr` with specified bitdepth and optional palette
    /// 
    /// Pallette input is required when using `BitDepth::TwoBpp` and `BitDepth::FourBpp`
    fn draw_character(mem: &PpuMemory, char_addr: u16, bitdepth: BitDepth, palette: Option<u16>, flip_h: bool, flip_v: bool) -> Vec<Rgba> {
        let mut v = vec![Rgba::default(); 64];
        
        v.iter_mut().enumerate().for_each(|(idx, clr)| {
            let mut  x = idx % 8;
            let mut y = idx / 8;
            if flip_h { x = 8 - x };
            if flip_v { y = 8 - y };
            
            *clr = match bitdepth {
                BitDepth::TwoBpp => Self::get_2bpp_color(mem, char_addr, x, y, palette.unwrap()),
                BitDepth::FourBpp => Self::get_4bpp_color(mem, char_addr, x, y, palette.unwrap()),
                BitDepth::EightBpp => Self::get_8bpp_color(mem, char_addr, x, y),
                BitDepth::DirectColor => Self::get_direct_color(mem, char_addr, x, y),
            };
        });
        
        v
    }
    
    
    fn get_bg_tile_color(mem: &PpuMemory, bg: &Background, tile: &Tile, char_addr: u16) -> Vec<Rgba> {
        match bg.layer {
            BackgroundLayer::Background1 => Self::get_bg1_tile_color(mem, tile, char_addr),
            BackgroundLayer::Background2 => Self::get_bg2_tile_color(mem, tile, char_addr),
            BackgroundLayer::Background3 => Self::get_bg3_tile_color(mem, tile, char_addr),
            BackgroundLayer::Background4 => Self::get_bg4_tile_color(mem, tile, char_addr),
        }
    }
    
    fn get_bg1_tile_color(mem: &PpuMemory, tile: &Tile, char_addr: u16) -> Vec<Rgba> {
        match mem.ppustate.background_mode {
            // 2 bpp
            0 => Self::draw_character(mem, char_addr, BitDepth::TwoBpp, Some(tile.palette as u16), tile.flip_h, tile.flip_v),
            // 4 bpp
            1 | 2 | 5 | 6 => Self::draw_character(mem, char_addr, BitDepth::FourBpp, Some(tile.palette as u16), tile.flip_h, tile.flip_v),
            // 8 bpp (direct color)
            3 | 4 | 7 => {
                if mem.colormath.direct_color_mode {
                    Self::draw_character(mem, char_addr, BitDepth::DirectColor, None, tile.flip_h, tile.flip_v)
                    
                } else {
                    Self::draw_character(mem, char_addr, BitDepth::EightBpp, None, tile.flip_h, tile.flip_v)
                }
            },
            _ => vec![Rgba::default(); 64],
        }
    }
    
    fn get_bg2_tile_color(mem: &PpuMemory, tile: &Tile, char_addr: u16) -> Vec<Rgba> {
        match mem.ppustate.background_mode {
            // 2 bpp (with palette offset)
            0 => Self::draw_character(mem, char_addr, BitDepth::TwoBpp, Some((tile.palette + 0x10) as u16), tile.flip_h, tile.flip_v),
            // 2 bpp
            4 | 5 => Self::draw_character(mem, char_addr, BitDepth::TwoBpp, Some(tile.palette as u16), tile.flip_h, tile.flip_v),
            // 4 bpp
            1 | 2 | 3 => Self::draw_character(mem, char_addr, BitDepth::FourBpp, Some(tile.palette as u16), tile.flip_h, tile.flip_v),
            
            _ => vec![Rgba::default(); 64],
        }
    }
    
    fn get_bg3_tile_color(mem: &PpuMemory, tile: &Tile, char_addr: u16) -> Vec<Rgba> {
        match mem.ppustate.background_mode {
            // 2 bpp (with palette offset)
            0 => Self::draw_character(mem, char_addr, BitDepth::TwoBpp, Some((tile.palette + 0x20) as u16), tile.flip_h, tile.flip_v),
            // 2 bpp
            1 => Self::draw_character(mem, char_addr, BitDepth::TwoBpp, Some(tile.palette as u16), tile.flip_h, tile.flip_v),
            
            _ => vec![Rgba::default(); 64],
        }
    }
    
    fn get_bg4_tile_color(mem: &PpuMemory, tile: &Tile, char_addr: u16) -> Vec<Rgba> {
        match mem.ppustate.background_mode {
            // 2 bpp (with palette offset)
            0 => Self::draw_character(mem, char_addr, BitDepth::TwoBpp, Some((tile.palette + 0x30) as u16), tile.flip_h, tile.flip_v),
            _ => vec![Rgba::default(); 64],
        }
    }
    
    /// Get address of tilemap entry corresponding to x and y screen position for this background
    fn get_tile(mem: &PpuMemory, bg: &Background, x: usize, y: usize) -> Tile {
        // Get t_xth and t_yth tile on tilemap
        // These are based on screen coordinates, and will have to be translated to the proper tilemap quadrant
        let x = x + bg.scroll_x as usize;
        let y = y + bg.scroll_y as usize;
        
        let tx = x / bg.char_size;
        let ty = y / bg.char_size;
        
        // Tilemaps are always 32x32, the top left of each quadrant is tile (0, 0) + quad_offset
        // where quad_offset increment by 0x800 for quadrants topleft, topright, bottomleft, bottomright
        // in that order
        let addr = bg.tilemap_vram_addr + match Self::get_quadrant(tx, ty, bg) {
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
    fn get_quadrant(tx: usize, ty: usize, bg: &Background) -> TileMapQuadrant {
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
    fn get_2bpp_color(mem: &PpuMemory, char_addr: u16, x: usize, y: usize, palette: u16) -> Rgba {
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
    fn get_4bpp_color(mem: &PpuMemory, char_addr: u16, x: usize, y: usize, palette: u16) -> Rgba {
        // In 2bpp, characters are stored as [row0plane1, row0plane0, row1plane1, row1plane0, ...]
        // To get the correct row, 2*y should be added
        let addr_row = char_addr + 2*y as u16;
        
        let plane10 = mem.vram.read(addr_row);
        // Next plane is in the next 16 bytes, mem.vram.read() reads a long, so 8 addresses should be skipped
        let plane32 = mem.vram.read(addr_row.wrapping_add(0x8));
        
        let palette_offset = nth_bit!(plane32, x + 8) << 3 | nth_bit!(plane32, x) << 2 | 
        nth_bit!(plane10, x + 8) << 1 | nth_bit!(plane10, x);
        
        let palette_addr = palette << 4 + palette_offset;
        let color = mem.cgram.read(palette_addr);
        Rgba::from_snes_palette(color)
    }
    
    /// Get color of character at char_addr with pixel coord `x` and `y` using 8bpp palette
    fn get_8bpp_color(mem: &PpuMemory, char_addr: u16, x: usize, y: usize) -> Rgba {
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
    fn get_direct_color(mem: &PpuMemory, char_addr: u16, x: usize, y: usize) -> Rgba {
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
