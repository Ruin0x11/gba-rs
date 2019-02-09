// Memory addresses
pub const MEM_BIOS_START:    usize = 0x00000000; /// Start address of On-board Work RAM.
pub const MEM_BIOS_END:      usize = 0x00003FFF; /// End address of BIOS ROM.

pub const MEM_EWRAM_START:   usize = 0x02000000; /// Start address of On-board Work RAM.
pub const MEM_EWRAM_END:     usize = 0x0203FFFF; /// End address of On-board Work RAM.

pub const MEM_IWRAM_START:   usize = 0x03000000; /// Start address of In-chip Work RAM.
pub const MEM_IWRAM_END:     usize = 0x03007FFF; /// End address of In-chip Work RAM.

pub const MEM_IO_START:      usize = 0x04000000; /// Start address of IO register memory.
pub const MEM_IO_END:        usize = 0x040003FE; /// End address of IO register memory.

pub const MEM_PAL_START:     usize = 0x05000000; /// Start address of BG/OBJ palette memory.
pub const MEM_PAL_END:       usize = 0x050003FF; /// End address of BG/OBJ palette memory.

pub const MEM_VRAM_START:    usize = 0x06000000; /// Start address of VRAM.
pub const MEM_VRAM_END:      usize = 0x06017FFF; /// End address of VRAM.

pub const MEM_OAM_START:     usize = 0x07000000; /// Start address of OBJ Attribute memory.
pub const MEM_OAM_END:       usize = 0x070003FF; /// End address of OBJ Attribute memory.

// GamePak addresses
pub const PAK_ROM_WAIT0_START: usize = 0x08000000; /// Start address of GamePak ROM (wait state 0).
pub const PAK_ROM_WAIT0_END:   usize = 0x09FFFFFF; /// End address of GamePak ROM (wait state 0).
pub const PAK_ROM_WAIT1_START: usize = 0x0A000000; /// Start address of GamePak ROM (wait state 1).
pub const PAK_ROM_WAIT1_END:   usize = 0x0BFFFFFF; /// End address of GamePak ROM (wait state 1).
pub const PAK_ROM_WAIT2_START: usize = 0x0C000000; /// Start address of GamePak ROM (wait state 2).
pub const PAK_ROM_WAIT2_END:   usize = 0x0DFFFFFF; /// End address of GamePak ROM (wait state 2).
pub const PAK_RAM_START:       usize = 0x0E000000; /// Start address of GamePak SRAM.
pub const PAK_RAM_END:         usize = 0x0E00FFFF; /// End address of GamePak SRAM.

// Screen
pub const SCREEN_WIDTH:  u32 = 240;
pub const SCREEN_HEIGHT: u32 = 160;
pub const SCREEN_SIZE:   u32 = 240 * 160;
