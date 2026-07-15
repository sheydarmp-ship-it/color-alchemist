pub struct ColorRGB {
    pub r: u8,
    pub y: u8,
    pub b: u8,
}
impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
}
}