pub struct ApuState {
    /// If set to true, reading from $FFC0 to $FFFF will read from boot ROM instead of RAM
    /// 
    /// Writing will always write to RAM, ignoring this bit
    boot_rom_enabled: bool,

}