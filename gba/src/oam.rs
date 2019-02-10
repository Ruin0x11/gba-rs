use register::{mmio::*, register_bitfields};
use core::ptr;
use crate::consts;

register_bitfields! [u16,
    Attr0 [
        POS_Y OFFSET(0) NUMBITS(8) [],
        ROTATE OFFSET(8) NUMBITS(1) [],

        // NOTE: Used when ROTATE is disabled
        OBJ_DISABLE OFFSET(9) NUMBITS(1) [
            Normal = 0,
            Disabled = 1
        ],

        // NOTE: Used when ROTATE is enabled
        SCALING OFFSET(9) NUMBITS(1) [
            Normal = 0,
            Double = 1
        ],

        OBJ_MODE OFFSET(10) NUMBITS(2) [
            Normal = 0,
            SemiTrans = 1,
            ObjWindow = 2
        ],
        OBJ_MOSAIC OFFSET(12) NUMBITS(1) [],
        COLORS     OFFSET(13) NUMBITS(1) [
            COLOR_16_16 = 0,
            COLOR_256_1 = 1
        ],
        OBJ_SHAPE OFFSET(14) NUMBITS(2) [
            Square = 0,
            Horz = 1,
            Vert = 2
        ]
    ],
    Attr1 [
        POS_X OFFSET(0) NUMBITS(9) [],

        // NOTE: Used when ROTATE is enabled
        ROT_PARAM OFFSET(9) NUMBITS(4) [],

        // NOTE: Used when ROTATE is disabled
        FLIP_HORZ OFFSET(12) NUMBITS(1) [],
        // NOTE: Used when ROTATE is disabled
        FLIP_VERT OFFSET(13) NUMBITS(1) [],

        // OBJ_SHAPE::Square
        OBJ_SIZE OFFSET(14) NUMBITS(2) [
            Square8 = 0,
            Square16 = 1,
            Square32 = 2,
            Square64 = 3
        ],
        // OBJ_SHAPE::Horz
        OBJ_SIZE_V OFFSET(14) NUMBITS(2) [
            Horz16_8 = 0,
            Horz32_8 = 1,
            Horz32_16 = 2,
            Horz64_32 = 3
        ],
        // OBJ_SHAPE::Vert
        OBJ_SIZE_H OFFSET(14) NUMBITS(2) [
            Vert8_16 = 0,
            Vert8_32 = 1,
            Vert16_32 = 2,
            Vert32_64 = 3
        ]
    ],
    Attr2 [
        TILE_ID OFFSET(0) NUMBITS(10) [],
        PRIORITY OFFSET(10) NUMBITS(2) [],

        // NOTE: Not used when COLORS is COLOR_256_1
        PALBANK OFFSET(12) NUMBITS(4) []
    ]
];

pub fn copy_objs(objs: &[ObjAttr]) {
    let src = objs.as_ptr() as *const u32;
    let dst = consts::MEM_OAM_START as *mut u32;

    unsafe {
        ptr::copy_nonoverlapping(src, dst, objs.len());
    }
}

pub fn init_objs(objs: &mut [ObjAttr]) {
    for obj in objs.iter_mut() {
        obj.attr0.write(Attr0::OBJ_DISABLE::Disabled);
    }

    copy_objs(objs);
}

#[repr(C)]
#[repr(align(4))]
pub struct Tile {
    pub data: [u32; 4]
}

#[repr(C)]
#[repr(align(4))]
pub struct ObjAttr {
    pub attr0: ReadWrite<u16, Attr0::Register>,
    pub attr1: ReadWrite<u16, Attr1::Register>,
    pub attr2: ReadWrite<u16, Attr2::Register>,
    _fill: u16,
}

impl ObjAttr {
    #[inline]
    pub fn set_pos(&mut self, x: u16, y: u16) {
        self.attr0.modify(Attr0::POS_Y.val(y));
        self.attr1.modify(Attr1::POS_X.val(x));
    }
}

#[repr(C)]
#[repr(align(4))]
pub struct ObjAffine {
    _fill0: [u16; 3],
    pub pa: i16,
    _fill1: [u16; 3],
    pub pb: i16,
    _fill2: [u16; 3],
    pub pc: i16,
    _fill3: [u16; 3],
    pub pd: i16,
}
