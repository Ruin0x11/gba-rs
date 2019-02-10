#![feature(global_asm)]
#![no_std]

extern crate gba_macros as macros;

pub use macros::entry;

global_asm!(include_str!("init.s"));


#[inline]
pub fn heap_start() -> *mut u32 {
    extern "C" {
        static mut __eheap_start: u32;
    }

    unsafe { &mut __eheap_start }
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {}
