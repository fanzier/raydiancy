pub use lin_alg::*;
pub use img_output::*;
use std::f64;

const INTENSITY_THRESHOLD: f64= 1./256.;
const MAX_DEPTH: usize = 4;

/// Structure for representing rays
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    /// Where the ray comes from.
    origin: Vec3,
    /// The direction the ray is traveling. Must be normalized!
    dir: Vec3,
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
}

/// Contains information about the intersection of a ray and an object.
pub struct Intersection {
    /// The intersection point is: (origin of ray) + t * (direction of ray).
    t: f64,
    /// The normal vector at the intersection point (orthogonal to the surface).
    normal: Vec3,
    /// The material properties at the intersection point.
    material: Material
}

/// Trait for finding ray intersections.
pub trait Surface {
    /// Returns information about the intersection of the object and the ray, if one exists.
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
    /// Checks whether the ray intersects the object, computes no additional information.
    /// If the offset is greater than `t_max`, it returns false.
    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool;
}

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
        Some(Intersection { t: t, normal: normal, material: self.material })
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
        Some(Intersection { t: t, normal: self.normal, material: self.material })
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
        Some(Intersection { t: t, normal: e.cross(f).normalize(), material: self.material })
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

/// Contains information about camera, like position, direction etc.
pub struct Camera {
    /// The position of the camera.
    pub pos: Vec3,
    /// A point that the camera looks at.
    pub look_at: Vec3,
    /// A vector pointing upwards, i.e. to the top in the image.
    pub up: Vec3,
    /// The horizontal field of view. Range: strictly between 0 and `PI`.
    pub horizontal_fov: f64,
    /// The quotient `width / height` of the image.
    pub aspect_ratio: f64,
    /// The width of the image in pixels.
    pub width: usize,
    /// The height of the image in pixels.
    pub height: usize
}

/// Information about a light source.
pub struct LightSource {
    pub pos: Vec3,
    pub col: Color
}

/// Contains all the information about a scene: camera and objects.
pub struct Scene {
    /// The camera in the scene.
    pub camera: Camera,
    /// The objects in the scene.
    pub objects: Vec<Box<Surface>>,
    /// The lights in the scene.
    pub lights: Vec<LightSource>,
    /// The color of ambient light in the scene.
    pub ambient_color: Color
}

impl Scene {
    /// Renders the scene and returns an image.
    pub fn render(&self) -> Image {
        let (w, h) = (self.camera.width as f64, self.camera.height as f64);
        let horizontal = (self.camera.horizontal_fov / 2.0).tan();
        let camera_dir = (self.camera.look_at - self.camera.pos).normalize();
        let up = horizontal / self.camera.aspect_ratio * self.camera.up.normalize();
        let right = horizontal * camera_dir.cross(self.camera.up).normalize();

        let mut img = Image::new(self.camera.width, self.camera.height);
        for (left,down,col) in img.iter_mut() {
            let (x,y) = ((left as f64 / w) - 0.5, 0.5 - (down as f64 / h));
            let ray_dir = camera_dir + x * right + y * up;
            let ray = Ray::new(self.camera.pos, ray_dir);
            *col = self.trace_ray(ray, 1.0, 0);
        }
        return img;
    }

    /// Traces the ray through the scene and returns its color.
    fn trace_ray(&self, ray: Ray, intensity: f64, depth: usize) -> AColor {
        let mut nearest: Option<Intersection> = None;
        let mut nearest_t: f64 = f64::INFINITY;
        for obj in self.objects.iter() {
            match obj.intersect(ray) {
                Some(intersection) => if intersection.t < nearest_t {
                    nearest_t = intersection.t;
                    nearest = Some(intersection);
                },
                None => ()
            }
        }
        intensity * match nearest {
            Some(ref intersection) => self.shade(ray, intersection, intensity, depth + 1),
            None => AColor::transparent()
        }
    }

    fn is_hit_by(&self, ray: Ray, t_max: f64) -> bool {
        for obj in self.objects.iter() {
            if obj.is_hit_by(ray, t_max) { return true }
        }
        false
    }

    /// Determines the color of an intersection point.
    fn shade(&self, ray: Ray, inter: &Intersection, intensity: f64, depth: usize) -> AColor {
        let mat = inter.material;
        // Start with the ambient color of the object.
        let mut color = (mat.ambient * (self.ambient_color * mat.color)).with_alpha();
        let point = ray.origin + (inter.t - EPS) * ray.dir;
        // Add the illuminance of every light up to get the final color:
        for light in self.lights.iter() {
            // Construct shadow ray:
            let light_vec = light.pos - point;
            let t_max = light_vec.norm();
            let light_dir = light_vec.normalize();
            let shadow = Ray::new(point, light_dir);
            if self.is_hit_by(shadow, t_max) {
                continue // the point is in the shadow of this light source
            }
            // Compute the diffuse reflection:
            let lambert_coefficient = mat.diffuse * f64::max(0.0, light_dir * inter.normal);
            let lambert = lambert_coefficient * (light.col * mat.color);
            // Compute the specular reflection (Blinn-Phong):
            let origin_dir = (ray.origin - point).normalize();
            let halfway = (light_dir + origin_dir).normalize();
            let specular_coefficient = mat.specular * f64::max(0.0, halfway * inter.normal).powf(mat.shininess);
            let specular = specular_coefficient * light.col;
            // Add these two terms to overall color:
            color = color + lambert.with_alpha() + specular.with_alpha();
        }
        // Compute the reflection:
        if mat.reflectance > 0. && mat.reflectance * intensity > INTENSITY_THRESHOLD && depth < MAX_DEPTH {
            let reflected_dir = ray.dir - 2. * (ray.dir * inter.normal) * inter.normal;
            let new_intensity = mat.reflectance * intensity;
            color = color + self.trace_ray(Ray::new(point, reflected_dir), new_intensity, depth + 1);
        }
        return color;
    }
}
