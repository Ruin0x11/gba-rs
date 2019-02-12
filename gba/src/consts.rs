// Memory addresses
pub const MEM_BIOS_START:  usize = 0x00000000; /// Start address of BIOS ROM.
pub const MEM_BIOS_SIZE:   usize = 0x00003FFF; /// Size of BIOS ROM.

pub const MEM_EWRAM_START: usize = 0x02000000; /// Start address of On-board Work RAM.
pub const MEM_EWRAM_SIZE:  usize = 0x00040000; /// Size of On-board Work RAM.

pub const MEM_IWRAM_START: usize = 0x03000000; /// Start address of In-chip Work RAM.
pub const MEM_IWRAM_SIZE:  usize = 0x00008000; /// Size of In-chip Work RAM.

pub const MEM_IO_START:    usize = 0x04000000; /// Start address of IO register memory.
pub const MEM_IO_SIZE:     usize = 0x000003FF; /// Size of IO register memory.

pub const MEM_PAL_START:   usize = 0x05000000; /// Start address of BG/OBJ palette memory.
pub const MEM_PAL_SIZE:    usize = 0x00000400; /// Size of BG/OBJ palette memory.

pub const MEM_VRAM_START:  usize = 0x06000000; /// Start address of VRAM.
pub const MEM_VRAM_SIZE:   usize = 0x00018000; /// Size of VRAM.

pub const MEM_OAM_START:   usize = 0x07000000; /// Start address of OBJ Attribute memory.
pub const MEM_OAM_SIZE:    usize = 0x00000400; /// Size of OBJ Attribute memory.

// GamePak addresses
pub const PAK_ROM_WAIT0_START: usize = 0x08000000; /// Start address of GamePak ROM (wait state 0).
pub const PAK_ROM_WAIT0_SIZE:  usize = 0x0A000000; /// Size of GamePak ROM (wait state 0).

pub const PAK_ROM_WAIT1_START: usize = 0x0A000000; /// Start address of GamePak ROM (wait state 1).
pub const PAK_ROM_WAIT1_SIZE:  usize = 0x0C000000; /// Size of GamePak ROM (wait state 1).

pub const PAK_ROM_WAIT2_START: usize = 0x0C000000; /// Start address of GamePak ROM (wait state 2).
pub const PAK_ROM_WAIT2_SIZE:  usize = 0x0E000000; /// Size of GamePak ROM (wait state 2).

pub const PAK_RAM_START:       usize = 0x0E000000; /// Start address of GamePak SRAM.
pub const PAK_RAM_SIZE:        usize = 0x00010000; /// Size of GamePak SRAM.

// Screen
pub const SCREEN_WIDTH:  u32 = 240;
pub const SCREEN_HEIGHT: u32 = 160;
pub const SCREEN_SIZE:   u32 = SCREEN_WIDTH * SCREEN_HEIGHT;

// Page flipping
pub const MODE3_PAGE_SIZE: usize = 0x14000;
pub const MODE3_PAGE:      usize = MEM_VRAM_START;
pub const MODE4_PAGE_SIZE: usize = 0xA000;
pub const MODE4_PAGE1:     usize = MEM_VRAM_START;
pub const MODE4_PAGE2:     usize = MEM_VRAM_START + MODE4_PAGE_SIZE;
pub const OBJ_TILES:       usize = MEM_VRAM_START + MODE4_PAGE_SIZE * 2;

// Key input
pub const REG_KEYINPUT: usize = MEM_IO_START + 0x130;

// VRAM
pub const VRAM_BG_START:  usize = MEM_VRAM_START;
pub const VRAM_BG_SIZE:   usize = 0x10000;
pub const VRAM_OBJ_START: usize = VRAM_BG_START + VRAM_BG_SIZE;
pub const VRAM_OBJ_SIZE:  usize = 0x08000;
pub const VRAM_CHARA_BLOCK_MAX: usize = 4 + 2;
pub const VRAM_SCREEN_BLOCK_MAX: usize = 32;

// PAL
pub const PAL_BG_START:  usize = MEM_PAL_START;
pub const PAL_BG_SIZE:   usize = 0x200;
pub const PAL_OBJ_START: usize = MEM_PAL_START + PAL_BG_SIZE;
pub const PAL_OBJ_SIZE:  usize = 0x200;

// OAM
pub const OAM_MAX_OBJ:    usize = 128;
pub const OAM_MAX_AFFINE: usize = 32;
