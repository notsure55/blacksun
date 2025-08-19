use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;
use winit::{ event_loop::EventLoop };
use glium::implement_vertex;
use glium::Surface;

pub mod rect;
pub mod filled_rect;
pub mod color;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub p: [u32; 2],
}

implement_vertex!(Vertex, p);

pub trait Draw {
    fn draw(
        &self,
        menu: &Window,
        frame: &mut glium::Frame
    );
}

pub enum Object {
    FilledRect(filled_rect::FilledRect),
}

impl Draw for Object {
    fn draw(
        &self,
        menu: &Window,
        frame: &mut glium::Frame
    ) {
        match self {
            Object::FilledRect(f) => {
                f.draw(menu, frame)
            }
        }
    }

}

pub struct Window {
    pub window: winit::window::Window,
    pub display: Display<WindowSurface>,
    pub size: (u32, u32),
    pub objects: Vec<Object>,
}

impl Window {
    pub fn new() -> (Self, EventLoop<()>) {
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .expect("event loop failed to build!, probaly trying to build not within main thread");

        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .build(&event_loop);

        let size = (window.inner_size().width, window.inner_size().height);

        (Self {
            window,
            display,
            size,
            objects: Vec::new()
        }, event_loop)
    }

    pub fn draw(&mut self) {
        let mut frame = self.display.draw();

        frame.clear_color(1.0, 0.0, 0.0, 1.0);

        let mut objects = std::mem::take(&mut self.objects);

        for object in objects.iter() {
            object.draw(self, &mut frame);
        }

        self.objects = std::mem::take(&mut objects);

        frame.finish().unwrap();
    }

    pub fn add_to_draw_list(&mut self, object: Object) {
        self.objects.push(object);
    }
}
