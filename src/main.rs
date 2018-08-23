extern crate gl;
extern crate glutin;

use std::mem::size_of;

use gl::types::*;

use glutin::dpi::*;
use glutin::*;

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

fn process_events(events_loop: &mut EventsLoop) -> bool {
    let mut close_requested = false;

    // Destructure and handle events as expected.
    events_loop.poll_events(|event| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => close_requested = true,
            WindowEvent::Resized(size) => { /* TODO */ }
            _ => (),
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::Key(keyboard_input) => close_requested = true,
            _ => (),
        },
        _ => (),
    });

    // Indicate whether a close was requested.
    close_requested
}

fn main() {
    let (window, mut events_loop) = create_window("Hello, world!".into());

    let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    unsafe {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
    }

    loop {
        let close_requested = process_events(&mut events_loop);
        if close_requested {
            break;
        }

        unsafe {
            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers().unwrap();
    }
}
