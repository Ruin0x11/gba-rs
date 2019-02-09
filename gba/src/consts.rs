// Memory addresses
pub const MEM_BIOS_START:  usize = 0x00000000; /// Start address of BIOS ROM.
pub const MEM_BIOS_SIZE:   usize = 0x00003FFF; /// Size of BIOS ROM.
pub const MEM_BIOS_END:    usize = MEM_BIOS_START + MEM_BIOS_SIZE - 1; /// End address of BIOS ROM.

pub const MEM_EWRAM_START: usize = 0x02000000; /// Start address of On-board Work RAM.
pub const MEM_EWRAM_SIZE:  usize = 0x00040000; /// Size of On-board Work RAM.
pub const MEM_EWRAM_END:   usize = MEM_EWRAM_START + MEM_EWRAM_SIZE - 1; /// End address of On-board Work RAM.

pub const MEM_IWRAM_START: usize = 0x03000000; /// Start address of In-chip Work RAM.
pub const MEM_IWRAM_SIZE:  usize = 0x00008000; /// Size of In-chip Work RAM.
pub const MEM_IWRAM_END:   usize = MEM_IWRAM_START + MEM_IWRAM_SIZE - 1; /// End address of In-chip Work RAM.

pub const MEM_IO_START:    usize = 0x04000000; /// Start address of IO register memory.
pub const MEM_IO_SIZE:     usize = 0x000003FF; /// Size of IO register memory.
pub const MEM_IO_END:      usize = MEM_IO_START + MEM_IO_SIZE - 1; /// End address of IO register memory.

pub const MEM_PAL_START:   usize = 0x05000000; /// Start address of BG/OBJ palette memory.
pub const MEM_PAL_SIZE:    usize = 0x00000400; /// Size of BG/OBJ palette memory.
pub const MEM_PAL_END:     usize = MEM_PAL_START + MEM_PAL_SIZE - 1; /// End address of BG/OBJ memory.

pub const MEM_VRAM_START:  usize = 0x06000000; /// Start address of VRAM.
pub const MEM_VRAM_SIZE:   usize = 0x00018000; /// Size of VRAM.
pub const MEM_VRAM_END:    usize = MEM_VRAM_START + MEM_VRAM_SIZE - 1; /// End address of VRAM.

pub const MEM_OAM_START:   usize = 0x07000000; /// Start address of OBJ Attribute memory.
pub const MEM_OAM_SIZE:    usize = 0x00000400; /// Size of OBJ Attribute memory.
pub const MEM_OAM_END:     usize = MEM_OAM_START + MEM_OAM_SIZE - 1; /// End address of OBJ Attribute memory.

// GamePak addresses
pub const PAK_ROM_WAIT0_START: usize = 0x08000000; /// Start address of GamePak ROM (wait state 0).
pub const PAK_ROM_WAIT0_SIZE:  usize = 0x0A000000; /// Size of GamePak ROM (wait state 0).
pub const PAK_ROM_WAIT0_END:   usize = PAK_ROM_WAIT0_START + PAK_ROM_WAIT0_SIZE - 1; /// End address of GamePak ROM (wait state 0).

pub const PAK_ROM_WAIT1_START: usize = 0x0A000000; /// Start address of GamePak ROM (wait state 1).
pub const PAK_ROM_WAIT1_SIZE:  usize = 0x0C000000; /// Size of GamePak ROM (wait state 1).
pub const PAK_ROM_WAIT1_END:   usize = PAK_ROM_WAIT1_START + PAK_ROM_WAIT1_SIZE - 1; /// End address of GamePak ROM (wait state 1).

pub const PAK_ROM_WAIT2_START: usize = 0x0C000000; /// Start address of GamePak ROM (wait state 2).
pub const PAK_ROM_WAIT2_SIZE:  usize = 0x0E000000; /// Size of GamePak ROM (wait state 2).
pub const PAK_ROM_WAIT2_END:   usize = PAK_ROM_WAIT2_START + PAK_ROM_WAIT2_SIZE - 1; /// End address of GamePak ROM (wait state 2).

pub const PAK_RAM_START:       usize = 0x0E000000; /// Start address of GamePak SRAM.
pub const PAK_RAM_SIZE:        usize = 0x00010000; /// Size of GamePak SRAM.
pub const PAK_RAM_END:         usize = PAK_RAM_START + PAK_RAM_SIZE - 1; /// End address of GamePak SRAM.

// Screen
pub const SCREEN_WIDTH:  u32 = 240;
pub const SCREEN_HEIGHT: u32 = 160;
pub const SCREEN_SIZE:   u32 = 240 * 160;

// Page flipping
pub const MODE3_PAGE_SIZE: usize = 0x14000;
pub const MODE3_PAGE:      usize = MEM_VRAM_START;
pub const MODE4_PAGE_SIZE: usize = 0xA000;
pub const MODE4_PAGE1:     usize = MEM_VRAM_START;
pub const MODE4_PAGE2:     usize = MEM_VRAM_START + MODE4_PAGE_SIZE;
pub const OBJ_TILES:       usize = MEM_VRAM_START + MODE4_PAGE_SIZE * 2;

// Key input
pub const REG_KEYINPUT:    usize = MEM_IO_START + 0x130;
