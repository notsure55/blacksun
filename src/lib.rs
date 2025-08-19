use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;
use winit::{ event_loop::EventLoop };
use glium::implement_vertex;
use glium::Surface;
use std::rc::Rc;

pub mod rect;
pub mod filled_rect;
pub mod color;
pub mod rusttype;
pub mod label;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub p: [f32; 2],
}

implement_vertex!(Vertex, p);

pub trait Draw {
    fn draw(
        &self,
        menu: &Window,
        frame: &mut glium::Frame
    );
}

pub enum Object<'a> {
    FilledRect(filled_rect::FilledRect),
    Label(label::Label<'a>),
}

impl<'a> Draw for Object<'a> {
    fn draw(
        &self,
        menu: &Window,
        frame: &mut glium::Frame
    ) {
        match self {
            Object::FilledRect(o) => {
                o.draw(menu, frame)
            },
            Object::Label(o) => {
                o.draw(menu, frame)
            }
        }
    }

}

pub struct Window<'a> {
    pub window: winit::window::Window,
    pub display: Display<WindowSurface>,
    pub size: (u32, u32),
    pub font: rusttype::FontTexture,
    pub text_system: rusttype::TextSystem,
    pub objects: Vec<Rc<Object<'a>>>,
}

impl<'a> Window<'a> {
    pub fn new() -> (Self, EventLoop<()>) {
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .expect("event loop failed to build!, probaly trying to build not within main thread");

        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .build(&event_loop);

        let size = (window.inner_size().width, window.inner_size().height);

        let text_system = rusttype::TextSystem::new(&display);

        let font = rusttype::FontTexture::new(
            &display,
            &include_bytes!("../fonts/arialbd.ttf")[..], 70,
            rusttype::FontTexture::ascii_character_list()
        ).unwrap();

        (Self {
            window,
            display,
            size,
            font,
            text_system,
            objects: Vec::new()
        } , event_loop)
    }

    pub fn draw(&mut self) {
        let mut frame = self.display.draw();

        frame.clear_color(1.0, 1.0, 1.0, 1.0);

        let mut objects = std::mem::take(&mut self.objects);

        for object in objects.iter() {
            object.as_ref().draw(self, &mut frame);
        }

        self.objects = std::mem::take(&mut objects);

        frame.finish().unwrap();
    }

    pub fn add_to_draw_list(&mut self, object: Rc<Object<'a>>) {
        self.objects.push(object);
    }
}
