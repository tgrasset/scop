
use std::ops::Mul;
use std::ops::MulAssign;

// #[derive(Clone)]
// pub struct Mat4 {
//     pub data: [[f32; 4]; 4],
// }

// impl Mat4 {
//     // Create a new identity matrix
//     pub fn identity() -> Self {
//         Mat4 {
//             data: [
//                 [1.0, 0.0, 0.0, 0.0],
//                 [0.0, 1.0, 0.0, 0.0],
//                 [0.0, 0.0, 1.0, 0.0],
//                 [0.0, 0.0, 0.0, 1.0],
//             ],
//         }
//     }

//     // Multiply two matrices
//     pub fn multiply(a: &Mat4, b: &Mat4) -> Mat4 {
//         let mut result = Mat4::identity();
//         for i in 0..4 {
//             for j in 0..4 {
//                 result.data[i][j] = a.data[i][0] * b.data[0][j]
//                                   + a.data[i][1] * b.data[1][j]
//                                   + a.data[i][2] * b.data[2][j]
//                                   + a.data[i][3] * b.data[3][j];
//             }
//         }
//         result
//     }

//     // Apply translation to the matrix
//     pub fn translate(&self, x: f32, y: f32, z: f32) -> Mat4 {
//         let mut result = self.clone();
//         result.data[3][0] += x;
//         result.data[3][1] += y;
//         result.data[3][2] += z;
//         result
//     }

//     // Apply rotation around the X axis
//     pub fn rotate_x(&self, angle: f32) -> Mat4 {
//         let (sin_a, cos_a) = angle.sin_cos();
//         let mut result = self.clone();
//         let row1 = self.data[1];
//         let row2 = self.data[2];
//         result.data[1][1] = row1[1] * cos_a - row2[1] * sin_a;
//         result.data[1][2] = row1[1] * sin_a + row2[1] * cos_a;
//         result.data[2][1] = row1[2] * cos_a - row2[2] * sin_a;
//         result.data[2][2] = row1[2] * sin_a + row2[2] * cos_a;
//         result
//     }

//     // Apply rotation around the Y axis
//     pub fn rotate_y(&self, angle: f32) -> Mat4 {
//         let (sin_a, cos_a) = angle.sin_cos();
//         let mut result = self.clone();
//         let row0 = self.data[0];
//         let row2 = self.data[2];
//         result.data[0][0] = row0[0] * cos_a + row2[0] * sin_a;
//         result.data[0][2] = -row0[0] * sin_a + row2[0] * cos_a;
//         result.data[2][0] = row0[2] * cos_a - row2[2] * sin_a;
//         result.data[2][2] = row0[2] * sin_a + row2[2] * cos_a;
//         result
//     }

//     // Apply rotation around the Z axis
//     pub fn rotate_z(&self, angle: f32) -> Mat4 {
//         let (sin_a, cos_a) = angle.sin_cos();
//         let mut result = self.clone();
//         let row0 = self.data[0];
//         let row1 = self.data[1];
//         result.data[0][0] = row0[0] * cos_a - row1[0] * sin_a;
//         result.data[0][1] = row0[0] * sin_a + row1[0] * cos_a;
//         result.data[1][0] = row0[1] * cos_a - row1[1] * sin_a;
//         result.data[1][1] = row0[1] * sin_a + row1[1] * cos_a;
//         result
//     }

//     // Apply scaling to the matrix
//     pub fn scale(&self, x: f32, y: f32, z: f32) -> Mat4 {
//         let mut result = self.clone();
//         result.data[0][0] *= x;
//         result.data[1][1] *= y;
//         result.data[2][2] *= z;
//         result
//     }

//     pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
//         let f = 1.0 / (fov / 2.0).tan();
//         let range_inv = 1.0 / (near - far);

//         Mat4 {
//             data: [
//                 [f / aspect_ratio, 0.0, 0.0, 0.0],
//                 [0.0, f, 0.0, 0.0],
//                 [0.0, 0.0, (near + far) * range_inv, -1.0],
//                 [0.0, 0.0, near * far * range_inv * 2.0, 0.0],
//             ],
//         }
//     }
//     pub fn print(&self) {
//         for row in &self.data {
//             for elem in row {
//                 print!("{:.2} ", elem); // Adjust precision as needed
//             }
//             println!(); // Move to the next row
//         }
//     }
// }

// impl Mul<Mat4> for Mat4 {
//     type Output = Mat4;

//     fn mul(self, rhs: Mat4) -> Mat4 {
//         Mat4::multiply(&self, &rhs)
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Mat4 {
    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let mut result = Mat4::identity();

        for i in 0..4 {
            for j in 0..4 {
                result.0[i][j] = self.0[i][0] * other.0[0][j]
                                + self.0[i][1] * other.0[1][j]
                                + self.0[i][2] * other.0[2][j]
                                + self.0[i][3] * other.0[3][j];
            }
        }

        result
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        let mut result = self.clone();
        result.0[0][3] += x;
        result.0[1][3] += y;
        result.0[2][3] += z;
        result
    }

    pub fn rotate_x(&self, angle: f32) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let mut result = self.clone();
        result *= Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_angle, -sin_angle, 0.0],
            [0.0, sin_angle, cos_angle, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        result
    }

    pub fn rotate_y(&self, angle: f32) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let mut result = self.clone();
        result *= Self([
            [cos_angle, 0.0, sin_angle, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin_angle, 0.0, cos_angle, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        result
    }

    pub fn rotate_z(&self, angle: f32) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let mut result = self.clone();
        result *= Self([
            [cos_angle, -sin_angle, 0.0, 0.0],
            [sin_angle, cos_angle, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        result
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        let mut result = self.clone();
        result.0[0][0] *= x;
        result.0[1][1] *= y;
        result.0[2][2] *= z;
        result
    }

    pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();
        let range_inv = 1.0 / (near - far);

        Self([
            [f / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (far + near) * range_inv, -1.0],
            [0.0, 0.0, 2.0 * far * near * range_inv, 0.0],
        ])
    }
    pub fn as_ptr(&self) -> *const f32 {
        self.0.as_ptr() as *const f32
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.multiply(&rhs)
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.multiply(&rhs);
    }
}