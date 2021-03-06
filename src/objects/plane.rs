use basic::*;
use objects::surface::*;

/// Representation of a plane.
pub struct Plane {
    /// Normal vector of the plane.
    pub normal: UnitVec3,
    /// The offset is normal * x for any point x on the plane.
    pub offset: f64,
    /// The material of the plane.
    pub material: Material,
}

impl Surface for Plane {
    fn intersect<'a>(&'a self, ray: &'a Ray, t_max: f64) -> Option<DelayedIntersection> {
        let nd = self.normal * ray.dir;
        if f64::abs(nd) < EPS {
            return None;
        }
        let t = (self.offset - self.normal * ray.origin) / nd;
        if t < EPS || t > t_max {
            return None;
        }
        Some(DelayedIntersection::new(t, move || {
            // Make the normal vector point to the origin of the ray.
            // This is important for the epsilon displacement for shadow and reflection rays.
            let normal = if nd < 0. {
                self.normal
            } else {
                -self.normal
            };
            Intersection::new(ray, t, normal, self.material)
        }))
    }

    fn is_hit_by(&self, ray: &Ray, t_max: f64) -> bool {
        let nd = self.normal * ray.dir;
        if f64::abs(nd) < EPS {
            return false;
        }
        let t = (self.offset - self.normal * ray.origin) / nd;
        if t < EPS {
            return false;
        }
        t < t_max
    }

    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}
