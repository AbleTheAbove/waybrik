use std::{collections::HashMap, ffi::CString, ptr};

use gl::types::{GLboolean, GLfloat, GLsizeiptr, GLuint};

use crate::engine::renderer::shad_comp::{FS_SRC, VS_SRC, compile_shader, link_program};

extern crate sdl2;

static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

mod shad_comp;

pub struct Renderer {
    shader_registry: HashMap<String, u32>,
    program_registry: HashMap<String, u32>,
    vertex_array_registry: HashMap<String, u32>,
    vertex_buffer_registry: HashMap<String, u32>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            shader_registry: HashMap::new(),
            program_registry: HashMap::new(),
            vertex_array_registry: HashMap::new(),
            vertex_buffer_registry: HashMap::new(),
        }
    }
    pub fn setup(&mut self) {
        // TODO reorganize the 6 lines here
        let vs: u32 = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);
        self.shader_registry.insert("shader.vert".to_string(), vs);
        self.shader_registry.insert("shader.frag".to_string(), fs);
        self.program_registry.insert("shader".to_string(), program);
        // FINTODO
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        let mut vao = 0;
        self.vertex_array_registry
            .insert("VAO0".to_string(), program);

        let mut vbo = 0;
        self.vertex_buffer_registry
            .insert("VBO0".to_string(), program);

        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&VERTEX_DATA) as GLsizeiptr,
                VERTEX_DATA.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            // Use shader program
            gl::UseProgram(program);
            gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());

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

    pub fn cleanup(&mut self) {
        for (program_name, program_id) in self.program_registry.iter() {
            unsafe {
                gl::DeleteProgram(*program_id);
            }
        }
        for (shader_name, shader_id) in self.shader_registry.iter() {
            unsafe {
                gl::DeleteShader(*shader_id);
            }
        }

        for (vbo_name, vbo_id) in self.vertex_buffer_registry.iter() {
            unsafe {
                gl::DeleteBuffers(1, vbo_id);
            }
        }
        for (vao_name, vao_id) in self.vertex_array_registry.iter() {
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
            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
