pub struct ColorRYB {
    pub r: u8,
    pub y: u8,
    pub b: u8,
}
impl ColorRYB {
    pub fn new(r: u8, y: u8, b: u8) -> Self {
    Self { r, y, b }
}
}