use basic::*;

/// Representation of a plane.
pub struct Plane {
    /// Normal vector of the plane.
    pub normal: Vec3,
    /// The offset is normal * x for any point x on the plane.
    pub offset: f64,
    /// The material of the plane.
    pub material: Material
}

impl Surface for Plane {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let nd = self.normal * ray.dir;
        if f64::abs(nd) < EPS {
            return None
        }
        let t = (self.offset - self.normal * ray.origin) / nd;
        if t < EPS {
            return None
        }
        Some(Intersection::new(ray, t, self.normal, self.material))
    }

    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool {
        let nd = self.normal * ray.dir;
        if f64::abs(nd) < EPS {
            return false
        }
        let t = (self.offset - self.normal * ray.origin) / nd;
        if t < EPS {
            return false
        }
        t < t_max
    }
}