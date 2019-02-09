#![no_std]
#![no_main]
use gba::{mmio::{self, Dispcnt}, vram};
use boot::entry;

#[entry]
fn main() -> ! {
    let mmio = mmio::get_mut();
    mmio.dispcnt.write(Dispcnt::SCR_MODE::Bg2 + Dispcnt::BG_MODE::BitmapMode3);

    vram::plot_pixel(120, 80, vram::rgb15(31, 0, 0));
    vram::plot_pixel(136, 80, vram::rgb15(0, 31, 0));
    vram::plot_pixel(120, 96, vram::rgb15(0, 0, 31));

    loop {}
}

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _Unwind_Resume() {}
