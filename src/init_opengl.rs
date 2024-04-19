use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid};
use glfw::Context;

use std::{io::{Error, ErrorKind}, mem::size_of, os::raw::c_void};

use crate::models::{gl_var::GlVar, obj_data::ObjData};

pub fn init_window(width: u32, height: u32) -> Result<GlVar, Error> {

    println!("Initializing glfw...");
    let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
        Ok(glfw) => glfw,
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    };
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) =
        glfw.create_window(
            width,
            height,
            "scop",
            glfw::WindowMode::Windowed)
            .expect("Failed to create window");
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    
    Ok(GlVar {glfw: glfw, window: window, events: events, shader_prgm_id: 0, texture_id: 0})
}

pub unsafe fn send_data_to_opengl(obj_data: &ObjData) -> (GLuint, GLuint, GLuint) {

    let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
    gl::Enable(gl::DEPTH_TEST);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER,
                    (obj_data.vertices_raw.len() * size_of::<GLfloat>()) as GLsizeiptr,
                    obj_data.vertices_raw.as_ptr() as *const GLvoid,
                    gl::STATIC_DRAW);

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                    obj_data.indices_buffer_size as GLsizeiptr,
                    obj_data.indices.as_ptr() as *const c_void,
                    gl::STATIC_DRAW);
    let stride: i32 = 8 * size_of::<GLfloat>() as GLsizei;
    //position attribute
    gl::VertexAttribPointer(
        0,
        3, 
        gl::FLOAT,
        gl::FALSE,
        stride,
        std::ptr::null());
    gl::EnableVertexAttribArray(0);
    //color attribute
    gl::VertexAttribPointer(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);
    //texture coord attribute
    gl::VertexAttribPointer(
        2,
        2, 
        gl::FLOAT,
        gl::FALSE,
        stride,
        (6 * size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(2);
    (vao, vbo, ebo)
}