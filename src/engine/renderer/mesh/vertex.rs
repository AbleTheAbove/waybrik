use gl::types::GLfloat;

#[derive(Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        let color = [1.0, 0.0, 0.0];
        Self {
            position: [x, y],
            color,
        }
    }
}
