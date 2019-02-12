use register::{mmio::*, register_bitfields};
use tock_registers::registers::{Field, FieldValue};
use crate::{consts, util};

const KEY_MASK: u16 = 0x03ff;

register_bitfields! [u16,
    /// Key status
    Keyinput [
        BUTTON_A  OFFSET(0) NUMBITS(1) [],
        BUTTON_B  OFFSET(1) NUMBITS(1) [],
        SELECT    OFFSET(2) NUMBITS(1) [],
        START     OFFSET(3) NUMBITS(1) [],
        PAD_RIGHT OFFSET(4) NUMBITS(1) [],
        PAD_LEFT  OFFSET(5) NUMBITS(1) [],
        PAD_UP    OFFSET(6) NUMBITS(1) [],
        PAD_DOWN  OFFSET(7) NUMBITS(1) [],
        BUTTON_R  OFFSET(8) NUMBITS(1) [],
        BUTTON_L  OFFSET(9) NUMBITS(1) [],

        // for detecting any pad key
        PAD       OFFSET(4) NUMBITS(4) [
            Any = 0x0F
        ],

        // for detecting no key
        NONE      OFFSET(0) NUMBITS(1) []
    ]
];

pub type Key = Field<u16, Keyinput::Register>;
pub type KeyValue = FieldValue<u16, Keyinput::Register>;
pub type Input = ReadOnly<u16, Keyinput::Register>;


#[inline]
pub fn get<'a>() -> &'a Input {
    let input = consts::REG_KEYINPUT as *const Input;

    unsafe { &*input }
}

#[inline]
pub fn poll() -> u16 {
    !(get().get() & KEY_MASK)
}

#[inline]
pub fn was_hit_now(curr: u16, prev: u16, reg: KeyValue) -> bool {
    (curr & !prev) & reg.value != 0
}

#[inline]
pub fn was_released_now(curr: u16, prev: u16, reg: KeyValue) -> bool {
    (!curr & prev) & reg.value != 0
}

#[inline]
pub fn is_held(curr: u16, prev: u16, reg: KeyValue) -> bool {
    (curr & prev) & reg.value != 0
}

#[inline]
pub fn is_down(curr: u16, reg: KeyValue) -> bool {
    curr & reg.value != 0
}

#[inline]
pub fn key(index: usize) -> FieldValue<u16, Keyinput::Register> {
    Key::new(1, index).val(1)
}


#[inline]
pub fn tri_pad_horz() -> i16 {
    util::tri_flag::<Keyinput::Register>(get(), Keyinput::PAD_RIGHT, Keyinput::PAD_LEFT)
}

#[inline]
pub fn tri_pad_vert() -> i16 {
    util::tri_flag::<Keyinput::Register>(get(), Keyinput::PAD_DOWN, Keyinput::PAD_UP)
}

#[inline]
pub fn tri_pad_fire() -> i16 {
    util::tri_flag::<Keyinput::Register>(get(), Keyinput::BUTTON_A, Keyinput::BUTTON_B)
}

#[inline]
pub fn tri_pad_lr() -> i16 {
    util::tri_flag::<Keyinput::Register>(get(), Keyinput::BUTTON_R, Keyinput::BUTTON_L)
}
