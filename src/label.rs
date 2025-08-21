use glium::{ Frame };
use crate::{ *, traits::*, shape::*};
use std::cell::{ Cell, RefCell };
use std::rc::Rc;

#[derive(Default)]
pub enum Direction {
    #[default]
    Top,
    Left,
    Right,
    Bottom,
    Inside,
}

pub struct Label<'a> {
    attached: Rc<dyn Drawable>,
    color: color::Color,
    direction: Direction,
    text: Cell<&'a str>,
    scale: f32,
    dist_scale: f32,
}

impl<'a> Label<'a> {
    pub fn new(attached: Rc<dyn Drawable>, color: color::Color, direction: Direction, text: &'a str, scale: f32, dist_scale: f32) -> Self {
        Self {
            attached,
            color,
            direction,
            text: Cell::new(text),
            scale,
            dist_scale
        }
    }
}

impl<'a> Shapeable for Label<'a>  {
    fn get_shape(&self) -> &Shape {
        self.attached.get_shape()
    }
}

impl<'a> Colorable for Label<'a>  {
    fn get_color(&self) -> &color::Color {
        &self.color
    }
}

impl<'a> Drawable for Label<'a> {
    fn draw(
        &self,
        window: &Window,
        frame: &mut Frame,
    ) {
        let top_left =  {
            match self.attached.get_shape() {
                 Shape::Rect(r) => {
                    match self.direction {
                        Direction::Top => {
                            Vertex { p: [ r.top_left.p[0],
                                          r.top_left.p[1] - r.height * self.dist_scale] }
                        },
                        Direction::Left => {
                            Vertex { p: [ r.top_left.p[0] - r.width,
                                          r.top_left.p[1] + r.height * self.dist_scale ] }
                        },
                        Direction::Right => {
                            Vertex { p: [ r.top_left.p[0] + r.width,
                                          r.top_left.p[1] + r.height * self.dist_scale ] }
                        },
                        Direction::Bottom => {
                            Vertex { p: [ r.top_left.p[0],
                                          r.top_left.p[1] + r.height * (self.dist_scale + 1.0) ] }
                        },
                        Direction::Inside => {
                            Vertex { p: [ r.top_left.p[0] + r.width * (self.dist_scale),
                                          r.top_left.p[1] + r.height / 2.0  ] }
                        },
                    }
                },
                _ => { return },
            }
        };

        let text = rusttype::TextDisplay::new(&window.text_system, &window.font, self.text.get());
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
}
