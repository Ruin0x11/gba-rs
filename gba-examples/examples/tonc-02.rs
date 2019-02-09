#![no_std]
#![no_main]

extern crate panic_halt;

use gba::{consts, mmio::{self, Dispcnt}, util};
use boot::entry;

fn bmp16_line(left: i32, top: i32, right: i32, bottom: i32, color: u16, addr: usize, mut pitch: u32) {
    let mut canvas = (addr + left as usize * 2 + top as usize * pitch as usize) as *mut u16;
    pitch = pitch / 2;

    let plot = |off| {
        unsafe {
            *canvas.offset((off) as isize) = color;
        }
    };

    let (xstep, dx) = if left > right {
        (-1, left - right)
    }
    else {
        (1, right - left)
    };

    let (ystep, dy) = if top > bottom {
        (-(pitch as isize), top - bottom)
    }
    else {
        (pitch as isize, bottom - top)
    };


    if dy == 0 {
        for x in 0..dx+1 {
            plot(x * xstep as i32);
        }
    }
    else if dx == 0 {
        for y in 0..dy+1 {
            plot(y * ystep as i32);
        }
    }
    else if dx >= dy {
        let mut delta = 2 * dy - dx;
        let mut y = top;

        for _ in 0..dx+1 {
            unsafe { *canvas = color; }
            if delta > 0 {
                y = y + 1;
                delta = delta - 2 * dx;
                unsafe { canvas = canvas.offset(ystep); }
            }
            delta = delta + 2 * dy;
            unsafe { canvas = canvas.offset(xstep); }
        }
    }
    else {
        let mut delta = 2 * dx - dy;
        let mut x = left;

        for _ in 0..dy+1 {
            unsafe { *canvas = color; }
            if delta > 0 {
                x = x + 1;
                delta = delta - 2 * dy;
                unsafe { canvas = canvas.offset(xstep); }
            }
            delta = delta + 2 * dx;
            unsafe { canvas = canvas.offset(ystep); }
        }
    }
}

fn bmp16_rect(left: i32, top: i32, right: i32, bottom: i32, color: u16, addr: usize, mut pitch: u32) {
    let canvas = (addr + left as usize * 2 + top as usize * pitch as usize) as *mut u16;
    pitch = pitch / 2;

    for j in 0..(bottom - top) {
        for i in 0..(right - left) {
            unsafe {
                *canvas.offset((j * pitch as i32 + i) as isize) = color;
            }
        }
    }
}

fn bmp16_frame(left: i32, top: i32, mut right: i32, mut bottom: i32, color: u16, canvas: usize, pitch: u32) {
    right = right - 1;
    bottom = bottom - 1;

    bmp16_line(left,  top,    right, top,    color, canvas, pitch);
    bmp16_line(left,  bottom, right, bottom, color, canvas, pitch);
    bmp16_line(left,  top,    left,  bottom, color, canvas, pitch);
    bmp16_line(right, top,    right, bottom, color, canvas, pitch);
}

#[inline]
fn mode3_line(left: i32, top: i32, right: i32, bottom: i32, color: u16) {
    bmp16_line(left, top, right, bottom, color,
                consts::MEM_VRAM_START,
                consts::SCREEN_WIDTH * 2);
}

#[inline]
fn mode3_rect(left: i32, top: i32, right: i32, bottom: i32, color: u16) {
    bmp16_rect(left, top, right, bottom, color,
                consts::MEM_VRAM_START,
                consts::SCREEN_WIDTH * 2);
}

#[inline]
fn mode3_frame(left: i32, top: i32, right: i32, bottom: i32, color: u16) {
    bmp16_frame(left, top, right, bottom, color,
                consts::MEM_VRAM_START,
                consts::SCREEN_WIDTH * 2);
}

fn mode3_clear(color: u16) {
    let vram = consts::MEM_VRAM_START as *mut u32;
    let fill: u32 = ((color as u32) << 16) | color as u32;

    for i in 0..(consts::SCREEN_SIZE as isize / 2) {
        unsafe {
            *vram.offset(i) = fill;
        }
    }
}

#[entry]
fn main() -> ! {
    let mmio = mmio::get_mut();
    mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg2 + Dispcnt::BG_MODE::BitmapMode3);

    mode3_clear(util::rgb15(12, 12, 14));

    mode3_rect( 12,  8, 108,  72, util::rgb15(31, 0,  0));
    mode3_rect(108, 72, 132,  88, util::rgb15(0,  31, 0));
    mode3_rect(132, 88, 228, 152, util::rgb15(0,  0, 31));

    mode3_frame(132,  8, 228,  72, util::rgb15(0, 31, 31));
    mode3_frame(109, 73, 131,  87, util::rgb15(0,  0,  0));
    mode3_frame( 12, 88, 108, 152, util::rgb15(31, 31, 0));

    for i in 0..9 {
        let j = 3 * i as u16 + 7;

        mode3_line(132 + 11 * i,  9, 226, 12 + 7 * i, util::rgb15(j, 0, j));
        mode3_line(226 - 11 * i, 70, 133, 69 - 7 * i, util::rgb15(j, 0, j));
    }

    for i in 0..9 {
        let j = 3 * i as u16 + 7;

        mode3_line(15 + 11 * i, 88, 104 - 11 * i, 150, util::rgb15(0, j, j));
    }

    loop {}
}
