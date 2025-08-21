use crate::*;
use crate::shape::Shape;

pub trait Colorable {
    fn get_color(&self) -> &color::Color;
}

pub trait Shapeable {
    fn get_shape(&self) -> &Shape;
}

pub trait Drawable: Shapeable {
    fn draw(&self, window: &Window, frame: &mut glium::Frame);

    fn in_bounds(&self, window: &Window) -> bool {
        match self.get_shape() {
            Shape::Rect(r) => {
                r.in_bounds(window)
            },
            _ => false,
        }
    }

    fn clicked(&self, _window: &Window) {}
    fn selected(&self, window: &Window) {}
}
