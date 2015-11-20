use basic::*;

/// Represents a triangle.
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
        intersect_triangle(self.a, self.b, self.c, ray).map(|(e,f,_,_,t)| {
            let normal = e.cross(f).normalize();
            // Make the normal vector point to the origin of the ray.
            // This is important for the epsilon displacement for shadow and reflection rays.
            let normal = if normal * ray.dir < 0. { normal } else { -normal };
            Intersection::new(ray, t, normal, self.material)
        })
    }

    /// Checks whether the ray hits the triangle.
    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool {
        is_triangle_hit_by(self.a, self.b, self.c, ray, t_max)
    }
}

#[inline(always)]
#[doc(hidden)]
pub fn intersect_triangle(a: Vec3, b: Vec3, c: Vec3, ray: Ray) -> Option<(Vec3,Vec3,f64,f64,f64)> {
    let d = ray.dir;
    let e = b - a;
    let f = c - a;
    let g = ray.origin - a;
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
    return Some((e,f,u,v,t))
}

#[inline(always)]
#[doc(hidden)]
pub fn is_triangle_hit_by(a: Vec3, b: Vec3, c: Vec3, ray: Ray, t_max: f64) -> bool {
    let d = ray.dir;
    let e = b - a;
    let f = c - a;
    let g = ray.origin - a;
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
