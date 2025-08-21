use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;
use winit::{ event_loop::EventLoop };
use glium::implement_vertex;
use glium::Surface;
use std::rc::Rc;

use winit::event::MouseButton;
use glium::{ uniform };

pub mod rect;
pub mod filled_rect;
pub mod color;
pub mod rusttype;
pub mod label;
pub mod circle;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub p: [f32; 2],
}

implement_vertex!(Vertex, p);

pub enum Shape {
    Rect(rect::Rect),
    Circle(circle::Circle),
}

struct WindowOptions {

}

pub trait Draw {
    fn get_color(&self) -> &color::Color;

    fn draw(
        &self,
        window: &Window,
        frame: &mut glium::Frame
    ) {
        let uniforms = uniform! {
            matrix: [
                [ 2.0 / window.size.0 as f32, 0.0, 0.0, 0.0 ],
                [ 0.0,-2.0 / window.size.1 as f32, 0.0, 0.0 ],
                [ 0.0, 0.0, 1.0, 0.0 ],
                [-1.0,  1.0, 0.0, 1.0f32 ],
            ],
            color_input: self.get_color().v
        };

        match self.get_shape() {
            Shape::Rect(r) => {
                let vertexes = r.get_vertexes();
                let vertex_buffer = glium::VertexBuffer::new(&window.display, &vertexes).unwrap();
                let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

                let vertex_shader_src = r#"
                    #version 140

                    in vec2 p;
                    uniform mat4 matrix;

                    void main() {
                    gl_Position = matrix * vec4(p, 0.0, 1.0);
                    }
                    "#;
                let fragment_shader_src = r#"
                    #version 140

                    uniform vec4 color_input;
                    out vec4 color;

                    void main() {
                    color = color_input;
                   }
                   "#;
                let program = glium::Program::from_source(&window.display, vertex_shader_src, fragment_shader_src, None).unwrap();

                frame.draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default()
                ).unwrap();
            },
            _ => return,
        };
    }

    fn get_shape(
        &self
    ) -> &Shape;

    fn get_options(&self) -> &WindowOptions;

    fn in_bounds(&self, window: &Window) -> bool {
        match self.get_shape() {
            Shape::Rect(r) => {
                let size = r.get_size();
                let top_left = r.get_top_left();

                if window.mouse_pos.0 < top_left.p[0] + size.0 && window.mouse_pos.0 > top_left.p[0]
                && window.mouse_pos.1 < top_left.p[1] + size.1 && window.mouse_pos.1 > top_left.p[1] {
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    fn clicked(
        &self,
        window: &Window
    ) -> bool {
        if self.in_bounds(window) && window.clicked {
            println!("clicked!");
            true
        } else {
            false
        }
    }
}

pub struct Window {
    window: winit::window::Window,
    display: Display<WindowSurface>,
    size: (u32, u32),
    font: rusttype::FontTexture,
    text_system: rusttype::TextSystem,
    objects: Vec<Rc<dyn Draw>>,
    mouse_pos: (f32, f32),
    clicked: bool
}

impl Window {
    pub fn new() -> (Self, EventLoop<()>) {
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .expect("event loop failed to build!, probaly trying to build not within main thread");

        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_inner_size(1400, 800)
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
            objects: Vec::new(),
            mouse_pos: (0.0, 0.0),
            clicked: false
        } , event_loop)
    }

    pub fn draw(&mut self) {
        let mut frame = self.display.draw();

        self.size = (self.window.inner_size().width, self.window.inner_size().height);

        frame.clear_color(1.0, 1.0, 1.0, 1.0);

        let mut objects = std::mem::take(&mut self.objects);

        objects.iter().for_each(|object| {
            object.as_ref().draw(self, &mut frame);
            object.as_ref().clicked(self);
        });

        // reset clicked
        self.clicked = false;
        self.objects = std::mem::take(&mut objects);

        frame.finish().unwrap();
    }

    pub fn run_loop(&mut self, event_loop: EventLoop<()>) {
        #[allow(deprecated)]
        event_loop.run(move |event, window_target| {
            match event {
                glium::winit::event::Event::WindowEvent { event, .. } => match event {
                    // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                    glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    glium::winit::event::WindowEvent::CursorMoved { position, .. } => {
                        self.mouse_pos = (position.x as f32, position.y as f32);
                    },
                    glium::winit::event::WindowEvent::MouseInput { state, button, .. } => {
                        if state.is_pressed() && button == MouseButton::Left {
                            self.clicked = true;
                        } else {
                            self.clicked = false;
                        }
                    },
                    glium::winit::event::WindowEvent::RedrawRequested => {
                        self.draw();
                        self.window.request_redraw()
                    },
                    _ => (),
                },
                _ => (),
            };
        })
        .unwrap();
    }

    pub fn add_to_draw_list(&mut self, object: Rc<dyn Draw>) {
        self.objects.push(object);
    }
}
