pub use lin_alg::*;
pub use color::*;
pub use material::*;


/// Structure for representing rays
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    /// Where the ray comes from.
    pub origin: Vec3,
    /// The direction the ray is traveling. Must be normalized!
    pub dir: Vec3,
}

impl Ray {
    /// Creates a `Ray`.
    ///
    /// It normalizes the direction vector.
    /// # Example
    /// ```
    /// use raydiancy::raytrace::*;
    /// assert_eq!(Ray::new(Vec3::zero(), 42.0*Vec3::e1()),
    ///            Ray::new(Vec3::zero(), Vec3::e1()));
    /// ```
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin: origin, dir: dir.normalize() }
    }
}

/// Contains information about the intersection of a ray and an object.
pub struct Intersection {
    /// The intersection point is: (origin of ray) + t * (direction of ray).
    pub t: f64,
    /// The normal vector at the intersection point (orthogonal to the surface).
    pub normal: Vec3,
    /// The material properties at the intersection point.
    pub material: Material
}

/// Trait for finding ray intersections.
pub trait Surface {
    /// Returns information about the intersection of the object and the ray, if one exists.
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
    /// Checks whether the ray intersects the object, computes no additional information.
    /// If the offset is greater than `t_max`, it returns false.
    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool;
}
