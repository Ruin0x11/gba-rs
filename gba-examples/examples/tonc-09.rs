#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{data, mmio::{self, Dispcnt, Bgxcnt}, tile::{Tile4, Tile8, ScrEntry, Scrdata}};
use boot::entry;
use core::slice;

const CBLOCK_4BPP: usize = 0;
const SBLOCK_4BPP: usize = 2;

const CBLOCK_8BPP: usize = 2;
const SBLOCK_8BPP: usize = 4;

#[inline]
unsafe fn screen_entries<'a>(index: usize, offset: usize) -> &'a mut [ScrEntry] {
    let ptr = data::screen_block(index) as *mut ScrEntry;

    slice::from_raw_parts_mut(ptr.add(offset), 0x1000)
}

#[inline]
fn load_tiles() {
    let ids4_tiles = include_bytes!("res/ids4.img.bin").as_ptr() as *const Tile4;
    let ids8_tiles = include_bytes!("res/ids8.img.bin").as_ptr() as *const Tile8;

    unsafe {
        data::load_tile4(0, 1, ids4_tiles.add(1));
        data::load_tile4(0, 2, ids4_tiles.add(2));
        data::load_tile4(1, 0, ids4_tiles.add(3));
        data::load_tile4(1, 1, ids4_tiles.add(4));

        data::load_tile8(2, 1, ids8_tiles.add(1));
        data::load_tile8(2, 2, ids8_tiles.add(2));
        data::load_tile8(3, 0, ids8_tiles.add(3));
        data::load_tile8(3, 1, ids8_tiles.add(4));
        data::load_tile8(4, 0, ids8_tiles.add(5));
        data::load_tile8(4, 1, ids8_tiles.add(6));
        data::load_tile8(5, 0, ids8_tiles.add(7));
        data::load_tile8(5, 1, ids8_tiles.add(8));
    }

    let ids4_pal = include_bytes!("res/ids4.pal.bin");

    unsafe {
        data::load_bg_palette(0, ids4_pal);
        data::load_obj_palette(0, ids4_pal);
    }
}

#[inline]
fn init_maps() {
    let se4 = unsafe { screen_entries(SBLOCK_4BPP, 2 * 32) };
    let se8 = unsafe { screen_entries(SBLOCK_8BPP, 8 * 32) };

    se4[0x01].write(Scrdata::TILE_ID.val(0x1));
    se4[0x02].write(Scrdata::TILE_ID.val(0x2));

    se4[0x20].write(Scrdata::TILE_ID.val(0x200));
    se4[0x21].write(Scrdata::TILE_ID.val(0x201));

    se8[0x01].write(Scrdata::TILE_ID.val(0x1));
    se8[0x02].write(Scrdata::TILE_ID.val(0x2));

    se8[0x20].write(Scrdata::TILE_ID.val(0x100));
    se8[0x21].write(Scrdata::TILE_ID.val(0x101));

    se8[0x40].write(Scrdata::TILE_ID.val(0x200));
    se8[0x41].write(Scrdata::TILE_ID.val(0x201));

    se8[0x60].write(Scrdata::TILE_ID.val(0x300));
    se8[0x61].write(Scrdata::TILE_ID.val(0x301));
}

#[entry]
fn main() -> ! {
    load_tiles();
    init_maps();

    let mmio = mmio::get_mut();
    mmio.bg0cnt.write(Bgxcnt::CHAR_BASE_BLK.val(CBLOCK_4BPP as u16)
                    + Bgxcnt::SCRN_BASE_BLK.val(SBLOCK_4BPP as u16)
                    + Bgxcnt::COLORS::Color16_16);
    mmio.bg1cnt.write(Bgxcnt::CHAR_BASE_BLK.val(CBLOCK_8BPP as u16)
                    + Bgxcnt::SCRN_BASE_BLK.val(SBLOCK_8BPP as u16)
                    + Bgxcnt::COLORS::Color256_1);
    mmio.dispcnt.write(Dispcnt::BG_MODE::TileMode0
                     + Dispcnt::SCR_MODE::Bg0
                     + Dispcnt::SCR_MODE::Bg1
                     + Dispcnt::SCR_MODE::Obj);

    loop {}
}
