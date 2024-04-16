use gl::types::GLushort;

use crate::models::vec3::Vec3;

pub struct ObjData {
    pub vertices: Vec<Vertex>,
    pub num_vertices: u32,
    pub faces: Vec<Face>,
    pub indices: Vec<GLushort>,
    pub num_indices: u32,
}

pub struct Vertex {
    pub position: Vec3,
    pub rgb: Option<Vec3>,
}

pub struct Face {
    pub indices: Vec<usize>,
}