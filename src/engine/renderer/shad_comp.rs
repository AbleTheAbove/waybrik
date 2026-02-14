use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

// TODO: make some sort of asset loader for shaders textual rep
pub static VS_SRC: &str = include_str!("../../../assets/addons/core/shaders/brik/shader.vert");
pub static FS_SRC: &str = include_str!("../../../assets/addons/core/shaders/brik/shader.frag");

// TODO: make this automatically happen when the asset loader finds a shader.
pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            let mut outlen = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            gl::GetShaderInfoLog(
                shader,
                len,
                &mut outlen, // Number of characters written excluding null terminator
                buf.as_mut_ptr() as *mut GLchar,
            );
            buf.set_len(outlen as usize);
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}
pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            let mut outlen = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            gl::GetProgramInfoLog(
                program,
                len,
                &mut outlen, // Number of characters written excluding null terminator
                buf.as_mut_ptr() as *mut GLchar,
            );
            buf.set_len(outlen as usize);
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}
