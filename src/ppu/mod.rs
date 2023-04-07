pub mod memory;
pub mod background;
pub mod mode7;
pub mod window;

use std::sync::{Mutex, Arc};

use crate::{arc_mut};

/// Picture processing unit handles visual stuff
pub struct Ppu {

}


impl Ppu {
    pub fn new() -> Self {
        Ppu {
        }
    }

}