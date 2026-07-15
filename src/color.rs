use rand::Rng;
pub struct ColorRGB {
    pub r: u8,
    pub y: u8,
    pub b: u8,
}
impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
}
pub fn random() -> Self {
    let mut rng = rand::thread_rng();

    Self {
        r: rng.gen_range(0..=255),
        g: rng.gen_range(0..=255),
        b: rng.gen_range(0..=255),
    }
}
pub fn distance(&self, other: &ColorRGB) -> f32 {
    let dr = self.r as f32 - other.r as f32;
    let dg = self.g as f32 - other.g as f32;
    let db = self.b as f32 - other.b as f32;

    (dr * dr + dg * dg + db * db).sqrt()
}
 pub fn similarity(&self, other: &ColorRGB) -> f32 {
        let distance = self.distance(other);

        let max_distance =
            (255.0_f32 * 255.0 + 255.0 * 255.0 + 255.0 * 255.0).sqrt();

        ((1.0 - distance / max_distance) * 100.0).clamp(0.0, 100.0)
    }

    pub fn to_macroquad(&self) -> Color {
        Color::from_rgba(self.r, self.y, self.b, 255)
    }
}