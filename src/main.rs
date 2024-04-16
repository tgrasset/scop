use std::env;
use std::sync::mpsc::Receiver;

use globals::{WIN_HEIGHT, WIN_WIDTH};

extern crate gl;
extern crate glfw;
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
    // println!("OBJ data: {}", objdata);
    let mut glvar = match init_opengl::init_window(WIN_WIDTH, WIN_HEIGHT) {
        Ok(vars) => vars,
        Err(err) => {
            eprintln!("Error while initializing window: {}", err);
            std::process::exit(1);
        }
    };
    unsafe {init_opengl::send_data_to_opengl(&objdata);}
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
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // gl::BindVertexArray(vao);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
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