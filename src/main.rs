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
mod texture_loader;
mod render;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Error: Wrong number of arguments\nUsage: ./scop path/to/object/file path/to/bmp/texture");
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
    let (vao, vbo, ebo) = unsafe {init_opengl::send_data_to_opengl(&objdata)};
    match compile_shaders::compile_shaders() {
        Ok(shader_prgm_id) => glvar.set_shader_prgm_id(shader_prgm_id),
        Err(err) => {
            eprintln!("Error while compiling shaders: {}", err);
            std::process::exit(1);
        }
    }
    match texture_loader::load_texture(&args[2]) {
        Ok(texture_id) => glvar.set_texture_id(texture_id),
        Err(err) => {
            eprintln!("Error while loading texture: {}", err);
            std::process::exit(1);
        }
    };
    render::render_loop(&mut glvar, &vao);
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }
}