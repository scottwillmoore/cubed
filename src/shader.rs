use std;
use std::ffi::{CStr, CString};

use gl;
use gl::types::*;

pub enum ShaderType {
    Fragment,
    Geometry,
    Vertex,
}

impl Into<GLenum> for ShaderType {
    fn into(self) -> GLenum {
        match self {
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::Vertex => gl::VERTEX_SHADER,
        }
    }
}

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(source: &str, kind: GLenum) -> Shader {
        let id = unsafe { gl::CreateShader(kind) };
        let shader = Shader { id };

        let c_source = CString::new(source).unwrap();
        let success = shader.compile(&c_source);
        if !success {
            let info_log = shader.get_info_log();
            panic!("\n{}\n", info_log);
        }

        shader
    }

    fn compile(&self, source: &CStr) -> bool {
        unsafe {
            gl::ShaderSource(self.id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(self.id);
        }

        let mut success = gl::FALSE as GLint;
        unsafe { gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success) };

        success == (gl::TRUE as GLint)
    }

    fn get_info_log(&self) -> String {
        let mut len = 0;
        unsafe { gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len) };

        let mut buf: Vec<u8> = std::iter::repeat(0).take(len as usize).collect();
        unsafe {
            gl::GetShaderInfoLog(
                self.id,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            )
        };
        buf.pop();

        String::from_utf8(buf).expect("ShaderInfoLog is not valid utf8")
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}
