#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, mmio::{self, Dispcnt}, input::{self, Keyinput}, video, oam::*, util};
use boot::entry;
use core::{ptr, mem};

fn oam_copy(objs: &[ObjAttr]) {
    let src = objs.as_ptr() as *const u32;
    let dst = consts::MEM_OAM_START as *mut u32;

    unsafe {
        ptr::copy_nonoverlapping(src, dst, objs.len());
    }
}

fn oam_init(objs: &mut [ObjAttr]) {
    for obj in objs.iter_mut() {
        obj.attr0.write(Attr0::OBJ_DISABLE::Disabled);
    }

    oam_copy(objs);
}

#[entry]
fn main() -> ! {
    let metr_bitmap = include_bytes!("res/metr.img.bin");
    let metr_pal = include_bytes!("res/metr.pal.bin");

    unsafe {
        ptr::copy_nonoverlapping(metr_bitmap.as_ptr() as *const u32,
                                 consts::VRAM_OBJ_START as *mut u32,
                                 metr_bitmap.len() / 4);
        ptr::copy_nonoverlapping(metr_pal.as_ptr() as *const u32,
                                 consts::PAL_OBJ_START as *mut u32,
                                 metr_pal.len() / 4);
    }

    let mmio = mmio::get_mut();
    mmio.dispcnt.write(Dispcnt::SCR_MODE::Obj + Dispcnt::OBJ_DIM::OneDim);

    let mut obj_buffer: [ObjAttr; 128] = unsafe { mem::uninitialized() };
    oam_init(&mut obj_buffer);

    let mut x = 96;
    let mut y = 32;
    let mut tile_id = 0;
    let mut pal_id;
    let mut prev_keys = 0;

    {
        let metr = &mut obj_buffer[0];
        metr.attr0.write(Attr0::OBJ_SHAPE::Square);
        metr.attr1.write(Attr1::OBJ_SIZE::Square64);
    }

    loop {
        video::vsync();
        let curr_keys = input::poll();

        x = x + 2 * input::tri_pad_horz();
        y = y + 2 * input::tri_pad_vert();

        tile_id = tile_id + input::tri_pad_lr();

        {
            let metr = &mut obj_buffer[0];

            if input::was_hit_now(curr_keys, prev_keys, Keyinput::BUTTON_A)
            {
                util::flip_flag(&metr.attr1, Attr1::FLIP_HORZ);
            }
            if input::was_hit_now(curr_keys, prev_keys, Keyinput::BUTTON_B)
            {
                util::flip_flag(&metr.attr1, Attr1::FLIP_VERT);
            }

            if input::is_held(curr_keys, prev_keys, Keyinput::SELECT)
            {
                pal_id = 1;
            }
            else
            {
                pal_id = 0;
            }

            if input::was_hit_now(curr_keys, prev_keys, Keyinput::START)
            {
                util::flip_flag(&mmio.dispcnt, Dispcnt::OBJ_DIM);
            }

            metr.attr2.write(Attr2::PALBANK.val(pal_id)
                           + Attr2::TILE_ID.val(tile_id as u16));
            metr.set_pos(x as u16, y as u16);

            unsafe { *(consts::MEM_IWRAM_START as *mut u16) = metr.attr2.get(); }

        }

        oam_copy(&obj_buffer[0..2]);

        prev_keys = curr_keys;
    }
}
