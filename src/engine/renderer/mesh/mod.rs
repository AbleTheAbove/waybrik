use std::{collections::HashMap, ffi::CString, ptr};

use gl::types::{GLboolean, GLfloat, GLsizeiptr, GLuint};

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
    pub program: u32,
    pub verts: Vec<Vertex>,
}
impl Mesh {
    // Handle taking a vec of verts and building a mesh.
    pub fn new(program: u32, verts: Vec<Vertex>) -> Self {
        let mut verts_flat = vec![];
        for vert in verts.clone() {
            verts_flat.push(vert.position[0]);
            verts_flat.push(vert.position[1]);
        }
        let mut vao = 0;
        let mut vbo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&verts_flat) as GLsizeiptr,
                verts_flat.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }

        // {
        //     info!("VAO ID {}", vao);
        //     self.vertex_array_registry.insert("VAO0".to_string(), vao);
        //     self.vertex_buffer_registry.insert("VBO0".to_string(), vbo);
        //     println!("Mesh count {}", self.meshes.index.len());
        //     let mesh = self.meshes.index.get_mut(&mesh_id).unwrap();
        //     // mesh.vao = vao;
        //     // println!("Mesh vao {}", mesh.vao);
        //     // mesh.vbo = vbo;
        //     // println!("Mesh vbo {}", mesh.vbo);
        // }
        unsafe {
            {
                // Use shader program
                gl::UseProgram(program);
                gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());
            }

            {
                // Specify the layout of the vertex data
                let pos_attr =
                    gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
                gl::EnableVertexAttribArray(pos_attr as GLuint);
                gl::VertexAttribPointer(
                    pos_attr as GLuint,
                    2,
                    gl::FLOAT,
                    gl::FALSE as GLboolean,
                    0,
                    ptr::null(),
                );
            }
        }

        Self {
            is_dirty: true,
            vbo,
            vao,
            verts: vec![],
            program,
        }
    }
    pub fn render(&self) {
        let vbo = self.vbo;
        let vao = self.vao;

        unsafe {
            trace!("Binding Vertex Array");
            gl::BindVertexArray(vao);
            trace!("Binding Vertex Buffer Array");
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // TODO: keep track of triangle count on the mesh.
            gl::DrawArrays(gl::TRIANGLES, 1, 3);
        }
    }
    pub fn gpu_upload(&mut self) {
        let mut verts_flat = vec![];
        for vert in self.verts.clone() {
            verts_flat.push(vert.position[0]);
            verts_flat.push(vert.position[1]);
        }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&verts_flat) as GLsizeiptr,
                verts_flat.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }
    }
}
