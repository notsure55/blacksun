use crate::*;

pub struct Circle {
    point: Vertex,
    radius: f32,
    color: color::Color,
    filled: bool,
}

impl Circle {
    pub fn is_filled(&self) -> &bool{
        &self.filled
    }
    pub fn get_color(&self) -> &color::Color {
        &self.color
    }
}
