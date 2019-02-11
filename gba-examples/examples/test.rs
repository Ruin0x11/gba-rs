#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate panic_halt;

use gba::bios;
use boot::entry;

static DOOD: [i32; 4] = [0xD0, 0x0D, 0xBE, 0xEF];

#[entry]
fn main() -> ! {
    let a = unsafe { *(0x08000000 as *const usize) };
    let b = 2;
    let mut c = 0;

    for i in 0..3 {

        c += gba::lut::DOOD[i + a] as i32;
    }

    let dood = (gba::lut::DOOD.as_ptr() as usize) as i32;

    loop {
        bios::div(c, b);
        bios::div(c, dood);
    }
}
