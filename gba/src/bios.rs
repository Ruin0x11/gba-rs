#[inline]
pub fn soft_reset() {
    unsafe { asm!("swi #0x00"); }
}

#[inline]
pub fn register_ram_reset(flags: u32) {
    unsafe { asm!("swi #0x01" : : "{r0}" (flags)); }
}

#[inline]
pub fn halt() {
    unsafe { asm!("swi #0x02"); }
}

#[inline]
pub fn stop() {
    unsafe { asm!("swi #0x03"); }
}

#[inline]
pub fn intr_wait() {
    unsafe { asm!("swi #0x04"); }
}

#[inline]
pub fn vblank_intr_wait() {
    unsafe { asm!("swi #0x05"); }
}

#[inline]
pub fn div(num: i32, dem: i32) -> (i32, i32, u32) {
    let (res_div, res_mod, res_abs_div): (i32, i32, u32);
    unsafe { asm!("swi #0x06" : "={r0}" (res_div), "={r1}" (res_mod), "={r3}" (res_abs_div) : "{r0}" (num), "{r1}" (dem)); }
    (res_div, res_mod, res_abs_div)
}

#[inline]
pub fn div_arm(dem: i32, num: i32) -> i32 {
    let res: i32;
    unsafe { asm!("swi #0x07" : "={r0}" (res) : "{r0}" (dem), "{r1}" (num)); }
    res
}

#[inline]
pub fn sqrt(num: u32) -> u32 {
    let res: u32;
    unsafe { asm!("swi #0x08" : "={r0}" (res) : "{r0}" (num)); }
    res
}

#[inline]
pub fn arctan(dydx: i16) -> i16  {
    let res: i16;
    unsafe { asm!("swi #0x09" : "={r0}" (res) : "{r0}" (dydx)); }
    res
}

#[inline]
pub fn arctan2(x: i16, y: i16) -> i16 {
    let res: i16;
    unsafe { asm!("swi #0x0A" : "={r0}" (res) : "{r0}" (x), "{r1}" (y)); }
    res
}

#[inline]
pub unsafe fn cpu_set(src: *const u32, dst: *mut u32, mode: u32) {
    asm!("swi #0x0B" : : "{r0}" (src), "{r1}" (dst), "{r2}" (mode));
}

#[inline]
pub unsafe fn cpu_fast_set(src: *const u32, dst: *mut u32, mode: u32) {
    asm!("swi #0x0C" : : "{r0}" (src), "{r1}" (dst), "{r2}" (mode));
}

#[inline]
pub fn bios_checksum() -> u32 {
    let res: u32;
    unsafe { asm!("swi #0x0D" : "={r0}" (res) : : "r1", "r3"); }
    res
}

#[repr(C)]
#[repr(align(4))]
pub struct BitUnpack {
    pub length: u16,
    pub src_width: u8,
    pub dst_width: u8,
    pub offset: u32
}

#[inline]
pub unsafe fn bit_unpack(src: *const u8, dst: *mut u32, data: &BitUnpack) {
    asm!("swi #0x10" : : "{r0}" (src), "{r1}" (dst), "{r2}" (data as *const BitUnpack));
}


#[inline]
pub unsafe fn lz77_uncomp_wram(src: *const u8, dst: *mut u8) {
    asm!("swi #0x11" : : "{r0}" (src), "{r1}" (dst));
}

#[inline]
pub unsafe fn lz77_uncomp_vram(src: *const u8, dst: *mut u8) {
    asm!("swi #0x12" : : "{r0}" (src), "{r1}" (dst));
}

#[inline]
pub unsafe fn huff_uncomp(src: *const u32, dst: *mut u8) {
    asm!("swi #0x13" : : "{r0}" (src), "{r1}" (dst));
}

#[inline]
pub unsafe fn rl_uncomp_wram(src: *const u8, dst: *mut u8) {
    asm!("swi #0x14" : : "{r0}" (src), "{r1}" (dst));
}

#[inline]
pub unsafe fn rl_uncomp_vram(src: *const u8, dst: *mut u8) {
    asm!("swi #0x15" : : "{r0}" (src), "{r1}" (dst));
}

#[inline]
pub unsafe fn diff_8bit_unfilter_wram(src: *const u8, dst: *mut u8) {
    asm!("swi #0x16" : : "{r0}" (src), "{r1}" (dst));
}

#[inline]
pub unsafe fn diff_8bit_unfilter_vram(src: *const u8, dst: *mut u8) {
    asm!("swi #0x17" : : "{r0}" (src), "{r1}" (dst));
}

#[inline]
pub unsafe fn diff_16bit_unfilter(src: *const u8, dst: *mut u8) {
    asm!("swi #0x18" : : "{r0}" (src), "{r1}" (dst));
}
