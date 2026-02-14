use std::{collections::HashMap, ffi::CString, ptr};

use gl::types::{GLboolean, GLfloat, GLsizeiptr, GLuint};
use log::{debug, info, trace};

use crate::engine::renderer::{
    mesh::{Mesh, MeshID, mesh_index::MeshIndex, vertex::VERTEX_DATA},
    shad_comp::{FS_SRC, VS_SRC, compile_shader, link_program},
};

extern crate sdl2;
use crate::engine::renderer::mesh::vertex::Vertex;

mod mesh;
mod shad_comp;

pub struct Renderer {
    shader_registry: HashMap<String, u32>,
    program_registry: HashMap<String, u32>,
    vertex_array_registry: HashMap<String, u32>,
    vertex_buffer_registry: HashMap<String, u32>,
    pub meshes: MeshIndex,
}

impl Renderer {
    pub fn new() -> Self {
        let meshes = MeshIndex::new();

        Self {
            shader_registry: HashMap::new(),
            program_registry: HashMap::new(),
            vertex_array_registry: HashMap::new(),
            vertex_buffer_registry: HashMap::new(),
            meshes,
        }
    }

    pub fn setup(&mut self) {
        let verts = vec![
            Vertex {
                position: [0.0, 0.5],
            },
            Vertex {
                position: [0.5, -0.5],
            },
            Vertex {
                position: [-0.5, -0.5],
            },
        ];

        // TODO reorganize the 6 lines here into the asset loader
        let vs: u32 = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);
        self.shader_registry.insert("shader.vert".to_string(), vs);
        self.shader_registry.insert("shader.frag".to_string(), fs);
        self.program_registry.insert("shader".to_string(), program);
        // FINTODO

        let _mesh_id = self.meshes.new_mesh(verts, program);
    }

    pub fn cleanup(&mut self) {
        for (_program_name, program_id) in self.program_registry.iter() {
            unsafe {
                gl::DeleteProgram(*program_id);
            }
        }
        for (_shader_name, shader_id) in self.shader_registry.iter() {
            unsafe {
                gl::DeleteShader(*shader_id);
            }
        }

        for (_vbo_name, vbo_id) in self.vertex_buffer_registry.iter() {
            unsafe {
                gl::DeleteBuffers(1, vbo_id);
            }
        }
        for (_vao_name, vao_id) in self.vertex_array_registry.iter() {
            unsafe {
                gl::DeleteVertexArrays(1, vao_id);
            }
        }
    }

    pub fn render(&mut self) {
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for (mesh_id, mesh) in self.meshes.index.iter_mut() {
            trace!("Rendering mesh-ID({})", mesh_id);
            mesh.render();
        }
    }
}
