use crate::models::vec3::Vec3;

pub struct ObjData {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

pub struct Vertex {
    pub position: Vec3,
    pub rgb: Option<Vec3>,
}

pub struct Face {
    pub indices: Vec<usize>,
}