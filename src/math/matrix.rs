use std::ops::{Mul, Index, IndexMut};
use math::vector::Vector3;

#[derive(Clone, Debug)]
pub struct Matrix {
    elements: [[f32; 4]; 4],
}
impl Matrix {
    pub fn identity() -> Matrix {
        Matrix {
            elements: [[1.0, 0.0, 0.0, 0.0],
                       [0.0, 1.0, 0.0, 0.0],
                       [0.0, 0.0, 1.0, 0.0],
                       [0.0, 0.0, 0.0, 1.0]]
        }
    }

    pub fn scale_linear(s: f32) -> Matrix {
        Matrix::scale(s, s, s)
    }

    pub fn scale(sx: f32, sy: f32, sz: f32) -> Matrix {
        Matrix {
            elements: [[ sx, 0.0, 0.0, 0.0],
                       [0.0,  sy, 0.0, 0.0],
                       [0.0, 0.0,  sz, 0.0],
                       [0.0, 0.0, 0.0, 1.0]]
        }
    }

    pub fn rotate_x(t: f32) -> Matrix {
        let sin = t.sin();
        let cos = t.cos();
        Matrix {
            elements: [[1.0, 0.0, 0.0, 0.0],
                       [0.0, cos, sin, 0.0],
                       [0.0,-sin, cos, 0.0],
                       [0.0, 0.0, 0.0, 1.0]],
        }
    }

    pub fn rotate_y(t: f32) -> Matrix {
        let sin = t.sin();
        let cos = t.cos();
        Matrix {
            elements: [[cos, 0.0, -sin, 0.0],
                       [0.0, 1.0, 0.0, 0.0],
                       [sin, 0.0, cos, 0.0],
                       [0.0, 0.0, 0.0, 1.0]],
        }
    }

    pub fn rotate_z(t: f32) -> Matrix {
        let sin = t.sin();
        let cos = t.cos();
        Matrix {
            elements: [[cos, sin, 0.0, 0.0],
                       [-sin, cos, 0.0, 0.0],
                       [0.0, 0.0, 1.0, 0.0],
                       [0.0, 0.0, 0.0, 1.0]],
        }
    }

    pub fn translate(tx: f32, ty:f32, tz: f32) -> Matrix {
        Matrix {
            elements: [[0.0, 0.0, 0.0,  tx],
                       [0.0, 0.0, 0.0,  ty],
                       [0.0, 0.0, 0.0,  tz],
                       [0.0, 0.0, 0.0, 0.0]],
        }
    }

    pub fn inverse(&self) -> Matrix {
        let mut s = Matrix::identity();
        let mut t = self.clone();
        // Forward elimination
        for i in 0..3 {
            let mut pivot = i;
            let mut pivot_size = t[i][i].abs();
            for j in (i + 1)..4 {
                let tmp = t[j][i].abs();
                if tmp > pivot_size {
                    pivot = j;
                    pivot_size = tmp;
                }
            }

            if pivot_size == 0.0 {
                return Matrix::identity();
            }
            if pivot != i {
                for j in 0..4 {
                    let mut tmp: f32;

                    tmp = t[i][j];
                    t[i][j] = t[pivot][j];
                    t[pivot][j] = tmp;

                    tmp = s[i][j];
                    s[i][j] = s[pivot][j];
                    s[pivot][j] = tmp;
                }
            }
            for j in (i + 1)..4 {
                let f = t[j][i] / t[i][i];

                for k in 0..4 {
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }
        // Backward substitution
        for i in (0..4).rev() {
            let mut f: f32 = t[i][i];

            if f == 0.0 {
                // Cannot invert singular matrix
                return Matrix::identity();
            }

            for j in 0..4 {
                t[i][j] /= f;
                s[i][j] /= f;
            }

            for j in 0..i {
                f = t[j][i];

                for k in 0..4 {
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }

        s
    }
}
impl Index<usize> for Matrix {
    type Output = [f32; 4];

    fn index(&self, idx: usize) -> &[f32; 4] {
        &self.elements[idx]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, idx: usize) -> &mut [f32; 4] {
        &mut self.elements[idx]
    }
}
impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        let mut result = Matrix::identity();
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self[i][0] * other[0][j] +
                               self[i][1] * other[1][j] +
                               self[i][2] * other[2][j] +
                               self[i][3] * other[3][j];
            }
        }
        result
    }
}

impl Mul<Vector3> for Matrix {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: other.x * self[0][0] + other.y * self[1][0] + other.z * self[2][0],
            y: other.x * self[0][1] + other.y * self[1][1] + other.z * self[2][1],
            z: other.x * self[0][2] + other.y * self[1][2] + other.z * self[2][2],
        }
    }
}
