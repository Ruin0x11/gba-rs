#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, mmio::{self, Dispcnt, Bgxcnt}, input, video};
use boot::entry;
use core::ptr;

#[inline]
fn chara_block(index: usize) -> *mut u16 {
    (consts::VRAM_BG_START + 1024 * index * 16) as *mut u16
}

#[inline]
fn screen_block(index: usize) -> *mut u16 {
    (consts::VRAM_BG_START + 1024 * index * 2) as *mut u16
}

#[entry]
fn main() -> ! {
    let brin_bitmap = include_bytes!("res/brin.img.bin");
    let brin_pal = include_bytes!("res/brin.pal.bin");
    let brin_map = include_bytes!("res/brin.map.bin");

    let chara_block_id = 0;
    let screen_block_id = 30;

    unsafe {
        ptr::copy_nonoverlapping(brin_bitmap.as_ptr() as *const u16,
                                 chara_block(chara_block_id),
                                 brin_bitmap.len() / 2);
        ptr::copy_nonoverlapping(brin_pal.as_ptr() as *const u16,
                                 consts::PAL_BG_START as *mut u16,
                                 brin_pal.len() / 2);
        ptr::copy_nonoverlapping(brin_map.as_ptr() as *const u16,
                                 screen_block(screen_block_id),
                                 brin_map.len() / 2);
    }

    let mmio = mmio::get_mut();
    mmio.bg0cnt.write(Bgxcnt::CHAR_BASE_BLK.val(chara_block_id as u16)
                    + Bgxcnt::SCRN_BASE_BLK.val(screen_block_id as u16)
                    + Bgxcnt::COLORS::COLOR_16_16
                    + Bgxcnt::SIZE_TEXT::Size512_256);
    mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg0 + Dispcnt::BG_MODE::TileMode0);

    let mut x = 192;
    let mut y = 64;

    loop {
        video::vsync();
        input::poll();

        x += input::tri_pad_horz();
        y += input::tri_pad_vert();

        mmio.bg0hofs.set(x as u16);
        mmio.bg0vofs.set(y as u16);
    }
}
