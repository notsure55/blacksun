use crate::{ *, shape::Shape, traits::* };
use std::cell::Cell;

pub struct Button {
    shape: Shape,
    toggle: Cell<bool>,
}

impl Button {
    pub fn new(shape: Shape, toggle: Cell<bool> ) -> Self {
        Self {
            shape,
            toggle,
        }
    }
}

impl Shapeable for Button {
    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}

impl Colorable for Button {
    fn get_color(&self) -> &color::Color {
        match self.get_shape() {
            Shape::Rect(r) => { r.get_color() }
            Shape::Circle(c) => { c.get_color() }
        }
    }
}

impl Drawable for Button {
    fn draw(&self, window: &Window, frame: &mut glium::Frame) {
        self.shape.draw(window, frame);
    }

    fn clicked(&self, window: &Window) {
        if self.in_bounds(window) && window.clicked {
            let toggle = self.toggle.get();
            self.toggle.set(!toggle);
            println!("{}", self.toggle.get());
        }
    }
}
