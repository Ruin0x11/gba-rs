#![feature(global_asm)]
#![no_std]

extern crate gba_macros as macros;

pub use macros::entry;

global_asm!(include_str!("init.s"));


#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}
