#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{data, mmio::{self, Dispcnt, Bgxcnt}, input, video};
use boot::entry;

#[entry]
fn main() -> ! {
    let brin_bitmap = include_bytes!("res/brin.img.bin");
    let brin_pal = include_bytes!("res/brin.pal.bin");
    let brin_map = include_bytes!("res/brin.map.bin");

    let chara_block_id = 0;
    let screen_block_id = 30;

    unsafe {
        data::load_tiles(chara_block_id, brin_bitmap);
        data::load_bg_map(screen_block_id, brin_map);
        data::load_bg_palette(0, brin_pal);
    }

    let mmio = mmio::get_mut();
    mmio.bg0cnt.write(Bgxcnt::CHAR_BASE_BLK.val(chara_block_id as u16)
                    + Bgxcnt::SCRN_BASE_BLK.val(screen_block_id as u16)
                    + Bgxcnt::COLORS::Color16_16
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
