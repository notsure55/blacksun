use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;
use winit::keyboard::NamedKey;
use winit::{ event_loop::EventLoop };
use glium::implement_vertex;
use glium::Surface;
use std::rc::Rc;

use winit::keyboard::*;

use winit::event::MouseButton;
use glium::{ uniform };

pub mod button;
pub mod rect;
pub mod color;
pub mod rusttype;
pub mod label;
pub mod circle;
pub mod text;
pub mod traits;
pub mod shape;
pub mod grid;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub p: [f32; 2],
}

implement_vertex!(Vertex, p);

struct KeyBoard {
    input: String,
    backspace: bool,
}

impl KeyBoard {
    fn new() -> Self {
        Self {
            input: String::new(),
            backspace: false,
        }
    }
}

pub struct Window {
    window: winit::window::Window,
    display: Display<WindowSurface>,
    size: (u32, u32),
    font: rusttype::FontTexture,
    text_system: rusttype::TextSystem,
    objects: Vec<Rc<dyn traits::Drawable>>,
    mouse_pos: (f32, f32),
    clicked: bool,
    keyboard: KeyBoard
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
            clicked: false,
            keyboard: KeyBoard::new(),
        } , event_loop)
    }

    pub fn draw(&mut self) {
        let mut frame = self.display.draw();

        self.size = (self.window.inner_size().width, self.window.inner_size().height);

        frame.clear_color(1.0, 1.0, 1.0, 1.0);

        let mut objects = std::mem::take(&mut self.objects);

        objects.iter().for_each(|object| {
            object.draw(self, &mut frame);
            object.clicked(self);
            object.selected(self);
        });

        // reset clicked
        self.clicked = false;
        self.clear_keyboard();
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
                    glium::winit::event::WindowEvent::KeyboardInput { event, .. } => {
                        if event.state.is_pressed() {
                            match event.logical_key {
                                Key::Named(n) => {
                                    match n {
                                        NamedKey::Backspace => {
                                            self.keyboard.backspace = true
                                        },
                                        NamedKey::Space => {
                                            self.keyboard.input.push(' ');
                                        },
                                        _ => (),
                                    }
                                },
                                Key::Character(c) => {
                                    self.keyboard.input.push_str(c.as_str())
                                }
                                _ => (),
                            }
                        }
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
    pub fn get_keyboard(&self) -> &KeyBoard {
        &self.keyboard
    }

    pub fn clear_keyboard(&mut self) {
        self.keyboard.input = String::new();
        self.keyboard.backspace = false;
    }

    pub fn add_to_draw_list(&mut self, object: Rc<dyn traits::Drawable>) {
        self.objects.push(object);
    }
}
