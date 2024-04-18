
use std::ops::Mul;

pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Mat4 {
    // Create a new identity matrix
    pub fn identity() -> Self {
        Mat4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // Multiply two matrices
    pub fn multiply(a: Mat4, b: Mat4) -> Mat4 {
        let mut result = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = a.data[i][0] * b.data[0][j]
                                  + a.data[i][1] * b.data[1][j]
                                  + a.data[i][2] * b.data[2][j]
                                  + a.data[i][3] * b.data[3][j];
            }
        }
        result
    }

    // Apply translation to the matrix
    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.data[3][0] += x;
        self.data[3][1] += y;
        self.data[3][2] += z;
        self
    }

    // Apply rotation around the X axis
    pub fn rotate_x(mut self, angle: f32) -> Self {
        let (sin_a, cos_a) = angle.sin_cos();
        let row1 = self.data[1];
        let row2 = self.data[2];
        self.data[1][1] = row1[1] * cos_a - row2[1] * sin_a;
        self.data[1][2] = row1[2] * cos_a - row2[2] * sin_a;
        self.data[2][1] = row1[1] * sin_a + row2[1] * cos_a;
        self.data[2][2] = row1[2] * sin_a + row2[2] * cos_a;
        self
    }

    // Apply rotation around the Y axis
    pub fn rotate_y(mut self, angle: f32) -> Self {
        let (sin_a, cos_a) = angle.sin_cos();
        let row0 = self.data[0];
        let row2 = self.data[2];
        self.data[0][0] = row0[0] * cos_a - row2[0] * sin_a;
        self.data[0][2] = row0[0] * sin_a + row2[0] * cos_a;
        self.data[2][0] = row0[2] * cos_a + row2[2] * sin_a;
        self.data[2][2] = -row0[2] * sin_a + row2[2] * cos_a;
        self
    }

    // Apply rotation around the Z axis
    pub fn rotate_z(mut self, angle: f32) -> Self {
        let (sin_a, cos_a) = angle.sin_cos();
        let row0 = self.data[0];
        let row1 = self.data[1];
        self.data[0][0] = row0[0] * cos_a - row1[0] * sin_a;
        self.data[0][1] = row0[1] * cos_a - row1[1] * sin_a;
        self.data[1][0] = row0[0] * sin_a + row1[0] * cos_a;
        self.data[1][1] = row0[1] * sin_a + row1[1] * cos_a;
        self
    }

    // Apply scaling to the matrix
    pub fn scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.data[0][0] *= x;
        self.data[1][1] *= y;
        self.data[2][2] *= z;
        self
    }

    pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
        let f = 1.0 / (fov / 2.0).tan();
        let range_inv = 1.0 / (near - far);

        Mat4 {
            data: [
                [f / aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (near + far) * range_inv, -1.0],
                [0.0, 0.0, near * far * range_inv * 2.0, 0.0],
            ],
        }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        let mut result = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][0] * rhs.data[0][j]
                                  + self.data[i][1] * rhs.data[1][j]
                                  + self.data[i][2] * rhs.data[2][j]
                                  + self.data[i][3] * rhs.data[3][j];
            }
        }
        result
    }
}