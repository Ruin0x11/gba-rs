#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, data, mmio::{self, Dispcnt}, input::{self, Keyinput}, video};
use gba_boot::entry;

#[entry]
fn main() -> ! {
    let mmio = mmio::get_mut();
    let mut mode = 3;
    mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg2 + Dispcnt::BG_MODE.val(mode));

    let modes_bitmap = include_bytes!("res/modes.img.bin");
    let modes_pal = include_bytes!("res/modes.pal.bin");

    unsafe {
        data::load_bg_bitmap(0, modes_bitmap);
        data::load_bg_palette(0, modes_pal);
    }

    let mut prev_keys = 0;

    loop {
        video::vsync();

        let curr_keys = input::poll();

        if input::was_hit_now(curr_keys, prev_keys, Keyinput::PAD_LEFT::SET) && mode > 3 {
            mode = mode - 1;
        }
        else if input::was_hit_now(curr_keys, prev_keys, Keyinput::PAD_RIGHT::SET) && mode < 5 {
            mode = mode + 1;
        }

        unsafe { *(consts::MEM_EWRAM_START as *mut i32) = gba::bios::div(1000, 10).0; }

        prev_keys = curr_keys;

        mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg2 + Dispcnt::BG_MODE.val(mode));
    }
}
