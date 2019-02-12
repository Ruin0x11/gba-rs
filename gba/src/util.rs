use register::mmio::*;
use tock_registers::registers::{Field, RegisterLongName};

#[inline]
pub const fn rgb15(r: u16, g: u16, b: u16) -> u16 {
    r | (g << 5) | (b << 10)
}

#[inline]
pub fn tri_flag<R>(reg: &ReadOnly<u16, R>, neg: Field<u16, R>, pos: Field<u16, R>) -> i16
where R: RegisterLongName + Clone
{
    (reg.read(pos) & 1) as i16 - (reg.read(neg) & 1) as i16
}

#[inline]
pub fn flip_flag<R>(reg: &ReadWrite<u16, R>, flag: Field<u16, R>)
    where R: RegisterLongName + Clone
{
    let copy = reg.extract();
    let val = reg.read(flag.clone());
    reg.modify_no_read(copy, flag.val(val ^ 1));
}
