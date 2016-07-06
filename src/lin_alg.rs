use std::ops;
use std::cmp;
use std::marker::PhantomData;
pub use std::f64::consts::PI;

/// The smallest difference for values to be considered equal.
/// This should account for numerical instabilities.
pub const EPS: f64 = 0.0001;

/// Tests two f64 values for approximate equality (up to `EPS`)
///
/// This is done to account for rounding errors.
/// # Examples
/// ```
/// use raydiancy::lin_alg::*;
/// assert!(appr(0.0, EPS / 2.0));
/// assert!(!appr(0.0,1.0));
/// ```
pub fn appr(f: f64, g: f64) -> bool {
    if (f - g).abs() < EPS {
        true
    } else {
        false
    }
}

/// Compares two f64 values for approximate value (up to `EPS`)
///
/// This is done to account for rounding errors.
/// # Examples
/// ```
/// use raydiancy::lin_alg::*;
/// use std::cmp;
/// assert_eq!(appr_cmp(0.0, EPS / 2.0), cmp::Ordering::Equal);
/// assert_eq!(appr_cmp(0.0,1.0), cmp::Ordering::Less);
/// ```
pub fn appr_cmp(f: f64, g: f64) -> cmp::Ordering {
    let diff = f - g;
    if diff < -EPS {
        cmp::Ordering::Less
    } else if diff > EPS {
        cmp::Ordering::Greater
    } else {
        cmp::Ordering::Equal
    }
}

/// Represents a three-dimensional vector with a type marker `Marker`.
/// Note that most of the time, you want `Vec3` instead. (Exception: function parameters)
///
/// This marker is used to prove that certain vectors are normalized.
/// If you write a function that accepts a vector, it should accept a `Vec3M<M>`.
/// If the function returns a vector, it should be `UnitVec3`
/// if it's guaranteed to be a unit vector, and `Vec3` otherwise.
/// **Accept the most general type and return the most specialized type.**
///
/// You can always convert any vector into a `Vec3`, using `Vec3M<M>::to`.
#[derive(Debug, Clone)]
pub struct Vec3M<Marker: Clone> {
    x: [f64; 3],
    phantom: PhantomData<Marker>,
}

impl<M: Clone> Copy for Vec3M<M> {}

#[derive(Debug, Copy, Clone)]
struct VecMarker;

#[derive(Debug, Copy, Clone)]
struct UnitMarker;

/// Represents a three-dimensional vector.
pub type Vec3 = Vec3M<VecMarker>;

/// Represents a three-dimensional unit vector, i.e. its `norm()` is guaranteed to be 1.0.
pub type UnitVec3 = Vec3M<UnitMarker>;

impl ops::Neg for UnitVec3 {
    type Output = UnitVec3;

    fn neg(self) -> UnitVec3 {
        UnitVec3 {
            x: [-self.x[0], -self.x[1], -self.x[2]],
            phantom: PhantomData,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        (-1.0) * self
    }
}

impl<M, N> ops::Add<Vec3M<N>> for Vec3M<M>
    where M: Clone,
          N: Clone
{
    type Output = Vec3;

    fn add(self, b: Vec3M<N>) -> Vec3 {
        Vec3::new(self[0] + b[0], self[1] + b[1], self[2] + b[2])
    }
}

impl<M, N> ops::Sub<Vec3M<N>> for Vec3M<M>
    where M: Clone,
          N: Clone
{
    type Output = Vec3;

    fn sub(self, b: Vec3M<N>) -> Vec3 {
        Vec3::new(self[0] - b[0], self[1] - b[1], self[2] - b[2])
    }
}

impl<M> ops::Mul<Vec3M<M>> for f64
    where M: Clone
{
    type Output = Vec3;

    fn mul(self: f64, v: Vec3M<M>) -> Vec3 {
        Vec3::new(self * v[0], self * v[1], self * v[2])
    }
}

impl<M, N> ops::Mul<Vec3M<N>> for Vec3M<M>
    where M: Clone,
          N: Clone
{
    type Output = f64;

    fn mul(self: Vec3M<M>, v: Vec3M<N>) -> f64 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }
}

impl<M> ops::Div<f64> for Vec3M<M>
    where M: Clone
{
    type Output = Vec3;

    fn div(self, s: f64) -> Vec3 {
        let f = 1.0 / s;
        f * self
    }
}

impl<M> ops::Index<usize> for Vec3M<M>
    where M: Clone
{
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.x[idx]
    }
}

/// Compares vectors up to `EPS` (to take rounding errors into account).
/// Note that this relation is *not transitive*.
/// But it is still very useful because this property is often not needed.
impl<M, N> cmp::PartialEq<Vec3M<N>> for Vec3M<M>
    where M: Clone,
          N: Clone
{
    fn eq(&self, v: &Vec3M<N>) -> bool {
        for i in 0..3 {
            if !appr(self.x[i], v.x[i]) {
                return false;
            }
        }
        true
    }
}

/// Orders vectors up to `EPS` (to take rounding errors into account).
/// Note that this relation is *not transitive*.
/// But it is still very useful because this property is often not needed.
impl<M, N> cmp::PartialOrd<Vec3M<N>> for Vec3M<M>
    where M: Clone,
          N: Clone
{
    fn partial_cmp(&self, v: &Vec3M<N>) -> Option<cmp::Ordering> {
        let cmps = [appr_cmp(self[0], v[0]), appr_cmp(self[1], v[1]), appr_cmp(self[2], v[2])];
        if cmps.iter().all(|&x| x == cmp::Ordering::Equal) {
            Some(cmp::Ordering::Equal)
        } else if cmps.iter().all(|&x| x != cmp::Ordering::Greater) {
            Some(cmp::Ordering::Less)
        } else if cmps.iter().all(|&x| x != cmp::Ordering::Less) {
            Some(cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

impl<M: Clone> Vec3M<M> {
    /// Creates a vector with the given coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: [x, y, z],
            phantom: PhantomData,
        }
    }

    /// Creates a vector with coordinates given in the array of length 3.
    pub fn from_array(arr: [f64; 3]) -> Vec3 {
        Vec3 {
            x: arr,
            phantom: PhantomData,
        }
    }

    /// Converts a (possibly) polymorphic vector to a `Vec3`.
    pub fn to(self) -> Vec3 {
        Vec3::from_array(self.x)
    }

    /// Creates a `UnitVec3` out of any given vector. Panics if its norm is not 1.0.
    pub fn assert_unit_vector(self: Vec3M<M>) -> UnitVec3 {
        assert!(appr(self.norm2(), 1.));
        UnitVec3 {
            x: self.x,
            phantom: PhantomData,
        }
    }

    /// Returns the zero vector.
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    /// Returns a vector filled with ones.
    pub fn ones() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }

    /// Returns the unit vector in positive x-direction.
    pub fn e1() -> UnitVec3 {
        Vec3::new(1.0, 0.0, 0.0).assert_unit_vector()
    }

    /// Returns the unit vector in positive y-direction.
    pub fn e2() -> UnitVec3 {
        Vec3::new(0.0, 1.0, 0.0).assert_unit_vector()
    }

    /// Returns the unit vector in positive z-direction.
    pub fn e3() -> UnitVec3 {
        Vec3::new(0.0, 0.0, 1.0).assert_unit_vector()
    }

    /// Returns the unit vector in positive i-th direction
    pub fn e(i: usize) -> UnitVec3 {
        let mut arr = [0.0, 0.0, 0.0];
        arr[i] = 1.0;
        Vec3::from_array(arr).assert_unit_vector()
    }

    /// Returns the x-coordinate.
    pub fn x(self) -> f64 {
        self[0]
    }

    /// Returns the y-coordinate.
    pub fn y(self) -> f64 {
        self[1]
    }

    /// Returns the z-coordinate.
    pub fn z(self) -> f64 {
        self[2]
    }

    /// Computes the norm of the vector.
    pub fn norm(self) -> f64 {
        f64::sqrt(self * self)
    }

    /// Computes the square of the norm (saves a square root operation compared to `norm()`).
    pub fn norm2(self) -> f64 {
        self * self
    }

    /// Returns the unit vector pointing in the same direction.
    pub fn normalize(self) -> UnitVec3 {
        let n = self / self.norm();
        UnitVec3 {
            x: n.x,
            phantom: PhantomData,
        }
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
    pub fn cross<N>(self, v: Vec3M<N>) -> Vec3
        where M: Clone,
              N: Clone
    {
        Vec3::new(self.x[1] * v.x[2] - self.x[2] * v.x[1],
                  self.x[2] * v.x[0] - self.x[0] * v.x[2],
                  self.x[0] * v.x[1] - self.x[1] * v.x[0])
    }

    /// Returns the coordinate-wise maximum of the two vectors.
    pub fn max<N>(self, v: Vec3M<N>) -> Vec3
        where M: Clone,
              N: Clone
    {
        Vec3::new(self[0].max(v[0]), self[1].max(v[1]), self[2].max(v[2]))
    }

    /// Returns the coordinate-wise minimum of the two vectors.
    pub fn min<N>(self, v: Vec3M<N>) -> Vec3
        where M: Clone,
              N: Clone
    {
        Vec3::new(self[0].min(v[0]), self[1].min(v[1]), self[2].min(v[2]))
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
    m: [[f64; 4]; 3],
}

impl ops::Mul<Matrix34> for Matrix34 {
    type Output = Matrix34;

    fn mul(self, b: Matrix34) -> Matrix34 {
        let mut res: [[f64; 4]; 3] = [[0.0; 4]; 3];
        for i in 0..3 {
            for k in 0..4 {
                let mut entry: f64 = 0.0;
                for j in 0..3 {
                    entry += self.m[i][j] * b.m[j][k]
                }
                res[i][k] = entry +
                            if k == 3 {
                    self.m[i][3]
                } else {
                    0.0
                }
            }
        }
        Matrix34 { m: res }
    }
}

impl<M> ops::Mul<Vec3M<M>> for Matrix34
    where M: Clone
{
    type Output = Vec3;

    fn mul(self, v: Vec3M<M>) -> Vec3 {
        let mut res: [f64; 3] = [0.0; 3];
        for i in 0..3 {
            let mut entry: f64 = 0.0;
            for j in 0..3 {
                entry += self.m[i][j] * v[j]
            }
            entry += self.m[i][3];
            res[i] = entry
        }
        Vec3::from_array(res)
    }
}

impl cmp::PartialEq for Matrix34 {
    fn eq(&self, b: &Matrix34) -> bool {
        for i in 0..3 {
            for j in 0..4 {
                if !appr(self.m[i][j], b.m[i][j]) {
                    return false;
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
        Matrix34 { m: [[v[0], 0.0, 0.0, 0.0], [0.0, v[1], 0.0, 0.0], [0.0, 0.0, v[2], 0.0]] }
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
        Matrix34 { m: [[1.0, 0.0, 0.0, v[0]], [0.0, 1.0, 0.0, v[1]], [0.0, 0.0, 1.0, v[2]]] }
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
        Matrix34 {
            m: [[angle.cos() + u[0] * u[0] * (1.0 - angle.cos()),
                 u[0] * u[1] * (1.0 - angle.cos()) - u[2] * angle.sin(),
                 u[0] * u[2] * (1.0 - angle.cos()) + u[1] * angle.sin(),
                 0.0],
                [u[1] * u[0] * (1.0 - angle.cos()) + u[2] * angle.sin(),
                 angle.cos() + u[1] * u[1] * (1.0 - angle.cos()),
                 u[1] * u[2] * (1.0 - angle.cos()) - u[0] * angle.sin(),
                 0.0],
                [u[2] * u[0] * (1.0 - angle.cos()) - u[1] * angle.sin(),
                 u[2] * u[1] * (1.0 - angle.cos()) + u[0] * angle.sin(),
                 angle.cos() + u[2] * u[2] * (1.0 - angle.cos()),
                 0.0]],
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
        let mut res: [[f64; 4]; 3] = [[0.0; 4]; 3];
        for i in 0..3 {
            for j in 0..3 {
                res[i][j] = self.m[j][i]
            }
        }
        for i in 0..3 {
            res[i][3] = self.m[i][3]
        }
        Matrix34 { m: res }
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
        self.m[0][0] * self.m[1][1] * self.m[2][2] + self.m[0][1] * self.m[1][2] * self.m[2][0] +
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
    /// assert_eq!(Matrix34::scale(Vec3::new(1.0,2.0,4.0)).invert(),
    ///     Matrix34::scale(Vec3::new(1.0,0.5,0.25)));
    /// let v = Vec3::new(1.0,2.0,3.0);
    /// let angle = 0.5;
    /// assert_eq!(Matrix34::rotate(v, angle).invert(), Matrix34::rotate(v, -angle));
    /// assert_eq!(Matrix34::translate(v).invert(), Matrix34::translate(-v));
    /// let w = Vec3::new(1.0,0.5,1.0/3.0);
    /// assert_eq!((Matrix34::translate(v) * Matrix34::scale(v)).invert(),
    ///     Matrix34::scale(w) * Matrix34::translate(-v));
    /// ```
    pub fn invert(&self) -> Matrix34 {
        let d = self.determinant();
        let ref a = self.m;
        let mut b = [[(a[1][1] * a[2][2] - a[1][2] * a[2][1]) / d,
                      (a[0][2] * a[2][1] - a[2][2] * a[0][1]) / d,
                      (a[0][1] * a[1][2] - a[1][1] * a[0][2]) / d,
                      0.0],
                     [(a[1][2] * a[2][0] - a[2][2] * a[1][0]) / d,
                      (a[0][0] * a[2][2] - a[2][0] * a[0][2]) / d,
                      (a[0][2] * a[1][0] - a[1][2] * a[0][0]) / d,
                      0.0],
                     [(a[1][0] * a[2][1] - a[2][0] * a[1][1]) / d,
                      (a[0][1] * a[2][0] - a[2][1] * a[0][0]) / d,
                      (a[0][0] * a[1][1] - a[1][0] * a[0][1]) / d,
                      0.0]];
        b[0][3] = -(b[0][0] * a[0][3] + b[0][1] * a[1][3] + b[0][2] * a[2][3]);
        b[1][3] = -(b[1][0] * a[0][3] + b[1][1] * a[1][3] + b[1][2] * a[2][3]);
        b[2][3] = -(b[2][0] * a[0][3] + b[2][1] * a[1][3] + b[2][2] * a[2][3]);
        Matrix34 { m: b }
    }
}
