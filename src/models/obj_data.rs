use gl::types::GLushort;

use crate::models::vec3::Vec3;

pub struct ObjData {
    pub vertices: Vec<Vertex>,
    pub num_vertices: u32,
    pub vertices_raw: Vec<f32>,   //// MIGHT BE USELESS
    pub vertex_buffer_size: usize,
    pub indices: Vec<GLushort>,
    pub num_indices: usize,
    pub indices_buffer_size: usize,
}

impl std::fmt::Display for ObjData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Vertices ({}):", self.num_vertices)?;
        for (i, vertex) in self.vertices.iter().enumerate() {
            writeln!(f, "  Vertex {}: {}", i, vertex)?;
        }
        writeln!(f, "Indices ({}): {:?}", self.num_indices, self.indices)
    }
}

pub struct Vertex {
    pub position: Vec3,
    pub rgb: Option<Vec3>,
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position: {}", self.position)?;
        if let Some(rgb) = &self.rgb {
            write!(f, ", RGB: {}", rgb)?;
        }
        Ok(())
    }
}

pub struct Face {
    pub indices: Vec<GLushort>,
}