#![feature(asm)]
#![feature(lang_items)]
#![feature(global_asm)]

#![no_std]
#![no_main]

global_asm!(include_str!("init.s"));

use core::panic::PanicInfo;

const SCREEN_WIDTH: u8 = 240;

const MAP_IO_REG: usize = 0x04000000;
const MAP_VRAM:   usize = 0x06000000;

unsafe fn vmode_3_bg_2()
{
    let ptr = MAP_IO_REG as *mut u32;

    *ptr = 0x0403;
}

unsafe fn set_pixel(x: u8, y: u8, color: u16)
{
    let ptr = MAP_VRAM as *mut u16;

    *ptr.offset((x + y * SCREEN_WIDTH) as isize) = color;
}

unsafe fn our_main()
{
    vmode_3_bg_2();

    set_pixel(120, 80, 0x001F);
    set_pixel(136, 80, 0x03E0);
    set_pixel(120, 96, 0x7C00);
}

#[no_mangle]
#[link_section = ".iwram"]
unsafe fn in_your_iwram()
{
    set_pixel(120, 70, 0x001F);
    set_pixel(136, 70, 0x03E0);
    set_pixel(120, 86, 0x7C00);
}

#[no_mangle]
#[link_section = ".ewram"]
unsafe fn in_your_ewram()
{
    set_pixel(120, 60, 0x001F);
    set_pixel(136, 60, 0x03E0);
    set_pixel(120, 76, 0x7C00);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        our_main();
        in_your_iwram();
        in_your_ewram();
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
