#![no_std]
#![no_main]

extern crate panic_halt;

const CHARA_BLOCK: usize = 0;
const SCREEN_BLOCK: usize = 2;

use gba::{bios::{self, BitUnpack}, data, font, mmio::{self, Dispcnt, Bgxcnt},
          tile::Tile4, util};
use gba_boot::entry;

#[entry]
fn main() -> ! {
    let mmio = mmio::get_mut();
    mmio.bg0cnt.write(Bgxcnt::CHAR_BASE_BLK.val(CHARA_BLOCK as u16)
                    + Bgxcnt::SCRN_BASE_BLK.val(SCREEN_BLOCK as u16)
                    + Bgxcnt::COLORS::Color16_16);
    mmio.dispcnt.write(Dispcnt::BG_MODE::TileMode0 + Dispcnt::SCR_MODE::Bg0);

    let toncfont_img = include_bytes!("res/toncfont.img.bin");
    let mut toncfont_unpacked: [u32; 8 * 96] = [0; 8 * 96];

    let bit_unpack = BitUnpack {
        length: 4 * 8 * 96,
        src_width: 1,
        dst_width: 4,
        offset: 0
    };

    unsafe {
        bios::bit_unpack(toncfont_img.as_ptr(),
                         toncfont_unpacked.as_mut_ptr() as *mut u32,
                         &bit_unpack);

        data::load_tile4(0, 0, toncfont_unpacked.as_ptr() as *const Tile4, 96);
        data::load_bg_pal_color(0, 1, util::rgb15(31, 31, 0));
    }

    unsafe {
        font::draw_string(b"hello", 9, 8, 0, SCREEN_BLOCK);
        font::draw_string(b"world!", 9, 10, 0, SCREEN_BLOCK);
    }

    loop {}
}
