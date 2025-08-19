use crate::{ color::Color, Window, rect::*, Draw };
use glium::{ Surface, uniform, Frame };

pub struct FilledRect {
    pub rect: Rect,
    color: Color,
}

impl FilledRect {
    pub fn new(rect: Rect, color: Color) -> Self {
        Self {
            rect,
            color,
        }
    }
}

impl Draw for FilledRect {
    fn draw(
        &self,
        window: &Window,
        frame: &mut Frame
    ) {
        let uniforms = uniform! {
            screen_size: [window.size.0 as f32, window.size.1 as f32],
            color_input: self.color.v
        };

        let vertex_buffer = glium::VertexBuffer::new(&window.display, &self.rect.get_vertexes()).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

        let vertex_shader_src = r#"
        #version 140

        in vec2 p;
        uniform vec2 screen_size;

        void main() {
        vec2 zero_to_one = p / screen_size;
        vec2 zero_to_two = zero_to_one * 2.0;
        vec2 clip_space = zero_to_two - 1.0;
        clip_space.y = -clip_space.y;

        gl_Position = vec4(clip_space, 0.0, 1.0);
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
    }
}
