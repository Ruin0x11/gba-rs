#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, mmio::{self, Dispcnt}, input::{self, Keyinput}, video};
use boot::entry;
use core::ptr;

#[entry]
fn main() -> ! {
    let mmio = mmio::get_mut();
    let mut mode = 3;
    mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg2 + Dispcnt::BG_MODE.val(mode));

    let modes_bitmap = include_bytes!("res/modes.img.bin");
    let modes_pal = include_bytes!("res/modes.pal.bin");

    unsafe {
        ptr::copy_nonoverlapping(modes_bitmap.as_ptr() as *const u16,
                                 consts::MEM_VRAM_START as *mut u16,
                                 modes_bitmap.len() / 2);
        ptr::copy_nonoverlapping(modes_pal.as_ptr() as *const u16,
                                 consts::MEM_PAL_START as *mut u16,
                                 modes_pal.len() / 2);
    }

    let mut prev_keys = 0;

    loop {
        video::vsync();

        let curr_keys = input::poll();

        if input::was_hit_now(curr_keys, prev_keys, Keyinput::PAD_LEFT) && mode > 3 {
            mode = mode - 1;
        }
        else if input::was_hit_now(curr_keys, prev_keys, Keyinput::PAD_RIGHT) && mode < 5 {
            mode = mode + 1;
        }

        prev_keys = curr_keys;

        mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg2 + Dispcnt::BG_MODE.val(mode));
    }
}
