
use std::ops::Mul;
use std::ops::MulAssign;

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

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Mat4 {
        let mut result = self.clone();
        result.0[3][0] += x;
        result.0[3][1] += y;
        result.0[3][2] += z;
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