use std::env;

use globals::{WIN_HEIGHT, WIN_WIDTH};

extern crate gl;
extern crate glfw;

mod obj_parser;
mod models;
mod globals;
mod init_opengl;

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
    let glvar = match init_opengl::init_window(WIN_HEIGHT, WIN_WIDTH) {
        Ok(vars) => vars,
        Err(err) => {
            eprintln!("Error while initializing window: {}", err);
            std::process::exit(1);
        }
    };
}