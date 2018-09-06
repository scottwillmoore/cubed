use std;
use std::ffi::{CStr, CString};

use gl;
use gl::types::*;

use shader::Shader;

pub struct Uniform {
    pub id: GLint,
}

pub enum UniformData {
    Float(GLfloat),
    FloatVec2([GLfloat; 2]),
    FloatVec3([GLfloat; 3]),
    FloatVec4([GLfloat; 4]),
    FloatMat2([GLfloat; 2 * 2]),
    FloatMat3([GLfloat; 3 * 3]),
    FloatMat4([GLfloat; 4 * 4]),
    FloatMat2x3([GLfloat; 2 * 3]),
    FloatMat2x4([GLfloat; 2 * 4]),
    FloatMat3x2([GLfloat; 3 * 2]),
    FloatMat3x4([GLfloat; 3 * 4]),
    FloatMat4x2([GLfloat; 4 * 2]),
    FloatMat4x3([GLfloat; 4 * 3]),
}

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

    pub fn get_uniform_location(&self, name: &str) -> Uniform {
        let c_name = CString::new(name).unwrap();
        let id = unsafe { gl::GetUniformLocation(self.id, c_name.as_ptr()) };
        Uniform { id }
    }

    pub fn set_uniform(&self, uniform: Uniform, data: UniformData) {
        use UniformData::*;

        let id = uniform.id;
        match data {
            Float(data) => unsafe { gl::Uniform1fv(id, 1, &data) },
            FloatVec2(data) => unsafe { gl::Uniform2fv(id, 1, data.as_ptr()) },
            FloatVec3(data) => unsafe { gl::Uniform3fv(id, 1, data.as_ptr()) },
            FloatVec4(data) => unsafe { gl::Uniform4fv(id, 1, data.as_ptr()) },
            FloatMat2(data) => unsafe { gl::UniformMatrix2fv(id, 1, gl::FALSE, data.as_ptr()) },
            FloatMat3(data) => unsafe { gl::UniformMatrix3fv(id, 1, gl::FALSE, data.as_ptr()) },
            FloatMat4(data) => unsafe { gl::UniformMatrix4fv(id, 1, gl::FALSE, data.as_ptr()) },
            FloatMat2x3(data) => unsafe { gl::UniformMatrix2x3fv(id, 1, gl::FALSE, data.as_ptr()) },
            FloatMat2x4(data) => unsafe { gl::UniformMatrix2x4fv(id, 1, gl::FALSE, data.as_ptr()) },
            FloatMat3x2(data) => unsafe { gl::UniformMatrix3x2fv(id, 1, gl::FALSE, data.as_ptr()) },
            FloatMat3x4(data) => unsafe { gl::UniformMatrix3x4fv(id, 1, gl::FALSE, data.as_ptr()) },
            FloatMat4x2(data) => unsafe { gl::UniformMatrix4x2fv(id, 1, gl::FALSE, data.as_ptr()) },
            FloatMat4x3(data) => unsafe { gl::UniformMatrix4x3fv(id, 1, gl::FALSE, data.as_ptr()) },
        };
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
