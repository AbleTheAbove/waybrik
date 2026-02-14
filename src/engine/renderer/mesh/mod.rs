use std::collections::HashMap;

use gl::types::GLfloat;

pub mod mesh_index;
use log::{info, trace};
use mesh_index::MeshIndex;

use crate::engine::renderer::mesh::vertex::Vertex;

pub type MeshID = u64;
pub mod vertex;

#[derive(Clone)]
pub struct Mesh {
    /// Set this true when the mesh needs to be rebuilt.
    pub is_dirty: bool,
    pub vbo: u32,
    pub vao: u32,
    pub verts: Vec<Vertex>,
}
impl Mesh {
    // Handle taking a vec of verts and building a mesh.
    pub fn new() -> Self {
        Self {
            is_dirty: true,
            vbo: 0,
            vao: 0,
            verts: vec![],
        }
    }
    pub fn render(&self) {
        let vbo = self.vbo;
        let vao = self.vao;

        unsafe {
            info!("Binding Vertex Array");
            gl::BindVertexArray(vao);
            info!("Binding Vertex Buffer Array");
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // TODO: keep track of triangle count on the mesh.
            gl::DrawArrays(gl::TRIANGLES, 1, 3);
        }
    }
}
