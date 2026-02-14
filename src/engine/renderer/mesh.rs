use std::collections::HashMap;

use gl::types::GLfloat;

pub type MeshID = u64;

static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
#[derive(Clone)]
pub struct Vertex {}

#[derive(Clone)]
pub struct Mesh {
    pub vbo: u32,
    pub vao: u32,
    pub verts: Vec<Vertex>,
}

pub struct MeshIndex {
    next_mesh_id: MeshID,
    pub index: HashMap<MeshID, Mesh>,
}
impl MeshIndex {
    pub fn new() -> Self {
        Self {
            next_mesh_id: 0,
            index: HashMap::new(),
        }
    }
    pub fn new_mesh(&mut self) -> MeshID {
        let mesh_id = self.next_mesh_id;

        let mesh = Mesh {
            vbo: 0,
            vao: 0,
            verts: vec![],
        };
        self.index.insert(mesh_id, mesh);
        self.next_mesh_id += 1;
        mesh_id
    }
}
