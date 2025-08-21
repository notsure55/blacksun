use crate::{ color::Color, Window, rect::Rect, Draw, Shape };

pub struct FilledRect {
    pub shape: Shape,
    color: Color,
}

impl FilledRect {
    pub fn new(rect: Rect, color: Color) -> Self {
        Self {
            shape: Shape::Rect(rect),
            color,
        }
    }
}

impl Draw for FilledRect {
    fn get_shape(&self) -> &Shape {
        &self.shape
    }

    fn get_color(&self) -> &Color {
        &self.color
    }
}
