use basic::*;

/// Representation of a sphere.
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material
}

impl Surface for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let x = ray.origin - self.center;
        let b = 2.0 * x * ray.dir;
        let c = x.norm2() - self.radius*self.radius;
        let discriminant = b*b - 4.0*c;
        if discriminant < 0.0 {
            return None;
        }
        let t = (-b - f64::sqrt(discriminant)) / 2.0;
        if t < EPS {
            return None;
        }
        let normal = (x + t * ray.dir).normalize();
        Some(Intersection::new(ray, t, normal, self.material))
    }

    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool {
        let x = ray.origin - self.center;
        let b = 2.0 * x * ray.dir;
        let c = x.norm2() - self.radius*self.radius;
        let discriminant = b*b - 4.0*c;
        if discriminant < 0.0 {
            return false;
        }
        let t = (-b - f64::sqrt(discriminant)) / 2.0;
        if t < EPS {
            return false;
        }
        t < t_max
    }
}
