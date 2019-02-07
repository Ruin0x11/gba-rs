#![feature(asm)]
#![feature(lang_items)]
#![feature(global_asm)]

#![no_std]
#![no_main]

global_asm!(include_str!("init.s"));

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        asm!("nop");
        asm!("nop");
        asm!("nop");
        asm!("nop");
    }
    loop {}
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    //use core::fmt::Write;
    //write!(printer::Printer, "{}", info).unwrap();
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    loop {}
}

#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    loop {}
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}
