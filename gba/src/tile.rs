use register::{mmio::*, register_bitfields};

register_bitfields! [u16,
    Scrdata [
         // NOTE: Used when video mode is text
        TILE_ID_TEXT OFFSET(0) NUMBITS(8) [],
         // NOTE: Used when video mode is affine
        TILE_ID_AFFINE OFFSET(0) NUMBITS(10) [],

        FLIP_HORZ OFFSET(10) NUMBITS(1) [],
        FLIP_VERT OFFSET(11) NUMBITS(1) [],
        PALBANK OFFSET(12) NUMBITS(4) []
    ]
];

pub type ScrEntry = ReadWrite<u16, Scrdata::Register>;

#[repr(C)]
#[repr(align(4))]
pub struct Tile {
    pub data: [u32; 4]
}

#[repr(C)]
#[repr(align(4))]
pub struct Tile8 {
    pub data: [u32; 8]
}
