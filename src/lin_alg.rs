use std::ops;
use std::cmp;
pub use std::f64::consts::PI;

/// The smallest difference for values to be considered equal.
/// This should account for numerical instabilities.
pub const EPS: f64 = 0.0001;

/// Compares two f64 values for approximate value (up to `EPS`)
///
/// This is done to account for rounding errors.
/// # Examples
/// ```
/// use raydiancy::lin_alg::*;
/// assert!(appr(0.0, EPS / 2.0));
/// assert!(!appr(0.0,1.0));
/// ```
pub fn appr(f: f64, g: f64) -> bool {
    if (f - g).abs() < EPS { true } else { false }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x:[f64; 3]
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        (-1.0) * self
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, b: Vec3) -> Vec3 {
        Vec3 { x: [self[0] + b[0], self[1] + b[1], self[2] + b[2]]}
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, b: Vec3) -> Vec3 {
        Vec3 { x: [self[0] - b[0], self[1] - b[1], self[2] - b[2]]}
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self: f64, v: Vec3) -> Vec3 {
        Vec3 { x: [self * v[0], self * v[1], self * v[2]]}
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self: Vec3, v: Vec3) -> f64 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, s: f64) -> Vec3 {
        let f = 1.0 / s;
        f * self
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.x[idx]
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, v: &Vec3) -> bool {
        for i in 0..3 {
            if !appr(self.x[i], v.x[i]) {
                return false
            }
        }
        true
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: [x,y,z] }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    pub fn ones() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }
    pub fn e1() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
    pub fn e2() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }
    pub fn e3() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }

    pub fn norm(self) -> f64 {
        f64::sqrt(self * self)
    }

    pub fn norm2(self) -> f64 {
        self * self
    }

    pub fn normalize(self) -> Vec3 {
        self / self.norm()
    }

    /// Computes the cross product of two vectors
    ///
    /// # Examples
    /// ```
    /// use raydiancy::lin_alg::*;
    /// assert_eq!(Vec3::e1().cross(Vec3::e2()), Vec3::e3());
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// let w = Vec3::new(4.0, 5.0, 6.0);
    /// let c = v.cross(w);
    /// assert_eq!(v * c, 0.0);
    /// assert_eq!(w * c, 0.0);
    /// ```
    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x[1] * v.x[2] - self.x[2] * v.x[1],
            self.x[2] * v.x[0] - self.x[0] * v.x[2],
            self.x[0] * v.x[1] - self.x[1] * v.x[0])
    }
}

/// Matrices (4x4) of the shape
/// <pre>
/// ( A b )  * ( x )
/// ( 0 1 )  * ( 1 )
/// </pre>
/// representing the affine transformation <tt>x -> Ax + b</tt>.
/// Only the <tt>( A b )</tt> part (3x4) of the matrix is stored.
#[derive(Debug, Copy, Clone)]
pub struct Matrix34 {
    m: [[f64;4];3]
}

impl ops::Mul<Matrix34> for Matrix34 {
    type Output = Matrix34;

    fn mul(self, b: Matrix34) -> Matrix34 {
        let mut res: [[f64;4];3] = [[0.0; 4]; 3];
        for i in 0..3 {
            for k in 0..4 {
                let mut entry: f64 = 0.0;
                for j in 0..3 {
                    entry += self.m[i][j] * b.m[j][k]
                }
                res[i][k] = entry + if k == 3 { self.m[i][3] } else { 0.0 }
            }
        }
        Matrix34 { m: res }
    }
}

impl ops::Mul<Vec3> for Matrix34 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        let mut res: [f64;3] = [0.0; 3];
        for i in 0..3 {
            let mut entry: f64 = 0.0;
            for j in 0..3 {
                entry += self.m[i][j] * v[j]
            }
            entry += self.m[i][3];
            res[i] = entry
        }
        Vec3 { x: res }
    }
}

impl cmp::PartialEq for Matrix34 {
    fn eq(&self, b: &Matrix34) -> bool {
        for i in 0..3 {
            for j in 0..4 {
                if !appr(self.m[i][j],b.m[i][j]) {
                    return false
                }
            }
        }
        true
    }
}

impl Matrix34 {
    /// Creates a scaling matrix
    ///
    /// # Examples
    /// ```
    /// use raydiancy::lin_alg::*;
    /// assert_eq!(Matrix34::scale(Vec3::new(1.0,1.0,1.0)), Matrix34::identity());
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// assert_eq!(Matrix34::scale(v) * v, Vec3::new(1.0,4.0,9.0));
    /// ```
    pub fn scale(v: Vec3) -> Matrix34 {
        Matrix34 {
            m: [[v[0], 0.0, 0.0, 0.0], [0.0, v[1], 0.0, 0.0], [0.0, 0.0, v[2], 0.0]]
        }
    }

    /// Creates a translation matrix
    ///
    /// # Examples
    /// ```
    /// use raydiancy::lin_alg::*;
    /// assert_eq!(Matrix34::translate(Vec3::new(0.0,0.0,0.0)), Matrix34::identity());
    /// let e1 = Vec3::new(1.0, 0.0, 0.0);
    /// let translate_along_e3 = Matrix34::translate(Vec3::new(0.0,0.0,1.0));
    /// assert_eq!(translate_along_e3 * e1, Vec3::new(1.0,0.0,1.0));
    /// ```
    pub fn translate(v: Vec3) -> Matrix34 {
        Matrix34 {
            m: [[1.0, 0.0, 0.0, v[0]], [0.0, 1.0, 0.0, v[1]], [0.0, 0.0, 1.0, v[2]]]
        }
    }

    /// Creates a rotation matrix
    ///
    /// # Examples
    /// ```
    /// use raydiancy::lin_alg::*;
    /// assert_eq!(Matrix34::rotate(Vec3::new(1.0,0.0,0.0), 0.0), Matrix34::identity());
    /// let e1 = Vec3::new(1.0, 0.0, 0.0);
    /// let rotate_around_e3 = Matrix34::rotate(Vec3::new(0.0,0.0,1.0), PI / 2.0);
    /// assert_eq!(rotate_around_e3 * e1, Vec3::new(0.0,1.0,0.0));
    /// ```
    pub fn rotate(axis: Vec3, angle: f64) -> Matrix34 {
        let u = axis.normalize();
        Matrix34{
            m: [
                  [ angle.cos()+u[0]*u[0]*(1.0 - angle.cos()),
                    u[0]*u[1]*(1.0-angle.cos()) - u[2]*angle.sin(),
                    u[0]*u[2]*(1.0-angle.cos()) + u[1]*angle.sin(),
                    0.0
                  ],
                  [ u[1]*u[0]*(1.0-angle.cos()) + u[2]*angle.sin(),
                    angle.cos()+u[1]*u[1]*(1.0 - angle.cos()),
                    u[1]*u[2]*(1.0-angle.cos()) - u[0]*angle.sin(),
                    0.0
                  ],
                  [ u[2]*u[0]*(1.0-angle.cos()) - u[1]*angle.sin(),
                    u[2]*u[1]*(1.0-angle.cos()) + u[0]*angle.sin(),
                    angle.cos()+u[2]*u[2]*(1.0 - angle.cos()),
                    0.0
                  ]
                ]
        }
    }

    /// Transposes 3x3 part of matrix and leaves last column unchanged
    ///
    /// # Examples
    /// ```
    /// use raydiancy::lin_alg::*;
    /// assert_eq!(Matrix34::identity().transpose(), Matrix34::identity());
    /// {
    ///     let m = Matrix34::rotate(Vec3::new(1.0,2.0,3.0),1.0);
    ///     assert_eq!(m.transpose().transpose(), m);
    /// }
    /// {
    ///     let m = Matrix34::translate(Vec3::new(1.0,2.0,3.0));
    ///     assert_eq!(m.transpose(), m);
    /// }
    /// ```
    pub fn transpose(&self) -> Matrix34 {
        let mut res: [[f64;4];3] = [[0.0;4];3];
        for i in 0..3 {
            for j in 0..3 {
                res[i][j] = self.m[j][i]
            }
        }
        for i in 0..3 {
            res[i][3] = self.m[i][3]
        }
        Matrix34{ m: res }
    }

    /// Identity matrix
    pub fn identity() -> Matrix34 {
        Matrix34::scale(Vec3::new(1.0, 1.0, 1.0))
    }

    /// Computes the determinant of the 3x3 part of the matrix
    ///
    /// # Examples
    /// ```
    /// use raydiancy::lin_alg::*;
    /// assert_eq!(Matrix34::identity().determinant(), 1.0);
    /// assert_eq!(Matrix34::scale(Vec3::new(1.0,2.0,3.0)).determinant(), 6.0);
    /// assert!(appr(Matrix34::rotate(Vec3::new(1.0,2.0,3.0),1.0).determinant(), 1.0));
    /// ```
    pub fn determinant(&self) -> f64 {
        self.m[0][0] * self.m[1][1] * self.m[2][2] +
        self.m[0][1] * self.m[1][2] * self.m[2][0] +
        self.m[0][2] * self.m[1][0] * self.m[2][1] -
        self.m[0][0] * self.m[1][2] * self.m[2][1] -
        self.m[0][1] * self.m[1][0] * self.m[2][2] -
        self.m[0][2] * self.m[1][1] * self.m[2][0]
    }

    /// Inverts the matrix as a 4x4 matrix
    ///
    /// # Examples
    /// ```
    /// use raydiancy::lin_alg::*;
    /// assert_eq!(Matrix34::identity().invert(), Matrix34::identity());
    /// assert_eq!(Matrix34::scale(Vec3::new(1.0,2.0,4.0)).invert(), Matrix34::scale(Vec3::new(1.0,0.5,0.25)));
    /// let v = Vec3::new(1.0,2.0,3.0);
    /// let angle = 0.5;
    /// assert_eq!(Matrix34::rotate(v, angle).invert(), Matrix34::rotate(v, -angle));
    /// assert_eq!(Matrix34::translate(v).invert(), Matrix34::translate(-v));
    /// let w = Vec3::new(1.0,0.5,1.0/3.0);
    /// assert_eq!((Matrix34::translate(v) * Matrix34::scale(v)).invert(), Matrix34::scale(w) * Matrix34::translate(-v));
    /// ```
    pub fn invert(&self) -> Matrix34 {
        let d = self.determinant();
        let ref a = self.m;
        let mut b =
            [
                [
                    (a[1][1] * a[2][2] - a[1][2] * a[2][1]) / d,
                    (a[0][2] * a[2][1] - a[2][2] * a[0][1]) / d,
                    (a[0][1] * a[1][2] - a[1][1] * a[0][2]) / d,
                    0.0
                ],
                [
                    (a[1][2] * a[2][0] - a[2][2] * a[1][0]) / d,
                    (a[0][0] * a[2][2] - a[2][0] * a[0][2]) / d,
                    (a[0][2] * a[1][0] - a[1][2] * a[0][0]) / d,
                    0.0
                ],
                [
                    (a[1][0] * a[2][1] - a[2][0] * a[1][1]) / d,
                    (a[0][1] * a[2][0] - a[2][1] * a[0][0]) / d,
                    (a[0][0] * a[1][1] - a[1][0] * a[0][1]) / d,
                    0.0
                ]
            ];
        b[0][3] = - (b[0][0] * a[0][3] + b[0][1] * a[1][3] + b[0][2] * a[2][3]);
        b[1][3] = - (b[1][0] * a[0][3] + b[1][1] * a[1][3] + b[1][2] * a[2][3]);
        b[2][3] = - (b[2][0] * a[0][3] + b[2][1] * a[1][3] + b[2][2] * a[2][3]);
        Matrix34 { m: b }
    }
}
