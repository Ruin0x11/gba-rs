use core::ptr;
use crate::{consts, lut};

#[repr(C)]
#[repr(align(4))]
#[derive(Clone, Copy, Default)]
pub struct ObjAffine {
    _fill0: [u16; 3],
    pub pa: FixedInner,
    _fill1: [u16; 3],
    pub pb: FixedInner,
    _fill2: [u16; 3],
    pub pc: FixedInner,
    _fill3: [u16; 3],
    pub pd: FixedInner,
}

pub type Fixed = fixed::types::I24F8;
pub type FixedInner = fixed::types::I8F8;

impl ObjAffine {
    #[inline]
    pub fn identity(&mut self) {
        self.pa = FixedInner::from_bits(0x0100);
        self.pb = FixedInner::from_bits(0x0);
        self.pc = FixedInner::from_bits(0x0);
        self.pd = FixedInner::from_bits(0x0100);
    }

    pub fn rotate(&mut self, alpha: u16) {
        let s = lut::sin(alpha as usize).wrapping_shl(4).wrapping_to_fixed::<FixedInner>();
        let c = lut::cos(alpha as usize).wrapping_shl(4).wrapping_to_fixed::<FixedInner>();

        self.pa = c;
        self.pb = -s;
        self.pc = s;
        self.pd = c;
    }

    pub fn scale(&mut self, sx: Fixed, sy: Fixed) {
        self.pa = sx.wrapping_to_fixed::<FixedInner>();
        self.pb = FixedInner::from_bits(0x0);
        self.pc = FixedInner::from_bits(0x0);
        self.pd = sy.wrapping_to_fixed::<FixedInner>();
    }

    pub fn shear_x(&mut self, hx: Fixed) {
        self.pa = FixedInner::from_bits(0x0100);
        self.pb = hx.wrapping_to_fixed::<FixedInner>();
        self.pc = FixedInner::from_bits(0x0);
        self.pd = FixedInner::from_bits(0x0100);
    }

    pub fn shear_y(&mut self, hy: Fixed) {
        self.pa = FixedInner::from_bits(0x0100);
        self.pb = FixedInner::from_bits(0x0);
        self.pc = hy.wrapping_to_fixed::<FixedInner>();
        self.pd = FixedInner::from_bits(0x0100);
    }

    pub fn premul(&mut self, src: &ObjAffine) {
        let tmp_a = self.pa;
        let tmp_b = self.pb;
        let tmp_c = self.pc;
        let tmp_d = self.pd;

        self.pa = (src.pa * tmp_a + src.pb * tmp_c).wrapping_to_fixed::<FixedInner>();
        self.pb = (src.pa * tmp_b + src.pb * tmp_d).wrapping_to_fixed::<FixedInner>();
        self.pc = (src.pc * tmp_a + src.pd * tmp_c).wrapping_to_fixed::<FixedInner>();
        self.pd = (src.pc * tmp_b + src.pd * tmp_d).wrapping_to_fixed::<FixedInner>();
    }

    pub fn postmul(&mut self, src: &ObjAffine) {
        let tmp_a = self.pa;
        let tmp_b = self.pb;
        let tmp_c = self.pc;
        let tmp_d = self.pd;

        self.pa = (tmp_a * src.pa + tmp_b * src.pc).wrapping_to_fixed::<FixedInner>();
        self.pb = (tmp_a * src.pb + tmp_b * src.pd).wrapping_to_fixed::<FixedInner>();
        self.pc = (tmp_c * src.pa + tmp_d * src.pc).wrapping_to_fixed::<FixedInner>();
        self.pd = (tmp_c * src.pb + tmp_d * src.pd).wrapping_to_fixed::<FixedInner>();
    }

    #[inline]
    pub fn scale_inv(&mut self, wx: Fixed, wy: Fixed) {
        self.scale((Fixed::from_bits(1 << 24) / wx).wrapping_shr(8),
                   (Fixed::from_bits(1 << 24) / wy).wrapping_shr(8))
    }
}

pub fn copy_slice(objs: &[ObjAffine]) {
    let src = objs.as_ptr() as *const u32;
    let dst = consts::MEM_OAM_START as *mut u32;

    unsafe {
        ptr::copy_nonoverlapping(src, dst, objs.len());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_eq_hex {
        ($a:expr, $b:expr) => { assert_eq!($a, $b, "#{:#X} {:#X}", $a, $b) }
    }

    #[test]
    fn test_rotate() {
        let mut a = ObjAffine::default();

        a.rotate(0x0080);

        assert_eq_hex!(Fixed::from_bits(0x00FF), a.pa);
        assert_eq_hex!(Fixed::from_bits(0x0004), a.pb);
        assert_eq_hex!(Fixed::from_bits(0xFFFC), a.pc);
        assert_eq_hex!(Fixed::from_bits(0x00FF), a.pd);
    }

    #[test]
    fn test_postmul() {
        let mut a = ObjAffine::default();
        a.pa = FixedInner::from_bits(0x41);
        a.pb = FixedInner::from_bits(0x21);
        a.pc = FixedInner::from_bits(0x11);
        a.pd = FixedInner::from_bits(0x01);

        let mut b = ObjAffine::default();
        b.pa = FixedInner::from_bits(0x23);
        b.pb = FixedInner::from_bits(0x05);
        b.pc = FixedInner::from_bits(0x23);
        b.pd = FixedInner::from_bits(0x05);

        a.postmul(&b);

        assert_eq_hex!(Fixed::from_bits(0x000D), a.pa);
        assert_eq_hex!(Fixed::from_bits(0x0001), a.pb);
        assert_eq_hex!(Fixed::from_bits(0x0002), a.pc);
        assert_eq_hex!(Fixed::from_bits(0x0000), a.pd);
    }
}
