use crate::consts;

#[inline]
pub fn rgb15(r: u16, g: u16, b: u16) -> u16 {
    r | (g << 5) | (b << 10)
}

#[inline]
pub fn plot_pixel(x: u32, y: u32, color: u16) {
    let offset = (y * consts::SCREEN_WIDTH + x) as isize;
    let vram = consts::MEM_VRAM_START as *mut u16;

    unsafe {
        *vram.offset(offset) = color;
    }
}
