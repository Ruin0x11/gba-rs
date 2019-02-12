#![feature(slice_patterns)]
#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, data, mmio::{self, Dispcnt, Bgxcnt, FixedBgxx}, input::{self, Keyinput}, obj::*,
          tile::{Tile4, Tile8}, util, video};
use gba_boot::entry;

#[inline]
fn init_cross() {
    let tile: [Tile4; 1] = [Tile4 {
        data: [0x00011100, 0x00100010, 0x01022201, 0x01021201,
               0x01022201, 0x00100010, 0x00011100, 0x00000000]
    }];

    unsafe {
        data::load_tile4(4, 1, tile.as_ptr() as *const Tile4, 1);

        data::load_obj_pal_color(0, 1, util::rgb15(31, 31, 31));
        data::load_obj_pal_color(1, 2, util::rgb15(31, 31, 31));
        data::load_obj_pal_color(0, 2, util::rgb15( 0,  0,  0));
        data::load_obj_pal_color(1, 1, util::rgb15( 0,  0,  0));
    }

    unsafe {
        let obj_buffer = consts::MEM_OAM_START as *mut ObjAttr;
        let cross = obj_buffer;
        let disp = obj_buffer.add(1);

        (*cross).attr2.write(Attr2::TILE_ID.val(0x0001));
        (*disp).attr2.write(Attr2::TILE_ID.val(0x1001));
    }
}

#[inline]
fn init_map(mmio: &mut mmio::Mmio) {
    let nums8_tiles = include_bytes!("res/nums8.img.bin");
    let nums8_pal = include_bytes!("res/nums8.pal.bin");

    unsafe {
        data::load_tile8(0, 1, nums8_tiles.as_ptr() as *const Tile8, nums8_tiles.len() / 4 / 16);
        data::load_bg_palette(0, nums8_pal);
    }

    mmio.bg2cnt.write(Bgxcnt::CHAR_BASE_BLK.val(0)
                    + Bgxcnt::SCRN_BASE_BLK.val(8)
                    + Bgxcnt::SIZE_AFFINE::Size512_512);

    let screen = data::screen_block(8) as *mut u32;
    let mut map_block = 0x01010101;

    for i in 0..16 {
        for j in 0..64 {
            unsafe {
                screen.add(i * 64 + j).write(map_block);
            }
        }
        map_block += 0x01010101;
    }
}

fn sblock_affine(mmio: &mut mmio::Mmio) -> ! {
    let mut prev_keys = 0;

    let mut dst = mmio::BgAffine::new();

    let mut src = mmio::BgAffineSrc {
        tex_x: 32 << 8,
        tex_y: 64 << 8,
        scr_x: 120,
        scr_y: 80,

        scale_x: 256,
        scale_y: 256,
        rot: 0
    };

    let mut scale = FixedBgxx::from_bits(0x0100);

    loop {
        video::vsync();
        let curr_keys = input::poll();

        if input::is_down(curr_keys, Keyinput::BUTTON_A::SET) {
            src.scr_x += input::tri_pad_horz() as i32;
            src.scr_y += input::tri_pad_vert() as i32;
        }
        else {
            src.tex_x -= 256 * input::tri_pad_horz() as i32;
            src.tex_y -= 256 * input::tri_pad_vert() as i32;
        }

        src.rot -= 128 * input::tri_pad_lr() as isize;

        if input::is_down(curr_keys, Keyinput::BUTTON_B::SET) {
            let diff = if input::is_down(curr_keys, Keyinput::SELECT::SET) {
                -1
            }
            else {
                1
            };
            scale += FixedBgxx::from_bits(diff);
        }

        if input::was_hit_now(curr_keys, prev_keys, Keyinput::START::SET)
        {
            if input::is_down(curr_keys, Keyinput::SELECT::SET)
            {
                src.tex_x = 0;
                src.tex_y = 0;
                src.scr_x = 0;
                src.scr_y = 0;
                src.rot = 0;
                scale = FixedBgxx::from_bits(0x0100)
            }
            else
            {
                util::flip_flag(&mmio.bg2cnt, Bgxcnt::DISP_OVERFLOW);
            }
        }

        src.scale_x = FixedBgxx::from_bits(1 << 4).wrapping_div(scale).to_bits();
        src.scale_y = FixedBgxx::from_bits(1 << 4).wrapping_div(scale).to_bits();

        dst.rotate_scale(&src);
        mmio.bg2.set(&dst);

        unsafe {
            let obj_buffer = consts::MEM_OAM_START as *mut ObjAttr;
            let cross = obj_buffer;
            let disp = obj_buffer.add(1);
            (consts::MEM_IWRAM_START as *mut i32).write(scale.to_bits());

            (*cross).set_pos(src.scr_x as u16 - 3, src.scr_y as u16 - 3);
            (*disp).set_pos((dst.x.to_bits() >> 8) as u16, (dst.y.to_bits() >> 8) as u16);
        }

        prev_keys = curr_keys;
    }
}

#[entry]
fn main() -> ! {
    let mmio = mmio::get_mut();

    init_cross();
    init_map(mmio);

    mmio.dispcnt.write(Dispcnt::BG_MODE::TileMode1
                       + Dispcnt::SCR_MODE::Bg2
                       + Dispcnt::SCR_MODE::Obj);

    sblock_affine(mmio)
}
