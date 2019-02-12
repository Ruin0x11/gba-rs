use core::ptr;
use crate::{consts, tile::{Tile4, Tile8}};

const PALETTE_SIZE: usize = 8;

#[inline]
pub unsafe fn load_bg_bitmap(offset: usize, bytes: &[u8]) {
    ptr::copy_nonoverlapping(bytes.as_ptr() as *const u16,
                             (consts::VRAM_BG_START as *mut u16).add(offset * 2),
                             bytes.len() / 2);
}

#[inline]
pub unsafe fn load_bg_palette(index: usize, bytes: &[u8]) {
    ptr::copy_nonoverlapping(bytes.as_ptr() as *const u16,
                             (consts::PAL_BG_START as *mut u16).add(index * PALETTE_SIZE * 2),
                             bytes.len() / 2);
}

#[inline]
pub unsafe fn load_obj_bitmap(offset: usize, bytes: &[u8]) {
    ptr::copy_nonoverlapping(bytes.as_ptr() as *const u16,
                             (consts::VRAM_OBJ_START as *mut u16).add(offset * 2),
                             bytes.len() / 2);
}

#[inline]
pub unsafe fn load_obj_palette(index: usize, bytes: &[u8]) {
    ptr::copy_nonoverlapping(bytes.as_ptr() as *const u16,
                             (consts::PAL_OBJ_START as *mut u16).add(index * PALETTE_SIZE * 2),
                             bytes.len() / 2);
}

#[inline]
pub unsafe fn load_tiles(chara_block_id: usize, bytes: &[u8]) {
    ptr::copy_nonoverlapping(bytes.as_ptr() as *const u16,
                             chara_block(chara_block_id),
                             bytes.len() / 2);
}

#[inline]
pub unsafe fn load_bg_map(screen_block_id: usize, bytes: &[u8]) {
    ptr::copy_nonoverlapping(bytes.as_ptr() as *const u16,
                             screen_block(screen_block_id),
                             bytes.len() / 2);
}

#[inline]
pub unsafe fn load_tile4(chara_block_id: usize, tile_id: usize, tiles: *const Tile4, count: usize) {
    let tile_ram = chara_block(chara_block_id) as *mut Tile4;

    ptr::copy_nonoverlapping(tiles, tile_ram.add(tile_id), count);
}

#[inline]
pub unsafe fn load_tile8(chara_block_id: usize, tile_id: usize, tiles: *const Tile8, count: usize) {
    let tile_ram = chara_block(chara_block_id) as *mut Tile8;

    ptr::copy_nonoverlapping(tiles, tile_ram.add(tile_id), count);
}

#[inline]
pub unsafe fn load_bg_pal_color(pal_index: usize, color_index: usize, color: u16) {
    let pal_ram = consts::PAL_BG_START as *mut u16;
    *pal_ram.add(pal_index * 16 + color_index) = color;
}

#[inline]
pub unsafe fn load_obj_pal_color(pal_index: usize, color_index: usize, color: u16) {
    let pal_ram = consts::PAL_OBJ_START as *mut u16;
    *pal_ram.add(pal_index * 16 + color_index) = color;
}

#[inline]
pub unsafe fn get_bg_pal_color(pal_index: usize, color_index: usize) -> u16 {
    let pal_ram = consts::PAL_BG_START as *const u16;
    *pal_ram.add(pal_index * 16 + color_index)
}

#[inline]
pub unsafe fn get_obj_pal_color(pal_index: usize, color_index: usize) -> u16 {
    let pal_ram = consts::PAL_OBJ_START as *const u16;
    *pal_ram.add(pal_index * 16 + color_index)
}

#[inline]
pub fn chara_block(index: usize) -> *mut u16 {
    unsafe {
        (consts::VRAM_BG_START as *mut u16).add(1024 * 8 * (index % consts::VRAM_CHARA_BLOCK_MAX))
    }
}

#[inline]
pub fn screen_block(index: usize) -> *mut u16 {
    unsafe {
        (consts::VRAM_BG_START as *mut u16).add(1024 * (index % consts::VRAM_SCREEN_BLOCK_MAX))
    }
}
