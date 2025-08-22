use crate::*;

#[derive(Default, Copy, Clone)]
pub struct Rect {
    pub top_left: Vertex,
    pub width: f32,
    pub height: f32,
    color: color::Color,
    filled: bool,
    thickness: f32
}

impl Rect {
    pub fn new(top_left: Vertex, width: u32, height: u32, color: color::Color, filled: bool, thickness: f32) -> Self {
        Self {
            top_left,
            width: width as f32,
            height: height as f32,
            color,
            filled,
            thickness,
        }
    }
    pub fn get_top_left(&self) -> &Vertex {
        &self.top_left
    }
    pub fn get_size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
    pub fn get_vertexes(&self) -> [Vertex; 4] {
        return [ Vertex { p: [ self.top_left.p[0], self.top_left.p[1] ] },
        Vertex { p: [ self.top_left.p[0] + self.width, self.top_left.p[1]] },
        Vertex { p: [ self.top_left.p[0] + self.width, self.top_left.p[1] + self.height] },
        Vertex { p: [ self.top_left.p[0], self.top_left.p[1] + self.height] } ]
    }
    pub fn is_filled(&self) -> &bool{
        &self.filled
    }
    pub fn get_color(&self) -> &color::Color {
        &self.color
    }

    pub fn in_bounds(&self, window: &Window) -> bool {
        let size = self.get_size();
        let top_left = self.get_top_left();

        if window.mouse_pos.0 < top_left.p[0] + size.0 && window.mouse_pos.0 > top_left.p[0]
            && window.mouse_pos.1 < top_left.p[1] + size.1 && window.mouse_pos.1 > top_left.p[1] {
                true
            } else {
                false
            }
    }

    pub fn draw(
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

        let vertexes = self.get_vertexes();
        let vertex_buffer = glium::VertexBuffer::new(&window.display, &vertexes).unwrap();
        let indices = match self.is_filled() {
            true => { glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan) },
            false => { glium::index::NoIndices(glium::index::PrimitiveType::LineLoop) },
        };

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

        let params = glium::DrawParameters {
            line_width: Some(self.thickness),
            .. Default::default()
        };

        let program = glium::Program::from_source(&window.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        frame.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &params
        ).unwrap();
    }
}
