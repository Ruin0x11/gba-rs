#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, data, mmio::{self, Dispcnt}, input::{self, Keyinput}, video};
use gba_boot::entry;
use core::ptr;

fn load_graphics() {
    // VRAM writes must be u16 or u32.
    let page1 = consts::MODE4_PAGE1 as *mut u16;
    let page2 = consts::MODE4_PAGE2 as *mut u16;

    let front_bitmap = include_bytes!("res/front.img.bin").as_ptr() as *const u16;
    let back_bitmap  = include_bytes!("res/back.img.bin").as_ptr() as *const u16;

    for i in 0..16 {
        // copy_nonoverlapping copies by count * size_of<u16>(), so divide the length of the
        // bitmap's scanline by 2 for copying amount.
        unsafe {
            ptr::copy_nonoverlapping(front_bitmap.offset(i * 144 / 2), page1.offset(i * 120), 144 / 2);
            ptr::copy_nonoverlapping(back_bitmap.offset(i * 144 / 2), page2.offset(i * 120), 144 / 2);
        }
    }

    let front_pal = include_bytes!("res/front.pal.bin");

    unsafe {
        data::load_bg_palette(0, front_pal);
    }
}

#[entry]
fn main() -> ! {
    load_graphics();

    let mmio = mmio::get_mut();
    mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg2 + Dispcnt::BG_MODE::BitmapMode4);

    let input = input::get();
    let mut frames = 0;

    loop {
        while !input.is_set(Keyinput::START) {}
        video::vsync();

        frames = frames + 1;
        if frames == 60 {
            frames = 0;
            video::flip_frames();
        }
    }
}
