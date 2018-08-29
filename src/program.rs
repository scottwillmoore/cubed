use std;
use std::ffi::CStr;

use gl;
use gl::types::*;

use shader::Shader;

pub struct Program {
    pub id: GLuint,
}

impl Program {
    pub fn new(shaders: &[Shader]) -> Program {
        let id = unsafe { gl::CreateProgram() };
        let program = Program { id };

        let success = program.link(shaders);
        if !success {
            let info_log = program.get_info_log();
            panic!("{}", info_log);
        }

        program
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    fn link(&self, shaders: &[Shader]) -> bool {
        for shader in shaders {
            unsafe { gl::AttachShader(self.id, shader.id) };
        }

        unsafe { gl::LinkProgram(self.id) };

        let mut success = gl::FALSE as GLint;
        unsafe { gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success) };

        success == (gl::TRUE as GLint)
    }

    fn get_info_log(&self) -> String {
        let mut len = 0;
        unsafe { gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len) };

        let mut buf: Vec<u8> = std::iter::repeat(0).take(len as usize).collect();
        unsafe {
            gl::GetProgramInfoLog(
                self.id,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            )
        };
        buf.pop();

        String::from_utf8(buf).expect("ProgramInfoLog is not valid utf8")
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }
}
