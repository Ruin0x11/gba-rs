#![feature(lang_items)]
#![no_std]

pub mod consts;
pub mod mmio;
pub mod util;

#[lang = "eh_personality"]
fn eh_personality() {}

#[no_mangle]
extern "C" fn _Unwind_Resume() {}
