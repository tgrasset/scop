use std::env;
use std::sync::mpsc::Receiver;

use globals::{WIN_HEIGHT, WIN_WIDTH};

extern crate gl;
extern crate glfw;

extern crate glm; ////// INTERDIT !!!!
use glm::vec3;
use glm::Matrix4;
use glm::mat4;
use glm::ext::*;

use self::glfw::{Context, Key, Action};

mod obj_parser;
mod models;
mod globals;
mod init_opengl;
mod compile_shaders;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: Wrong number of arguments\nUsage: ./scop path/to/object/file");
        std::process::exit(1);
    }
    let objdata = match obj_parser::parse_obj_file(&args[1]) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error while parsing obj file: {}", err);
            std::process::exit(1);
        }
    };
    println!("OBJ data: {}", objdata);
    let mut glvar = match init_opengl::init_window(WIN_WIDTH, WIN_HEIGHT) {
        Ok(vars) => vars,
        Err(err) => {
            eprintln!("Error while initializing window: {}", err);
            std::process::exit(1);
        }
    };
    let vao = unsafe {init_opengl::send_data_to_opengl(&objdata)};
    match compile_shaders::compile_shaders() {
        Ok(shader_prgm_id) => glvar.set_shader_prgm_id(shader_prgm_id),
        Err(err) => {
            eprintln!("Error while compiling shaders: {}", err);
            std::process::exit(1);
        }
    }
    println!("Rendering...");
    while !glvar.window.should_close() {
        process_events(&mut glvar.window, &glvar.events);
        unsafe {
            gl::ClearColor(0.0, 0.1, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(glvar.shader_prgm_id);
            gl::BindVertexArray(vao);

            let timeValue = glvar.glfw.get_time() as f32;
            let greenValue = timeValue.sin() / 2.0 + 0.5;
            let ourColor = std::ffi::CString::new("our_color").unwrap();
            let vertexColorLocation = gl::GetUniformLocation(glvar.shader_prgm_id, ourColor.as_ptr());
            gl::Uniform4f(vertexColorLocation, 0.0, greenValue, 0.0, 1.0);

            // /// TEST
            // let model_transform_matrix = translate(&mat4(0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0), vec3(0.0, 0.0, -3.0));
            // let projection_matrix = perspective(60.0, WIN_WIDTH as f32 / WIN_HEIGHT as f32, 0.1, 10.0);
            // let model_transform_matrix_uniform_location = gl::GetUniformLocation(glvar.shader_prgm_id, "modelTransformMatrix".as_ptr() as *const gl::types::GLchar);
            // let projection_matrix_uniform_location = gl::GetUniformLocation(glvar.shader_prgm_id, "projectionMatrix".as_ptr() as *const gl::types::GLchar);

            // gl::UniformMatrix4fv(model_transform_matrix_uniform_location, 1, gl::FALSE, &model_transform_matrix[0][0]);
            // gl::UniformMatrix4fv(projection_matrix_uniform_location, 1, gl::FALSE, &projection_matrix[0][0]);
            // gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_SHORT, std::ptr::null());
            // // TEST


            gl::DrawArrays(gl::TRIANGLES, 0, 8);
        }
        glvar.window.swap_buffers();
        glvar.glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            // match viewport to window size if changed
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            // escape to close window
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}