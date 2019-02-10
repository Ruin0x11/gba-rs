use register::{mmio::*, register_bitfields};
use tock_registers::registers::Field;
use crate::consts;

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
        BUTTON_L  OFFSET(9) NUMBITS(1) []
    ]
];

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
pub fn was_hit_now(curr: u16, prev: u16, reg: Field<u16, Keyinput::Register>) -> bool {
    (curr & !prev) & (reg.mask << reg.shift) != 0
}

#[inline]
pub fn was_released_now(curr: u16, prev: u16, reg: Field<u16, Keyinput::Register>) -> bool {
    (!curr & prev) & (reg.mask << reg.shift) != 0
}

#[inline]
pub fn is_held(curr: u16, prev: u16, reg: Field<u16, Keyinput::Register>) -> bool {
    (curr & prev) & (reg.mask << reg.shift) != 0
}

#[inline]
pub fn key(index: usize) -> Field<u16, Keyinput::Register> {
    Field::<u16, Keyinput::Register>::new(1, index)
}
