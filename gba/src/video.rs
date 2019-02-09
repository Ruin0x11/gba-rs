use crate::mmio::{self, Dispcnt};
use crate::consts;

static mut VIDEO_PAGE: usize = consts::MODE4_PAGE2;

#[inline]
pub fn vsync() {
    let mmio = mmio::get();

    while mmio.vcount.get() >= 160 {} // VDraw
    while mmio.vcount.get() <  160 {} // VBlank
}

#[inline]
pub fn flip_frames() -> *const u16 {
    unsafe {
        VIDEO_PAGE = VIDEO_PAGE ^ consts::MODE4_PAGE_SIZE;
    }

    let mmio = mmio::get_mut();
    let val = mmio.dispcnt.read(Dispcnt::FRM_SEL);
    mmio.dispcnt.modify(Dispcnt::FRM_SEL.val(val ^ 1));

    unsafe {
        return VIDEO_PAGE as *const u16;
    }
}
