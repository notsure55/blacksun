use glium::{ Frame };
use crate::*;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Default)]
pub enum Direction {
    #[default]
    Top,
    Left,
    Right,
    Bottom,
}

pub struct Label<'a> {
    attached: Rc<Object<'a>>,
    color: color::Color,
    direction: Direction,
    text: Cell<&'a str>,
    scale: f32,
    dist_scale: f32,
}

impl<'a> Label<'a> {
    pub fn new(attached: Rc<Object<'a>>, color: color::Color, direction: Direction, text: &'a str, scale: f32, dist_scale: f32) -> Self {
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

impl<'a> Draw for Label<'a> {
    fn draw(
        &self,
        window: &Window,
        frame: &mut Frame,
    ) {
        let top_left =  {
            match self.attached.as_ref() {
                Object::FilledRect(o) => {
                    match self.direction {
                        Direction::Top => {
                            Vertex { p: [ o.rect.top_left.p[0],
                                          o.rect.top_left.p[1] - o.rect.height * self.dist_scale] }
                        },
                        Direction::Left => {
                            Vertex { p: [ o.rect.top_left.p[0] - o.rect.width,
                                          o.rect.top_left.p[1] + o.rect.height * self.dist_scale ] }
                        },
                        Direction::Right => {
                            Vertex { p: [ o.rect.top_left.p[0] + o.rect.width,
                                          o.rect.top_left.p[1] + o.rect.height * self.dist_scale ] }
                        },
                        Direction::Bottom => {
                            Vertex { p: [ o.rect.top_left.p[0],
                                          o.rect.top_left.p[1] + o.rect.height * (self.dist_scale + 1.0) ] }
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
            self.color.v.into()
        ).unwrap();
    }
}
