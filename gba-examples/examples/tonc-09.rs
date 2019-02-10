#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, mmio::{self, Dispcnt, Bgxcnt}, tile::{Tile4, Tile8, ScrEntry, Scrdata}};
use boot::entry;
use core::{ptr, slice};

const CBLOCK_4BPP: u16 = 0;
const SBLOCK_4BPP: u16 = 2;

const CBLOCK_8BPP: u16 = 2;
const SBLOCK_8BPP: u16 = 4;

#[inline]
fn screen_block<'a>(index: isize, offset: isize) -> &'a mut [ScrEntry] {
    let ptr = consts::VRAM_BG_START as *mut ScrEntry;

    unsafe {
        slice::from_raw_parts_mut(ptr.offset(index * 1024 + offset), 0x1000)
    }
}

#[inline]
fn load_tiles() {
    let ids4_tiles = include_bytes!("res/ids4.img.bin").as_ptr() as *const Tile4;
    let ids8_tiles = include_bytes!("res/ids8.img.bin").as_ptr() as *const Tile8;

    let copy = |dst_bank: isize, dst_id: isize, src_idx: isize| {
        let offset = consts::VRAM_BG_START as *mut Tile4;

        unsafe {
            let src = ids4_tiles.offset(src_idx);
            let dst = offset.offset(dst_bank * 512 + dst_id);
            core::ptr::copy_nonoverlapping(src, dst, 1);
        }
    };

    let copy8 = |dst_bank: isize, dst_id: isize, src_idx: isize| {
        let offset = consts::VRAM_BG_START as *mut Tile8;

        unsafe {
            let src = ids8_tiles.offset(src_idx);
            let dst = offset.offset(dst_bank * 256 + dst_id);
            core::ptr::copy_nonoverlapping(src, dst, 1);
        }
    };

    copy(0, 1, 1);
    copy(0, 2, 2);
    copy(1, 0, 3);
    copy(1, 1, 4);

    copy8(2, 1, 1);
    copy8(2, 2, 2);
    copy8(3, 0, 3);
    copy8(3, 1, 4);
    copy8(4, 0, 5);
    copy8(4, 1, 6);
    copy8(5, 0, 7);
    copy8(5, 1, 8);

    let ids4_pal = include_bytes!("res/ids4.pal.bin");
    let ids4_pal_ptr = ids4_pal.as_ptr() as *const u32;

    unsafe {
        ptr::copy_nonoverlapping(ids4_pal_ptr, consts::PAL_BG_START as *mut u32, ids4_pal.len() / 4);
        ptr::copy_nonoverlapping(ids4_pal_ptr, consts::PAL_OBJ_START as *mut u32, ids4_pal.len() / 4);
    }
}

#[inline]
fn init_maps() {
    let se4 = screen_block(SBLOCK_4BPP as isize, 2 * 32);
    let se8 = screen_block(SBLOCK_8BPP as isize, 8 * 32);

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
    mmio.bg0cnt.write(Bgxcnt::CHAR_BASE_BLK.val(CBLOCK_4BPP)
                    + Bgxcnt::SCRN_BASE_BLK.val(SBLOCK_4BPP)
                    + Bgxcnt::COLORS::Color16_16);
    mmio.bg1cnt.write(Bgxcnt::CHAR_BASE_BLK.val(CBLOCK_8BPP)
                    + Bgxcnt::SCRN_BASE_BLK.val(SBLOCK_8BPP)
                    + Bgxcnt::COLORS::Color256_1);
    mmio.dispcnt.write(Dispcnt::BG_MODE::TileMode0
                     + Dispcnt::SCR_MODE::Bg0
                     + Dispcnt::SCR_MODE::Bg1
                     + Dispcnt::SCR_MODE::Obj);

    loop {}
}
