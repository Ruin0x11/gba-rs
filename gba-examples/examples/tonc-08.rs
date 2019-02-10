#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, mmio::{self, Dispcnt, Bgxcnt}, input, tile::{Tile8, ScrEntry, Scrdata}, util, video};
use boot::entry;
use core::slice;

const CHARA_BLOCK: usize = 0;
const SCREEN_BLOCK: usize = 28;

const CROSS_TX: u32 = 15;
const CROSS_TY: u32 = 10;

const BG_MAP_SIZE: usize = 32 * 32;

#[inline]
fn chara_block(index: usize) -> *mut u16 {
    (consts::VRAM_BG_START + 1024 * index * 16) as *mut u16
}

#[inline]
fn screen_block_index(tx: u32, ty: u32, pitch: u32) -> u32 {
    let sbb = (tx >> 5) + (ty >> 5) * (pitch >> 5);
    sbb * 1024 + ((tx & 31) + (ty & 31) * 32)
}

#[inline]
fn screen_block<'a>(offset: usize) -> &'a mut [ScrEntry] {
    let ptr = (consts::VRAM_BG_START + offset * 1024 * 2) as *mut ScrEntry;

    unsafe {
        slice::from_raw_parts_mut(ptr, 0x1000)
    }
}

#[inline]
fn write_tile(cblock: usize, tile_id: isize, tile: &[u32]) {
    let tile_ram = chara_block(cblock) as *mut Tile8;

    unsafe {
        tile_ram.offset(tile_id).copy_from(tile.as_ptr() as *const Tile8, 1);
    }
}

#[inline]
fn write_palette(palbank: usize, index: usize, color: u16) {
    unsafe {
        *((consts::MEM_PAL_START + palbank * 16 * 2 + index * 2) as *mut u16) = color;
    }
}


#[inline]
fn init_map(mmio: &mut mmio::Mmio) {
    mmio.bg0cnt.write(Bgxcnt::CHAR_BASE_BLK.val(CHARA_BLOCK as u16)
                    + Bgxcnt::SCRN_BASE_BLK.val(SCREEN_BLOCK as u16)
                    + Bgxcnt::SIZE_TEXT::Size512_512);
    mmio.bg0hofs.set(0);
    mmio.bg0vofs.set(0);

    let tiles = [
        [0x11111111, 0x01111111, 0x01111111, 0x01111111, 0x01111111, 0x01111111, 0x01111111, 0x00000001],
        [0x00000000, 0x00100100, 0x01100110, 0x00011000, 0x00011000, 0x01100110, 0x00100100, 0x00000000],
    ];

    write_tile(CHARA_BLOCK, 0, &tiles[0]);
    write_tile(CHARA_BLOCK, 1, &tiles[1]);

    write_palette(0, 1, util::rgb15(31,  0,  0));
    write_palette(1, 1, util::rgb15( 0, 31,  0));
    write_palette(2, 1, util::rgb15( 0,  0, 31));
    write_palette(3, 1, util::rgb15(16, 16, 16));

    let screen = screen_block(SCREEN_BLOCK);
    let mut k = 0;

    for i in 0..4 {
        for j in 0..BG_MAP_SIZE {
            screen[k].write(Scrdata::PALBANK.val(i as u16));
            k += 1;
        }
    }
}

#[entry]
fn main() -> ! {
    let mmio = mmio::get_mut();

    init_map(mmio);
    mmio.dispcnt.write(Dispcnt::BG_MODE::TileMode0
                     + Dispcnt::SCR_MODE::Bg0
                     + Dispcnt::SCR_MODE::Obj);

    let mut x = 0;
    let mut y = 0;
    let mut tx;
    let mut ty;
    let mut sblock_curr;
    let mut sblock_prev = CROSS_TY * 32 + CROSS_TX;
    let screen = screen_block(SCREEN_BLOCK);

    screen[sblock_prev as usize].modify(Scrdata::TILE_ID_TEXT.val(1));

    loop {
        video::vsync();
        input::poll();

        x += input::tri_pad_horz();
        y += input::tri_pad_vert();

        tx = ((x as u32 >> 3) + CROSS_TX) & 0x3F;
        ty = ((y as u32 >> 3) + CROSS_TY) & 0x3F;

        sblock_curr = screen_block_index(tx, ty, 64);
        if sblock_curr != sblock_prev {
            screen[sblock_prev as usize].modify(Scrdata::TILE_ID_TEXT.val(0));
            screen[sblock_curr as usize].modify(Scrdata::TILE_ID_TEXT.val(1));
            sblock_prev = sblock_curr;
        }

        mmio.bg0hofs.set(x as u16);
        mmio.bg0vofs.set(y as u16);
    }
}
