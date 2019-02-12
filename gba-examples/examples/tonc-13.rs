#![feature(slice_patterns)]
#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, data, mmio::{self, Dispcnt, Bgxcnt, Mosaic}, input::{self, Keyinput},
          obj::{self, *}, tile::{Scrdata, ScrEntry}, util, video};
use gba_boot::entry;
use core::{cmp, slice};

#[inline]
unsafe fn screen_entries<'a>(index: usize, offset: usize) -> &'a mut [ScrEntry] {
    let ptr = data::screen_block(index) as *mut ScrEntry;

    slice::from_raw_parts_mut(ptr.add(offset), 0x1000)
}

fn load_metr() {
    let metr_tiles = include_bytes!("res/metr.img.bin");
    let metr_pal = include_bytes!("res/metr.pal.bin");

    unsafe {
        data::load_tiles(1, metr_tiles);
        data::load_tiles(4, metr_tiles);
        data::load_obj_palette(0, metr_pal);
    }

    unsafe {
        let mut obj_mem_slice = slice::from_raw_parts_mut(consts::MEM_OAM_START as *mut ObjAttr, 128);

        obj::init_slice(&mut obj_mem_slice);

        let metr = &mut obj_mem_slice[0];
        metr.attr0.write(Attr0::OBJ_SHAPE::Square + Attr0::OBJ_MOSAIC::SET);
        metr.attr1.write(Attr1::OBJ_SIZE::Square64);
        metr.set_pos(32, 24);
    }

    for i in 1..16 {
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
}

struct Point {
    pub x: i16,
    pub y: i16,
}

#[inline]
fn clamp(a: i16, min: i16, max: i16) -> i16 {
    cmp::max(min, cmp::min(a, max - 1))
}

fn test_mosaic(mmio: &mmio::Mmio) -> !{
    let mut obj_point = Point { x: 0, y: 0 };
    let mut bg_point = Point { x: 0, y: 0 };

    loop {
        video::vsync();
        let curr_keys = input::poll();

        let point = if input::is_down(curr_keys, Keyinput::BUTTON_A::SET) {
            &mut obj_point
        }
        else {
            &mut bg_point
        };

        point.x += input::tri_pad_horz();
        point.y -= input::tri_pad_vert();

        point.x = clamp(point.x, 0, 0x80);
        point.y = clamp(point.y, 0, 0x80);

        mmio.mosaic.write(Mosaic::BG_SIZE_H.val(obj_point.x as u16 >> 3)
                        + Mosaic::BG_SIZE_V.val(obj_point.y as u16 >> 3)
                        + Mosaic::OBJ_SIZE_H.val(bg_point.x as u16 >> 3)
                        + Mosaic::OBJ_SIZE_V.val(bg_point.y as u16 >> 3));
    }
}

#[entry]
fn main() -> ! {
    load_metr();

    let mmio = mmio::get_mut();
    mmio.bg1cnt.write(Bgxcnt::CHAR_BASE_BLK.val(1)
                    + Bgxcnt::SCRN_BASE_BLK.val(30)
                    + Bgxcnt::MOSAIC::SET);
    mmio.dispcnt.write(Dispcnt::BG_MODE::TileMode0
                     + Dispcnt::SCR_MODE::Bg0
                     + Dispcnt::SCR_MODE::Bg1
                     + Dispcnt::SCR_MODE::Obj
                     + Dispcnt::OBJ_DIM::OneDim);

    test_mosaic(&mmio)
}
