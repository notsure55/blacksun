use crate::{ color::Color, shape::Shape, traits::*, *, text::Text} ;

pub struct Grid {
    shape: Shape,
    color: Color,
    objects: Vec<GridText>,
}

impl Grid {
    pub fn new(rect: rect::Rect, color: Color) -> Self {
        Self {
            shape: Shape::Rect(rect),
            color,
            objects: Vec::new(),
        }
    }
}

impl Colorable for Grid {
    fn get_color(&self) -> &color::Color {
        &self.color
    }
}

impl Shapeable for Grid {
    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}

impl Drawable for Grid {
    fn draw(&self, window: &Window, frame: &mut glium::Frame) {
        self.shape.draw(window, frame);
        let rows = self.objects.len();

        if let Shape::Rect(rect) = &self.shape {
            let top_left = rect.get_top_left();
            let size = rect.get_size();
            let object_height = size.1 / rows as f32;
            let mut rect = rect::Rect::new(
                *top_left, size.0 as u32, object_height as u32,
                *self.get_color(),
                false,
                5.0
            );

            for (index, object) in self.objects.iter().enumerate() {
                rect.top_left = Vertex { p: [rect.top_left.p[0], rect.top_left.p[1] + object_height * index as f32] };
                object.draw(&rect, window, frame);
                object.clicked(&rect, window);
                object.selected(window);
            }
        }
    }
}

use std::cell::{ Cell, RefCell};

pub struct GridText {
    text: RefCell<String>,
    color: color::Color,
    scale: f32,
    selected: Cell<bool>
}

impl GridText {
    pub fn new(text: &str, color: color::Color, scale: f32) -> Self {
        Self {
            text: RefCell::new(String::from(text)),
            color,
            scale,
            selected: Cell::new(false),
        }
    }

    fn draw(
        &self,
        rect: &rect::Rect,
        window: &Window,
        frame: &mut glium::Frame,
    ) {
        let top_left = {
            rect.draw(window, frame);
            Vertex { p: [rect.top_left.p[0], rect.top_left.p[1] + rect.get_size().1] }
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

    fn clicked(&self, rect: &rect::Rect, window: &Window) {
        // if in bounds of box and clicked, we are selecting object
        if rect.in_bounds(window) && window.clicked {
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
    fn get_color(&self) -> &color::Color {
        &self.color
    }
}
