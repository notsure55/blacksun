use glium::{ Frame };
use crate::{ *, shape::*, traits::*};
use std::cell::{ Cell, RefCell};

pub struct Text {
    shape: Shape,
    text: RefCell<String>,
    color: color::Color,
    scale: f32,
    selected: Cell<bool>
}

impl Text {
    pub fn new(rect: rect::Rect, text: &str, color: color::Color, scale: f32) -> Self {
        Self {
            shape: Shape::Rect(rect),
            text: RefCell::new(String::from(text)),
            color,
            scale,
            selected: Cell::new(false),
        }
    }
}

impl Shapeable for Text {
    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}

impl Colorable for Text {
    fn get_color(&self) -> &color::Color {
        &self.color
    }
}

impl Drawable for Text {
    fn draw(
        &self,
        window: &Window,
        frame: &mut Frame,
    ) {
        let top_left = match self.get_shape() {
            Shape::Rect(r) => {
                r.draw(window, frame);
                Vertex { p: [r.top_left.p[0], r.top_left.p[1] + r.get_size().1] }
            },
            _ => return,
        };

        let text = rusttype::TextDisplay::new(&window.text_system, &window.font, &self.text.borrow());
        //let text_width = text.get_width();
        let sx = self.scale / (window.size.0 as f32 / 2.0);
        let sy = self.scale / (window.size.1 as f32 / 2.0);

        let x_ndc = (top_left.p[0] / window.size.0 as f32) * 2.0 - 1.0;
        let y_ndc = -((top_left.p[1] / window.size.1 as f32) * 2.0 - 1.0);

        let matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
            sx,  0.0, 0.0, 0.0,
            0.0, sy,  0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            x_ndc, y_ndc, 0.0, 1.0,
        ).into();

        rusttype::draw(
            &text,
            &window.text_system,
            frame,
            matrix,
            self.get_color().v.into()
        ).unwrap();
    }

    fn clicked(&self, window: &Window) {
        // if in bounds of box and clicked, we are selecting object
        if self.in_bounds(window) && window.clicked {
            self.selected.set(!self.selected.get());
        // if not in bounds we are clicking somewhere else so deselect this object
        } else if window.clicked {
            self.selected.set(false);
        }
    }

    fn selected(&self, window: &Window) {
        if self.selected.get() == true {
            let keyboard = window.get_keyboard();
            if keyboard.backspace {
                let text_length = self.text.borrow().len();
                if text_length >= 1 {
                    self.text.borrow_mut().truncate(text_length - 1);
                }
            } else if !keyboard.input.is_empty() {
                self.text.borrow_mut().push_str(&keyboard.input);
            }
        }
    }
}
