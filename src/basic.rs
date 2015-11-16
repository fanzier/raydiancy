pub use lin_alg::*;
pub use color::*;


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
    /// assert_eq!(Ray::new(Vec3::zero(), 42.0*Vec3::e1()), Ray::new(Vec3::zero(), Vec3::e1()));
    /// ```
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin: origin, dir: dir.normalize() }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    /// Color of the material.
    pub color: Color,
    /// Ambient reflection constant.
    pub ambient: f64,
    /// Diffuse reflection constant.
    pub diffuse: f64,
    /// Specular reflection constant.
    pub specular: f64,
    /// Shininess/specular exponent.
    /// When it is large, the specular highlight is small.
    /// It is larger for smoother and mirror-like surfaces.
    pub shininess: f64,
    /// Mirror reflectance. 0 means no reflection, 1 means perfect mirror.
    pub reflectance: f64,
    /// Refractivity. 0 means no refraction, 1 means only recfraction.
    pub refractivity: f64,
    /// Refraction index. 1 is vacuum.
    pub refraction_index: f64,
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
