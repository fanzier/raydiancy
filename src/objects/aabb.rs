use basic::*;

/// Helper function for intersection test.
fn sign(f: f64) -> usize {
    if f < 0. { 1 } else { 0 }
}

/// The material used when drawing bounding boxes for debugging purposes
fn bounding_box_material() -> Material {
    Material {
        color: Color::new_gray(0.5),
        ambient: 0.1,
        diffuse: 0.2,
        specular: 0.,
        shininess: 1.,
        reflectance: 0.,
        refractivity: 0.7,
        refraction_index: 1.
    }
}

/// Represents an axis-aligned bounding box.
pub struct Aabb {
    vertices: [Vec3; 2]
}

impl Aabb {
    /// Creates an axis-aligned bounding box, given any two opposite vertices.
    pub fn new(v: Vec3, w: Vec3) -> Aabb {
        Aabb { vertices: [v.min(w), v.max(w)] }
    }

    /// Returns the vertex with smallest coordinates.
    pub fn min(&self) -> Vec3 {
        self.vertices[0]
    }

    /// Returns the vertex with greatest coordinates.
    pub fn max(&self) -> Vec3 {
        self.vertices[1]
    }

    /// Checks wether the intersection of the ray from t=EPS to t=t1 and the box is nonempty.
    ///
    /// In contrast to is_hit_by, this also returns true
    /// if this part of the ray is completely inside the box.
    pub fn passes_through(&self, r: Ray, t1: f64) -> bool {
        // This an adaption of the code from the paper
        // "An Efficient and Robust Ray–Box Intersection Algorithm" by Williams et. al.
        // http://www.cs.utah.edu/~awilliam/box/
        // TODO: Maybe store the inverse vector and sign inside struct Ray?
        let r_inv = Vec3::new(1. / r.dir[0], 1. / r.dir[1], 1. / r.dir[2]);
        let sign = [sign(r_inv[0]), sign(r_inv[1]), sign(r_inv[2])];
        let mut tmin = (self.vertices[sign[0]].x() - r.origin.x()) * r_inv.x();
        let mut tmax = (self.vertices[1-sign[0]].x() - r.origin.x()) * r_inv.x();
        let tymin = (self.vertices[sign[1]].y() - r.origin.y()) * r_inv.y();
        let tymax = (self.vertices[1-sign[1]].y() - r.origin.y()) * r_inv.y();
        if (tmin > tymax) || (tymin > tmax) {
            return false
        }
        if tymin > tmin {
            tmin = tymin
        }
        if tymax < tmax {
            tmax = tymax;
        }
        let tzmin = (self.vertices[sign[2]].z() - r.origin.z()) * r_inv.z();
        let tzmax = (self.vertices[1-sign[2]].z() - r.origin.z()) * r_inv.z();
        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }
        tmin < t1 && tmax > EPS
    }

    pub fn intersect(&self, r: Ray, t1: f64) -> Option<Intersection> {
        // This an adaption of the code from the paper
        // "An Efficient and Robust Ray–Box Intersection Algorithm" by Williams et. al.
        // http://www.cs.utah.edu/~awilliam/box/
        // TODO: Maybe store the inverse vector and sign inside struct Ray?
        let r_inv = Vec3::new(1. / r.dir[0], 1. / r.dir[1], 1. / r.dir[2]);
        let sign = [sign(r_inv[0]), sign(r_inv[1]), sign(r_inv[2])];
        let mut imin = 0;
        let mut imax = 0;
        let mut tmin = (self.vertices[sign[0]].x() - r.origin.x()) * r_inv.x();
        let mut tmax = (self.vertices[1-sign[0]].x() - r.origin.x()) * r_inv.x();
        let tymin = (self.vertices[sign[1]].y() - r.origin.y()) * r_inv.y();
        let tymax = (self.vertices[1-sign[1]].y() - r.origin.y()) * r_inv.y();
        if (tmin > tymax) || (tymin > tmax) {
            return None
        }
        if tymin > tmin {
            tmin = tymin;
            imin = 1;
        }
        if tymax < tmax {
            tmax = tymax;
            imax = 1;
        }
        let tzmin = (self.vertices[sign[2]].z() - r.origin.z()) * r_inv.z();
        let tzmax = (self.vertices[1-sign[2]].z() - r.origin.z()) * r_inv.z();
        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }
        if tzmin > tmin {
            tmin = tzmin;
            imin = 2;
        }
        if tzmax < tmax {
            tmax = tzmax;
            imax = 2;
        }
        if tmax < EPS || tmin > t1 {
            return None
        }
        let (i,t) = if tmin > EPS {
            (imin, tmin)
        } else if tmax < t1 {
            (imax, tmax)
        } else { // The part from EPS to t1 of the ray is completely inside the box:
            return None
        };
        let normal = Vec3::e(i);
        Some(Intersection::new(r, t, normal.assert_unit_vector(), bounding_box_material()))
    }
}