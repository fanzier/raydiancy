use basic::*;
use std::f64;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use objects::bvh::*;
use objects::surface::*;
use objects::triangle::{intersect_triangle, is_triangle_hit_by};

/// Represents a triangle that is part of a mesh.
struct Face {
    pub vertex_indices: (usize, usize, usize)
}

impl Face {
    fn new(i: usize, j: usize, k: usize) -> Face {
        Face { vertex_indices: (i, j, k) }
    }
}

/// Represents a triangle mesh.
///
/// It is usually constructed from an OBJ file using `Mesh::from_obj_file`.
pub struct Mesh {
    vertices: Vec<Vec3>,
    faces: Vec<Face>,
    material: Material
}

impl Mesh {
    fn face_vertices(&self, f: &Face) -> [&Vec3; 3] {
        let (i,j,k) = f.vertex_indices;
        [&self.vertices[i], &self.vertices[j], &self.vertices[k]]
    }

    /// Builds a mesh from the OBJ file `path` and out of the given `material`.
    pub fn from_obj_file(path: &str, material: Material) -> io::Result<Bvh<Mesh>> {
        let mut vertices: Vec<Vec3> = vec![];
        let mut normals: Vec<Vec3> = vec![];
        let mut faces: Vec<Face> = vec![];

        let file = try!(File::open(path));
        let buf_reader = io::BufReader::new(file);

        for line in buf_reader.lines() {
            let line = try!(line);
            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("v") =>
                    match Mesh::parse3::<_,f64>(&mut tokens) {
                        Some((x,y,z)) => vertices.push(Vec3::new(x,y,z)),
                        None => continue
                    },
                Some("vn") =>
                    match Mesh::parse3::<_,f64>(&mut tokens) {
                        Some((x,y,z)) => normals.push(Vec3::new(x,y,z)),
                        None => continue
                    },
                Some("f") =>
                    // TODO: Handle normal vectors
                    match Mesh::parse3::<_,usize>(&mut tokens) {
                        Some((i,j,k)) =>
                            faces.push(Face::new(i - 1, j - 1, k - 1)),
                        None => continue
                    },
                _ => continue
            }
        }
        Ok(Bvh::new(Mesh {
            vertices: vertices,
            faces: faces,
            material: material,
        }))
    }

    fn parse3<'a, I, T>(tokens: &mut I) -> Option<(T,T,T)>
        where I: Iterator<Item=&'a str>, T: FromStr
    {
        tokens.next().and_then(|s| str::parse::<T>(s).ok()).and_then(|x|
        tokens.next().and_then(|s| str::parse::<T>(s).ok()).and_then(|y|
        tokens.next().and_then(|s| str::parse::<T>(s).ok()).map(|z| (x,y,z))))
    }

    /// Computes the bounding box for the given face.
    fn bounding_box_face(&self, f: &Face) -> Aabb {
        let min = self.face_vertices(f).iter().fold(
            f64::INFINITY * Vec3::ones(),
            |acc, &&item| acc.max(item)
        );
        let max = self.face_vertices(f).iter().fold(
            -f64::INFINITY * Vec3::ones(),
            |acc, &&item| acc.max(item)
        );
        Aabb::new(min, max)
    }
}

impl Surface for Mesh {
    fn intersect(&self, ray: Ray, t_max: f64) -> Option<DelayedIntersection> {
        let mut t_min = t_max;
        let mut nearest_face = None;
        for face in self.faces.iter() {
            let vertices = self.face_vertices(face);
            let a = *vertices[0];
            let b = *vertices[1];
            let c = *vertices[2];
            intersect_triangle(a, b, c, ray, t_min).map(|(_,_,_,_,t)| {
                t_min = t;
                nearest_face = Some(face);
            });
        }
        nearest_face.map(|f| {
            DelayedIntersection::new(t_min, move || {
                let vertices = self.face_vertices(f);
                // TODO: Interpolate normal if vertex normals are given.
                let normal = (*vertices[1] - *vertices[0]).cross(*vertices[2] - *vertices[0]).normalize();
                // Make the normal vector point to the origin of the ray.
                // This is important for the epsilon displacement for shadow and reflection rays.
                let normal = if normal * ray.dir < 0. { normal } else { -normal };
                Intersection::new(ray, t_min, normal, self.material)
            })
        })
    }

    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool {
        for face in self.faces.iter() {
            let vertices = self.face_vertices(face);
            let a = *vertices[0];
            let b = *vertices[1];
            let c = *vertices[2];
            if is_triangle_hit_by(a, b, c, ray, t_max) {
                return true
            }
        }
        false
    }

    fn bounding_box(&self) -> Option<Aabb> {
        if self.faces.len() == 0 {
            return None
        }
        let min = f64::INFINITY * Vec3::ones();
        let max = -f64::INFINITY * Vec3::ones();
        let (min, max): (Vec3, Vec3)= self.faces.iter().fold(
            (min,max),
            |(acc_min, acc_max), item| {
                let aabb = self.bounding_box_face(item);
                (acc_min.min(aabb.min()), acc_max.max(aabb.max()))
            }
        );
        Some(Aabb::new(min, max))
    }
}

impl SurfaceContainer for Mesh {
    fn elem_is_hit_by(&self, i: usize, ray: Ray, t_max: f64) -> bool {
        let vertices = self.face_vertices(&self.faces[i]);
        return is_triangle_hit_by(*vertices[0], *vertices[1], *vertices[2], ray, t_max)
    }

    fn elem_intersect(&self, i: usize, ray: Ray, t_max: f64) -> Option<DelayedIntersection> {
        let ref face = self.faces[i];
        let vertices = self.face_vertices(face);
        let a = *vertices[0];
        let b = *vertices[1];
        let c = *vertices[2];
        intersect_triangle(a, b, c, ray, t_max).map(|(_,_,_,_,t)| {
            DelayedIntersection::new(t, move || {
                // TODO: Interpolate normal if vertex normals are given.
                let normal = (b - a).cross(c - a).normalize();
                // Make the normal vector point to the origin of the ray.
                // This is important for the epsilon displacement for shadow and reflection rays.
                let normal = if normal * ray.dir < 0. { normal } else { -normal };
                Intersection::new(ray, t, normal, self.material)
            })
        })
    }

    fn elem_bounding_box(&self, i: usize) -> Option<Aabb> {
        let ref face = self.faces[i];
        let vertices = self.face_vertices(face);
        let a = *vertices[0];
        let b = *vertices[1];
        let c = *vertices[2];
        let min = a.min(b).min(c);
        let max = a.max(b).max(c);
        Some(Aabb::new(min, max))
    }

    fn count(&self) -> usize {
        self.faces.len()
    }
}
