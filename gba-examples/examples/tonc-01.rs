#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, mmio::{self, Dispcnt}, util};
use boot::entry;

#[inline]
fn mode3_plot(x: u32, y: u32, color: u16) {
    let offset = (y * consts::SCREEN_WIDTH + x) as isize;
    let vram = consts::MEM_VRAM_START as *mut u16;

    unsafe {
        *vram.offset(offset) = color;
    }
}

#[entry]
fn main() -> ! {
    let mmio = mmio::get_mut();
    mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg2 + Dispcnt::BG_MODE::BitmapMode3);

    mode3_plot(120, 80, util::rgb15(31, 0, 0));
    mode3_plot(136, 80, util::rgb15(0, 31, 0));
    mode3_plot(120, 96, util::rgb15(0, 0, 31));

    loop {}
}
