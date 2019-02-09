#[inline]
pub fn rgb15(r: u16, g: u16, b: u16) -> u16 {
    r | (g << 5) | (b << 10)
}
