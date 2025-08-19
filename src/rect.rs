use crate::Vertex;

pub struct Rect {
    top_left: Vertex,
    width: u32,
    height: u32,
}

impl Rect {
    pub fn new(top_left: Vertex, width: u32, height: u32) -> Self {
        Self {
            top_left,
            width,
            height,
        }
    }
    pub fn get_top_left(&self) -> &Vertex {
        &self.top_left
    }
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    pub fn get_vertexes(&self) -> [Vertex; 4] {
        return [ Vertex { p: [ self.top_left.p[0], self.top_left.p[1] ] },
        Vertex { p: [ self.top_left.p[0] + self.width, self.top_left.p[1]] },
        Vertex { p: [ self.top_left.p[0] + self.width, self.top_left.p[1] + self.height] },
        Vertex { p: [ self.top_left.p[0], self.top_left.p[1] + self.height] } ]
    }
}
