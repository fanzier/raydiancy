use basic::*;

pub struct Triangle {
    // First point of the triangle.
    pub a: Vec3,
    // Second point of the triangle.
    pub b: Vec3,
    // Third point of the triangle.
    pub c: Vec3,
    // The material of the triangle.
    pub material: Material
}

impl Surface for Triangle {
    /// Intersects a ray with a triangle.
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let d = ray.dir;
        let e = self.b - self.a;
        let f = self.c - self.a;
        let g = ray.origin - self.a;
        let p = d.cross(f);
        let det = p * e;
        // If the determinant is close to 0, the ray misses the triangle.
        if det.abs() < EPS {
            return None
        }
        let u = p * g / det;
        if u < 0.0 || u > 1.0 {
            return None
        }
        let q = g.cross(e);
        let v = q * d / det;
        if v < 0.0 || u + v > 1.0 {
            return None
        }
        let t = q * f / det;
        if t < EPS {
            return None
        }
        let normal = e.cross(f).normalize();
        Some(Intersection::new(ray, t, normal, self.material))
    }

    /// Checks whether the ray hits the triangle.
    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool {
        let d = ray.dir;
        let e = self.b - self.a;
        let f = self.c - self.a;
        let g = ray.origin - self.a;
        let p = d.cross(f);
        let det = p * e;
        // If the determinant is close to 0, the ray misses the triangle.
        if det.abs() < EPS {
            return false
        }
        let u = p * g / det;
        if u < 0.0 || u > 1.0 {
            return false
        }
        let q = g.cross(e);
        let v = q * d / det;
        if v < 0.0 || u + v > 1.0 {
            return false
        }
        let t = q * f / det;
        if t < EPS {
            return false
        }
        t < t_max
    }
}
