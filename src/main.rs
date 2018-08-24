extern crate gl;
extern crate glutin;

use std::ffi::{CStr, CString};

use gl::types::*;

use glutin::dpi::*;
use glutin::*;

static VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
static FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");

fn create_window(title: String) -> (GlWindow, EventsLoop) {
    // Create a window with the following properties:
    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_dimensions(LogicalSize::new(1024.0, 760.0));

    // Create an OpenGL context with the following properties:
    let context_builder = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(GlProfile::Core)
        .with_vsync(true);

    // Create an events loop.
    let events_loop = EventsLoop::new();

    // Create a window using the window, context and events loop.
    let window = GlWindow::new(window_builder, context_builder, &events_loop).unwrap();

    // Make the context active in the current thread.
    unsafe {
        window.make_current().unwrap();
    }

    // Load the correct OpenGL symbols from the context that was created.
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    (window, events_loop)
}

fn process_events(window: &GlWindow, events_loop: &mut EventsLoop) -> bool {
    let mut close_requested = false;

    // Destructure and handle events as expected.
    events_loop.poll_events(|event| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => close_requested = true,
            WindowEvent::Resized(logical_size) => {
                let dpi_factor = window.get_hidpi_factor();
                let physical_size = logical_size.to_physical(dpi_factor);

                window.resize(physical_size);

                let PhysicalSize { width, height } = physical_size;
                unsafe {
                    gl::Viewport(0, 0, width as GLint, height as GLint);
                }
            }
            _ => (),
        },
        _ => (),
    });

    // Indicate whether a close was requested.
    close_requested
}

fn create_shader(source: &CStr, kind: GLenum) -> GLuint {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == (gl::FALSE as GLint) {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buf = Vec::with_capacity(len as usize);
        unsafe {
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        panic!(
            "{}",
            std::str::from_utf8(&buf)
                .ok()
                .expect("ShaderInfoLog is not valid utf8")
        );
    }

    id
}

fn create_shader_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let id = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(id, vertex_shader);
        gl::AttachShader(id, fragment_shader);
        gl::LinkProgram(id);
    }

    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
    }

    if success == (gl::FALSE as GLint) {
        let mut len = 0;
        unsafe {
            gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buf = Vec::with_capacity(len as usize);
        unsafe {
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        panic!(
            "{}",
            std::str::from_utf8(&buf)
                .ok()
                .expect("ShaderInfoLog is not valid utf8")
        );
    }

    // NOTE: This shouldn't be in this function.
    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    id
}

fn main() {
    // Create the window and events loop.
    let (window, mut events_loop) = create_window("Hello, world!".into());

    // Load the vertex and fragment shader source as CStrings.
    let vertex_source = CString::new(VERTEX_SHADER).unwrap();
    let fragment_source = CString::new(FRAGMENT_SHADER).unwrap();

    // Create and compile the shaders.
    let vertex_shader = create_shader(&vertex_source, gl::VERTEX_SHADER);
    let fragment_shader = create_shader(&fragment_source, gl::FRAGMENT_SHADER);

    // Create and link the shader program.
    let shader_program = create_shader_program(vertex_shader, fragment_shader);

    // Define the vertices used for the triangle.
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let vertices: [GLfloat; 9] = [
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0, 0.5, 0.0
    ];

    // Create the VBO and VAO.
    let mut vbo = 0;
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<GLfloat>()) as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    loop {
        let close_requested = process_events(&window, &mut events_loop);
        if close_requested {
            break;
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }

        window.swap_buffers().unwrap();
    }
}
