use crate::{consts, mmio::{self, Dispcnt}, util};

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
    util::flip_flag(&mmio.dispcnt, Dispcnt::FRM_SEL);

    unsafe {
        return VIDEO_PAGE as *const u16;
    }
}
