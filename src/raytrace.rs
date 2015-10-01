pub use lin_alg::*;
pub use img_output::*;
use std::f64;

/// Structure for representing rays
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    /// Where the ray comes from.
    origin: Vec3,
    /// The direction the ray is traveling. Must be normalized!
    dir: Vec3,
    /// How much the color of this ray will affect the pixel in the end. Range: 0 to 1
    intensity: f64,
    /// How often the ray was reflected/refracted before
    depth: usize
}

impl Ray {
    /// Creates a `Ray`.
    ///
    /// It normalizes the direction vector.
    /// # Example
    /// ```
    /// use raydiancy::raytrace::*;
    /// assert_eq!(Ray::new(Vec3::zero(), 42.0*Vec3::e1(), 0.0, 0), Ray::new(Vec3::zero(), Vec3::e1(), 0.0, 0));
    /// ```
    pub fn new(origin: Vec3, dir: Vec3, intensity: f64, depth: usize) -> Ray {
        Ray { origin: origin, dir: dir.normalize(), intensity: intensity, depth: depth }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    /// Color of the material
    pub color: Color
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

/// Contains all the information about a scene: camera and objects.
pub struct Scene {
    /// The camera in the scene.
    pub camera: Camera,
    /// The objects in the scene.
    pub objects: Vec<Box<Surface>>
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
            let ray = Ray::new(self.camera.pos, ray_dir, 1.0, 0);
            *col = self.trace_ray(ray);
        }
        return img;
    }

    /// Traces the ray through the scene and returns its color.
    fn trace_ray(&self, ray: Ray) -> Color {
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
        match nearest {
            Some(intersection) => intersection.material.color,
            None => Color::new(0.0,0.0,0.0,0.0)
        }
    }
}