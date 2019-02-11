#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, data, mmio::{self, Dispcnt}, input, util, video};
use gba_boot::entry;

const KEY_MAX: usize = 10;
const BTN_PAL_ID: usize = 5;

fn write_palette(index: usize, color: u16) {
    unsafe {
        *((consts::MEM_PAL_START + index * 2) as *mut u16) = color;
    }
}

#[entry]
fn main() -> ! {
    let gba_pic_bitmap = include_bytes!("res/gba_pic.img.bin");
    let gba_pic_pal = include_bytes!("res/gba_pic.pal.bin");

    unsafe {
        data::load_bg_bitmap(0, gba_pic_bitmap);
        data::load_bg_palette(0, gba_pic_pal);
    }

    let mmio = mmio::get_mut();
    mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg2 + Dispcnt::BG_MODE::BitmapMode4);

    write_palette(0, 0);

    let mut curr_keys = 0;
    let mut prev_keys = 0;
    let mut frame = 0;

    loop {
        video::vsync();

        if frame & 7 == 0 {
            prev_keys = curr_keys;
            curr_keys = input::poll();
        }

        for i in 0..KEY_MAX {
            let key = input::key(i);

            let color = if input::was_hit_now(curr_keys, prev_keys, key) {
                util::rgb15(31, 0, 0)
            }
            else if input::was_released_now(curr_keys, prev_keys, key) {
                util::rgb15(31, 31, 0)
            }
            else if input::is_held(curr_keys, prev_keys, key) {
                util::rgb15(0, 31, 15)
            }
            else {
                util::rgb15(27, 27, 29)
            };

            write_palette(i + BTN_PAL_ID, color);
        }

        frame = frame + 1;
    }
}
