use std::f64;
use basic::*;

/// Helper function for intersection test.
fn sign(f: f64) -> usize {
    if f < 0. { 1 } else { 0 }
}

/// The material used when drawing bounding boxes for debugging purposes.
fn bounding_box_material() -> Material {
    Material {
        color: Color::new(1.0, 0.2, 0.2),
        ambient: 0.025,
        diffuse: 0.025,
        specular: 0.,
        shininess: 1.,
        reflectance: 0.,
        refractivity: 0.95,
        refraction_index: 1.
    }
}

/// Represents an axis-aligned bounding box.
#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    vertices: [Vec3; 2]
}

impl Aabb {
    /// Creates an axis-aligned bounding box, given any two opposite vertices.
    pub fn new(v: Vec3, w: Vec3) -> Aabb {
        Aabb { vertices: [v.min(w), v.max(w)] }
    }

    /// Creates an empty axis-aligned bounding box.
    /// It is the neutral element of `Aabb::union`.
    pub fn empty() -> Aabb {
        Aabb { vertices: [f64::INFINITY * Vec3::ones(), -f64::INFINITY * Vec3::ones()]}
    }

    /// Returns the vertex with smallest coordinates.
    pub fn min(&self) -> Vec3 {
        self.vertices[0]
    }

    /// Returns the vertex with greatest coordinates.
    pub fn max(&self) -> Vec3 {
        self.vertices[1]
    }

    /// Returns whether `self` contains the other bounding box.
    ///
    /// ```
    /// use raydiancy::basic::*;
    /// use raydiancy::objects::aabb::*;
    /// assert!(Aabb::new(Vec3::zero(), Vec3::ones()).contains(&Aabb::empty()));
    /// assert!(Aabb::new(Vec3::zero(), Vec3::ones()).contains(&Aabb::new(Vec3::zero(), 0.5 * Vec3::ones())));
    // ```
    pub fn contains(&self, b: &Aabb) -> bool {
        self.vertices[0] <= b.vertices[0] && b.vertices[1] <= self.vertices[1]
    }

    /// Returns whether `self` is contained in the other bounding box.
    pub fn is_contained(&self, b: &Aabb) -> bool {
        b.contains(self)
    }

    /// Returns the tightest bounding box around the union of the two given ones.
    pub fn union(&self, b: &Aabb) -> Aabb {
        Aabb { vertices: [
            self.vertices[0].min(b.vertices[0]),
            self.vertices[1].max(b.vertices[1])
            ],
        }
    }

    /// Given an iterator of bounding boxes, returns the tightest box around their union.
    pub fn union_all<T>(boxes: &mut T) -> Aabb where T: Iterator<Item=Aabb> {
        boxes.fold(Aabb::empty(), |acc, ref aabb| {
            acc.union(aabb)
        })
    }

    /// Returns the vector from the smallest vertex to the largest vertex.
    pub fn diagonal(&self) -> Vec3 {
        self.vertices[1] - self.vertices[0]
    }

    /// Returns the direction of the longest side (0 for x, 1 for y, 2 for z) and its length.
    pub fn longest_side(&self) -> (usize, f64) {
        let dim = self.diagonal();
        let mut max = (0, dim.x());
        if dim.y() > max.1 {
            max = (1, dim.y());
        }
        if dim.z() > max.1 {
            max = (2, dim.z());
        }
        max
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

    /// Computes the distance of the nearest intersection point if its less than `t1`,
    /// and `f64::INFINITY` otherwise.
    pub fn distance(&self, r: Ray, t1: f64) -> f64 {
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
            return f64::INFINITY;
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
            return f64::INFINITY;
        }
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }
        if tmin < EPS && tmax > t1 {
            0.0
        } else if EPS < tmin && tmin < t1 {
            tmin
        } else if EPS < tmax && tmax < t1 {
            tmax
        } else {
            f64::INFINITY
        }
    }

    /// Computes the intersection with this bounding box.
    /// This is supposed to be used for debugging only.
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
