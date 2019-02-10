#![feature(lang_items)]
#![no_std]

pub mod consts;
pub mod input;
pub mod mmio;
pub mod oam;
pub mod tile;
pub mod util;
pub mod video;

#[lang = "eh_personality"]
fn eh_personality() {}

#[no_mangle]
extern "C" fn _Unwind_Resume() {}
