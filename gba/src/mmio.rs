use register::{mmio::*, register_bitfields};
use crate::consts;

register_bitfields! [u16,
    /// LCD Control
    Dispcnt [
        BG_MODE OFFSET(0) NUMBITS(3) [
            TileMode0 = 0,
            TileMode1 = 1,
            TileMode2 = 2,
            BitmapMode3 = 3,
            BitmapMode4 = 4,
            BitmapMode5 = 5
        ],
        CGB_MODE OFFSET(3) NUMBITS(1) [
            Gba = 0,
            Cgb = 1
        ],
        DISP_FRAME OFFSET(4) NUMBITS(1) [
            Frame0 = 0,
            Frame1 = 1
        ],
        HBLANK_INTERVAL_FREE OFFSET(5) NUMBITS(1) [],
        OBJ_CHARA_VRAM_MAPPING OFFSET(6) NUMBITS(1) [
            TwoDimensional = 0,
            OneDimensional = 1
        ],
        FORCED_BLANK OFFSET(7) NUMBITS(1) [],
        SCR_MODE OFFSET(8) NUMBITS(5) [
            Bg0 = 1,
            Bg1 = 2,
            Bg2 = 4,
            Bg3 = 8,
            Obj = 16
        ],
        WIN_0_DISP OFFSET(13) NUMBITS(1) [],
        WIN_1_DISP OFFSET(14) NUMBITS(1) [],
        OBJ_WIN_DISP OFFSET(15) NUMBITS(1) []
    ]
];

#[allow(non_snake_case)]
#[repr(C)]
pub struct Mmio {
    pub dispcnt: ReadWrite<u16, Dispcnt::Register>, // 0x00
}

pub fn get<'a>() -> &'a Mmio {
    let regs = consts::MEM_IO_START as *const Mmio;

    unsafe { &*regs }
}

pub fn get_mut<'a>() -> &'a mut Mmio {
    let regs = consts::MEM_IO_START as *mut Mmio;

    unsafe { &mut *regs }
}
