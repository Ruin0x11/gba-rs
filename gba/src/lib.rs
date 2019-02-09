#![feature(lang_items)]
#![no_std]

pub mod consts;
pub mod mmio;
pub mod vram;

#[lang = "eh_personality"]
fn eh_personality() {}
