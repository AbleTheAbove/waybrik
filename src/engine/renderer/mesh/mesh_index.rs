use std::collections::HashMap;

use log::info;

use crate::engine::renderer::mesh::{Mesh, MeshID, Vertex};

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
            is_dirty: true,
        };
        self.index.insert(mesh_id, mesh);
        self.next_mesh_id += 1;
        mesh_id
    }
    pub fn set_mesh(&mut self, mesh_id: MeshID, verts: Vec<Vertex>) {
        let mesh = self.index.get_mut(&mesh_id).unwrap();
        mesh.verts = verts;
        mesh.is_dirty = true;
        info!("Mesh set.");
    }
}
