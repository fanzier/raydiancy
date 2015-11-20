pub use lin_alg::*;
pub use color::*;
pub use material::*;
use physics::*;


/// Structure for representing rays
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    /// Where the ray comes from.
    pub origin: Vec3,
    /// The direction the ray is traveling.
    pub dir: UnitVec3,
}

impl Ray {
    /// Creates a `Ray`.
    pub fn new(origin: Vec3, dir: UnitVec3) -> Ray {
        Ray { origin: origin, dir: dir }
    }

    /// Creates a `Ray` and normalizes the given direction.
    pub fn newn(origin: Vec3, dir: Vec3) -> Ray {
        Ray::new(origin, dir.normalize())
    }
}

/// Given a ray intersection and the direction of the light source, computes the shadow ray.
pub fn shadow_ray(inter: &Intersection, dir: UnitVec3) -> Ray {
    let point = inter.point + EPS * inter.normal;
    Ray::new(point, dir)
}

/// Given a ray intersection and the direction of a ray, computes its reflection ray.
pub fn reflect_ray(inter: &Intersection, dir: UnitVec3) -> Ray {
    let reflected_dir = reflect(dir, inter.normal);
    let point = inter.point + EPS * inter.normal;
    Ray::new(point, reflected_dir)
}

/// Computes the refracted ray or `None` in case of total internal reflection.
///
/// Inputs are a ray intersection, the direction of the ray,
/// and the index of refraction (ior of material after intersection / ior before intersection).
pub fn refract_ray(inter: &Intersection, dir: UnitVec3, ior: f64) -> Option<Ray> {
    let maybe_refracted_dir = refract(dir, inter.normal, ior);
    let point = inter.point - EPS * inter.normal;
    maybe_refracted_dir.map(|dir| Ray::new(point, dir))
}

/// Contains information about the intersection of a ray and an object.
pub struct Intersection {
    /// Distance of the hit point from the origin of the ray.
    pub t: f64,
    /// The intersection point. It is: (origin of ray) + t * (direction of ray).
    pub point: Vec3,
    /// The normal vector at the intersection point (orthogonal to the surface).
    pub normal: UnitVec3,
    /// The material properties at the intersection point.
    pub material: Material
}

impl Intersection {
    /// Constructs an intersection.
    ///
    /// `ray` is the ray that hits the object,
    /// `t` is the distance of the intersection point from `ray.origin`,
    /// `normal` is the normal vector at the intersection point,
    /// `material` is the material at the intersection point.
    pub fn new(ray: Ray, t: f64, normal: UnitVec3, material: Material) -> Intersection {
        Intersection {
            t: t,
            point: ray.origin + t * ray.dir,
            normal: normal,
            material: material
        }
    }
}

/// Trait for finding ray intersections.
pub trait Surface {
    /// Returns information about the intersection of the object and the ray, if one exists.
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
    /// Checks whether the ray intersects the object, computes no additional information.
    /// If the offset is greater than `t_max`, it returns false.
    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool;
}
