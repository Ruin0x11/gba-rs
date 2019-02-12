#![feature(slice_patterns)]
#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, data, mmio::{self, *}, input::{self, Keyinput}, obj::{self, *}, tile::{Tile4, Scrdata, ScrEntry},
          util, video};

use gba_boot::entry;
use core::{cmp, slice};

#[inline]
unsafe fn screen_entries<'a>(index: usize, offset: usize) -> &'a mut [ScrEntry] {
    let ptr = data::screen_block(index) as *mut ScrEntry;

    slice::from_raw_parts_mut(ptr.add(offset), 0x1000)
}

fn load_metr(mmio: &mmio::Mmio) {
    let metr_tiles = include_bytes!("res/metr.img.bin");
    let metr_pal = include_bytes!("res/metr.pal.bin");

    unsafe {
        data::load_tiles(2, metr_tiles);
        data::load_tiles(4, metr_tiles);
        data::load_obj_palette(0, metr_pal);
    }

    unsafe {
        let mut obj_mem_slice = slice::from_raw_parts_mut(consts::MEM_OAM_START as *mut ObjAttr, 128);

        obj::init_slice(&mut obj_mem_slice);

        let metr = &mut obj_mem_slice[0];
        metr.attr0.write(Attr0::OBJ_SHAPE::Square + Attr0::OBJ_MODE::SemiTrans);
        metr.attr1.write(Attr1::OBJ_SIZE::Square64);
        metr.set_pos(32, 24);
    }

    for i in 0..16 {
        unsafe {
            data::load_bg_pal_color(1, i, data::get_obj_pal_color(0, i) ^ util::rgb15(31, 31, 31));
        }
    }

    let screen = unsafe { screen_entries(30, 3 * 32 + 18) };
    for y in 0..8 {
        for x in 0..8 {
            screen[y * 32 + x].write(Scrdata::TILE_ID.val(y as u16 * 8 + x as u16) + Scrdata::PALBANK.val(1));
        }
    }

    mmio.bg1cnt.write(Bgxcnt::CHAR_BASE_BLK.val(2) + Bgxcnt::SCRN_BASE_BLK.val(30));
}

fn load_fence(mmio: &mmio::Mmio) {
    let fence: [Tile4; 1] = [Tile4 {
        data: [0x00012000, 0x00012000, 0x00022200, 0x22220222,
               0x11122211, 0x00112000, 0x00012000, 0x00012000]
    }];

    unsafe {
        data::load_tile4(2, 64, fence.as_ptr() as *const Tile4, 1);

        data::load_bg_pal_color(0, 0, util::rgb15(16, 10, 20));
        data::load_bg_pal_color(4, 1, util::rgb15( 0,  0, 31));
        data::load_bg_pal_color(4, 2, util::rgb15(16, 16, 16));
    }

    let screen = data::screen_block(29) as *mut u32;
    let map_block = 0x40404040;

    for i in 0..8 {
        for j in 0..64 {
            unsafe {
                screen.add(i * 64 + j).write(map_block);
            }
        }
    }

    mmio.bg2cnt.write(Bgxcnt::CHAR_BASE_BLK.val(2) + Bgxcnt::SCRN_BASE_BLK.val(29));
}

#[inline]
fn clamp(a: i16, min: i16, max: i16) -> i16 {
    cmp::max(min, cmp::min(a, max - 1))
}

fn test_blend(mmio: &mmio::Mmio) -> !{
    let mut mode = 0;

    mmio.bldcnt.write(Bldcnt::OBJ_TOP::SET + Bldcnt::BG1_TOP::SET
                    + Bldcnt::BG2_BOT::SET
                    + Bldcnt::MODE.val(mode as u16));

    let mut eva = 0x80;
    let mut evb = 0;
    let mut evy = 0;
    let mut prev_keys = 0;

    loop {
        video::vsync();
        let curr_keys = input::poll();

        eva += input::tri_pad_horz();
        evb += input::tri_pad_vert();
        evy += input::tri_pad_fire();

        mode += if input::was_hit_now(curr_keys, prev_keys, Keyinput::BUTTON_L::SET) {
            -1
        }
        else if input::was_hit_now(curr_keys, prev_keys, Keyinput::BUTTON_R::SET) {
            1
        }
        else {
            0
        };

        eva = clamp(eva, 0, 0x81);
        evb = clamp(evb, 0, 0x81);
        evy = clamp(evy, 0, 0x81);
        mode = clamp(mode, 0, 4);

        mmio.bldcnt.modify(Bldcnt::MODE.val(mode as u16));

        mmio.bldalpha.modify(Bldalpha::EVA.val(eva as u16 / 8) + Bldalpha::EVB.val(evb as u16 / 8));
        mmio.bldy.modify(Bldy::EVY.val(evy as u16 / 8));

        prev_keys = curr_keys;
    }
}

#[entry]
fn main() -> ! {
    let mmio = mmio::get_mut();

    load_metr(&mmio);
    load_fence(&mmio);

    mmio.dispcnt.write(Dispcnt::BG_MODE::TileMode0
                     + Dispcnt::SCR_MODE::Bg0
                     + Dispcnt::SCR_MODE::Bg1
                     + Dispcnt::SCR_MODE::Bg2
                     + Dispcnt::SCR_MODE::Obj
                     + Dispcnt::OBJ_DIM::OneDim);

    test_blend(&mmio)
}
