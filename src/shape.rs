use crate::*;
use crate::traits::*;

#[derive(Copy, Clone)]
pub enum Shape {
    Rect(rect::Rect),
    Circle(circle::Circle),
}

impl Shapeable for Shape {
    fn get_shape(&self) -> &Shape {
        self
    }
}

impl Drawable for Shape {
    fn draw(
        &self,
        window: &Window,
        frame: &mut glium::Frame
    ) {
        match self {
            Shape::Rect(r) => {
                r.draw(window, frame);
            },
            _ => return,
        }
    }
}

impl Colorable for Shape {
    fn get_color(&self) -> &color::Color {
        match self {
            Shape::Rect(r) => r.get_color(),
            Shape::Circle(c) => c.get_color(),
        }
    }
}
