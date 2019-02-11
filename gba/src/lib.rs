#![feature(asm)]
#![feature(lang_items)]
#![no_std]

pub mod bios;
pub mod consts;
pub mod data;
pub mod font;
pub mod input;
pub mod lut;
pub mod mmio;
pub mod obj;
pub mod obj_aff;
pub mod tile;
pub mod util;
pub mod video;

#[cfg(not(test))]
#[lang = "eh_personality"]
fn eh_personality() {}

#[cfg(not(test))]
#[no_mangle]
extern "C" fn _Unwind_Resume() {}
