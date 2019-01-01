use std::arch::x86_64::*;
use std::ops::{Mul, Index, IndexMut};
use math::vector::*;

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

    pub fn from_vector(v0: Vector4, v1: Vector4, v2: Vector4, v3: Vector4) -> Matrix {
        Matrix {
            elements: [[v0.x, v0.y, v0.z, v0.w],
                       [v1.x, v1.y, v1.z, v1.w],
                       [v2.x, v2.y, v2.z, v2.w],
                       [v3.x, v3.y, v3.z, v3.w]]
        }        
    }

    pub fn scale_uniform(s: f32) -> Matrix {
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
                       [0.0, 0.0, 0.0, 1.0]],
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
impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        for x in 0..4 {
            for y in 0..4 {
                if self[x][y] != other[x][y] {
                    return false;
                }
            }
        }

        true
    }
}
impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        let mut result = Matrix::identity();
        
        if is_x86_feature_detected!("avx2")
        {
            unsafe {
                for i in 0..4 {
                    let mut r = _mm_mul_ps(_mm_broadcast_ss(&self[i][0]), _mm_loadu_ps(&other[0] as *const f32));
                    r = _mm_add_ps(r, _mm_mul_ps(_mm_broadcast_ss(&self[i][1]), _mm_loadu_ps(&other[1] as *const f32)));
                    r = _mm_add_ps(r, _mm_mul_ps(_mm_broadcast_ss(&self[i][2]), _mm_loadu_ps(&other[2] as *const f32)));
                    r = _mm_add_ps(r, _mm_mul_ps(_mm_broadcast_ss(&self[i][3]), _mm_loadu_ps(&other[3] as *const f32)));

                    _mm_storeu_ps(&mut result[i][0] as *mut f32, r);
                }
            }
        }
        else {
            for i in 0..4 {
                for j in 0..4 {
                    result[i][j] = self[i][0] * other[0][j] +
                                self[i][1] * other[1][j] +
                                self[i][2] * other[2][j] +
                                self[i][3] * other[3][j];
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let m1 = Matrix {
            elements: [[0.0, 1.0, 0.0, 0.0],
                       [1.0, 0.0, 2.0, 0.0],
                       [2.0, 0.0, 1.0, 0.0],
                       [0.0, 0.0, 0.0, 1.0]]
        };
        let m2 = m1.inverse();
        let m3 = m1 * m2;
        println!("{:?}", m3);
        assert_eq!(m3, Matrix::identity());
    }
}