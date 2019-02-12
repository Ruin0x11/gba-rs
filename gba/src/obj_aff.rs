use core::cmp;
use crate::{consts, lut};

#[repr(C)]
#[repr(align(4))]
#[derive(Default)]
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

    pub fn copy_from(&mut self, other: &ObjAffine) {
        self.pa = other.pa;
        self.pb = other.pb;
        self.pc = other.pc;
        self.pd = other.pd;
    }

    pub fn rotate(&mut self, alpha: u16) {
        let s = lut::sin(alpha as usize).wrapping_shr(4).wrapping_to_fixed::<FixedInner>();
        let c = lut::cos(alpha as usize).wrapping_shr(4).wrapping_to_fixed::<FixedInner>();

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

        let mul = |a: FixedInner, b: FixedInner| a.wrapping_mul(b);
        let add = |a: FixedInner, b: FixedInner| a.wrapping_add(b);
        let ma = |a, b, c, d| add(mul(a, b), mul(c, d));

        self.pa = ma(src.pa, tmp_a, src.pb, tmp_c).wrapping_to_fixed::<FixedInner>();
        self.pb = ma(src.pa, tmp_b, src.pb, tmp_d).wrapping_to_fixed::<FixedInner>();
        self.pc = ma(src.pc, tmp_a, src.pd, tmp_c).wrapping_to_fixed::<FixedInner>();
        self.pd = ma(src.pc, tmp_b, src.pd, tmp_d).wrapping_to_fixed::<FixedInner>();
    }

    pub fn postmul(&mut self, src: &ObjAffine) {
        let tmp_a = self.pa;
        let tmp_b = self.pb;
        let tmp_c = self.pc;
        let tmp_d = self.pd;

        let mul = |a: FixedInner, b: FixedInner| a.wrapping_mul(b);
        let add = |a: FixedInner, b: FixedInner| a.wrapping_add(b);
        let ma = |a, b, c, d| add(mul(a, b), mul(c, d));

        self.pa = ma(tmp_a, src.pa, tmp_b, src.pc).wrapping_to_fixed::<FixedInner>();
        self.pb = ma(tmp_a, src.pb, tmp_b, src.pd).wrapping_to_fixed::<FixedInner>();
        self.pc = ma(tmp_c, src.pa, tmp_d, src.pc).wrapping_to_fixed::<FixedInner>();
        self.pd = ma(tmp_c, src.pb, tmp_d, src.pd).wrapping_to_fixed::<FixedInner>();
    }

    #[inline]
    pub fn scale_inv(&mut self, wx: Fixed, wy: Fixed) {
        self.scale((Fixed::from_bits(1 << 24).wrapping_div(wx)).wrapping_shr(16),
                   (Fixed::from_bits(1 << 24).wrapping_div(wy)).wrapping_shr(16))
    }
}

pub fn copy_slice(attrs: &[ObjAffine]) {
    let mut src = attrs.as_ptr() as *const ObjAffine;
    let mut dst = consts::MEM_OAM_START as *mut ObjAffine;

    for _ in 0..(cmp::min(attrs.len(), consts::OAM_MAX_AFFINE)) {
        unsafe {
            (*dst).pa = (*src).pa;
            (*dst).pb = (*src).pb;
            (*dst).pc = (*src).pc;
            (*dst).pd = (*src).pd;
            src = src.add(1);
            dst = dst.add(1);
        }
    }
}

pub fn init_slice(attrs: &mut [ObjAffine]) {
    for attr in attrs.iter_mut() {
        attr.identity();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(overflowing_literals)]
    fn test_rotate() {
        let mut a = ObjAffine::default();

        a.rotate(0xFF80);

        assert_eq!(0x000000FF, a.pa.to_bits());
        assert_eq!(0x00000004, a.pb.to_bits());
        assert_eq!(0x0000FFFC, a.pc.to_bits());
        assert_eq!(0x000000FF, a.pd.to_bits());
    }

    #[test]
    #[allow(overflowing_literals)]
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

        assert_eq!(0x0000000D, a.pa.to_bits());
        assert_eq!(0x00000001, a.pb.to_bits());
        assert_eq!(0x00000002, a.pc.to_bits());
        assert_eq!(0x00000000, a.pd.to_bits());
    }

    #[test]
    fn test_postmul_wrap() {
        let mut a = ObjAffine::default();
        let mut b = ObjAffine::default();

        a.identity();
        b.identity();

        let mut aff_value = Fixed::from_int(1);

        for _ in 0..100 {
            aff_value += Fixed::from_bits(-4);
            b.scale_inv(Fixed::from_int(1).wrapping_sub(aff_value), Fixed::from_int(1));
            a.postmul(&b);
        }
    }
}
