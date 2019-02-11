use crate::{data, tile::{ScrEntry, Scrdata}};

#[inline]
unsafe fn set_font_map(screen_block_index: usize,
                       x: u32,
                       y: u32,
                       ch: u16,
                       palette_index: u32) {
    let screen = data::screen_block(screen_block_index) as *mut ScrEntry;
    let entry = screen.add(y as usize * 32 + x as usize).as_ref().unwrap();
    entry.write(Scrdata::TILE_ID.val((ch - 32) as u16) +
                Scrdata::PALBANK.val(palette_index as u16));
}

pub unsafe fn draw_string(string: &[u8], mut x: u32, y: u32, palette_index: u32, screen_block_index: usize) {
    for ch in string.iter() {
        if x >= 31 {
            break;
        }

        set_font_map(screen_block_index, x, y, *ch as u16, palette_index);
        x += 1;
    }
}
