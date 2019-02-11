use register::{mmio::*, register_bitfields};
use crate::consts;

register_bitfields! [u16,
    /// LCD Control
    Dispcnt [
        BG_MODE   OFFSET(0) NUMBITS(3) [
            TileMode0 = 0,
            TileMode1 = 1,
            TileMode2 = 2,
            BitmapMode3 = 3,
            BitmapMode4 = 4,
            BitmapMode5 = 5
        ],
        GB_MODE   OFFSET(3) NUMBITS(1) [
            Gba = 0,
            Cgb = 1
        ],
        FRM_SEL   OFFSET(4) NUMBITS(1) [
            Frame0 = 0,
            Frame1 = 1
        ],
        OAM_HBL   OFFSET(5) NUMBITS(1) [],
        OBJ_DIM   OFFSET(6) NUMBITS(1) [
            TwoDim = 0,
            OneDim = 1
        ],
        FORCE_HBL OFFSET(7) NUMBITS(1) [],
        SCR_MODE  OFFSET(8) NUMBITS(5) [
            Bg0 = 1,
            Bg1 = 2,
            Bg2 = 4,
            Bg3 = 8,
            Obj = 16
        ],
        WIN_MODE  OFFSET(13) NUMBITS(3) [
            Win0 = 1,
            Win1 = 2,
            Obj = 4
        ]
    ],
    Dispstat [
        VBLANK    OFFSET(0) NUMBITS(1) [],
        HBLANK    OFFSET(1) NUMBITS(1) [],
        VCOUNTER  OFFSET(2) NUMBITS(1) [],
        VBL_IRQ   OFFSET(3) NUMBITS(1) [],
        HBL_IRQ   OFFSET(4) NUMBITS(1) [],
        VCT_IRQ   OFFSET(5) NUMBITS(1) [],
        VCT_OPT   OFFSET(8) NUMBITS(8) []
    ],
    Bgxcnt [
        PRIORITY OFFSET(0) NUMBITS(2) [],
        CHAR_BASE_BLK OFFSET(2) NUMBITS(2) [],
        MOSAIC OFFSET(6) NUMBITS(1) [],
        COLORS OFFSET(7) NUMBITS(1) [
            Color16_16 = 0,
            Color256_1 = 1
        ],
        SCRN_BASE_BLK OFFSET(8) NUMBITS(5) [],
        DISP_OVERFLOW OFFSET(13) NUMBITS(1) [
            Transp = 0,
            Wrap = 1
        ],

        // NOTE: Used when video mode is text
        SIZE_TEXT OFFSET(14) NUMBITS(2) [
            Size256_256 = 0,
            Size512_256 = 1,
            Size256_512 = 2,
            Size512_512 = 3
        ],
        // NOTE: Used when video mode is affine
        SIZE_AFFINE OFFSET(14) NUMBITS(2) [
            Size128_128 = 0,
            Size256_256 = 1,
            Size512_512 = 2,
            Size1024_1024 = 3
        ]
    ]
];

#[repr(C)]
pub struct Mmio {
    pub dispcnt: ReadWrite<u16, Dispcnt::Register>,   // 0x00
    _dummy: u16,
    pub dispstat: ReadWrite<u16, Dispstat::Register>, // 0x04
    pub vcount: ReadOnly<u8>,                         // 0x06
    _dummy2: u8,                                      // 0x07

    pub bg0cnt: ReadWrite<u16, Bgxcnt::Register>,     // 0x08
    pub bg1cnt: ReadWrite<u16, Bgxcnt::Register>,     // 0x0A
    pub bg2cnt: ReadWrite<u16, Bgxcnt::Register>,     // 0x0C
    pub bg3cnt: ReadWrite<u16, Bgxcnt::Register>,     // 0x0E

    pub bg0hofs: WriteOnly<u16>,                      // 0x10
    pub bg0vofs: WriteOnly<u16>,                      // 0x12
    pub bg1hofs: WriteOnly<u16>,                      // 0x14
    pub bg1vofs: WriteOnly<u16>,                      // 0x16
    pub bg2hofs: WriteOnly<u16>,                      // 0x18
    pub bg2vofs: WriteOnly<u16>,                      // 0x1A
    pub bg3hofs: WriteOnly<u16>,                      // 0x1C
    pub bg3vofs: WriteOnly<u16>,                      // 0x1E
}

#[inline]
pub fn get<'a>() -> &'a Mmio {
    let regs = consts::MEM_IO_START as *const Mmio;

    unsafe { &*regs }
}

#[inline]
pub fn get_mut<'a>() -> &'a mut Mmio {
    let regs = consts::MEM_IO_START as *mut Mmio;

    unsafe { &mut *regs }
}
