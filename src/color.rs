#[derive(Default, Copy, Clone)]
pub struct Color {
    pub v: [f32; 4]
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            v: [ f32::from(r)/ 255.0, f32::from(g) / 255.0, f32::from(b) / 255.0, f32::from(a) / 255.0],
        }
    }
}
