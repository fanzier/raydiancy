pub use basic::*;
pub use img_output::*;
pub use physics::*;
pub use objects::*;
use std::f64;

// TODO: Move these constants in a struct `RenderOptions`.
const INTENSITY_THRESHOLD: f64= 1./256.;
const MAX_DEPTH: usize = 10;

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
        let right = horizontal * camera_dir.cross(self.camera.up).normalize();
        let up = right.cross(camera_dir).normalize();
        let up = horizontal / self.camera.aspect_ratio * up;

        let mut img = Image::new(self.camera.width, self.camera.height);
        for (left,down,col) in img.iter_mut() {
            let (x,y) = ((left as f64 / w) - 0.5, 0.5 - (down as f64 / h));
            let ray_dir = camera_dir + x * right + y * up;
            let ray = Ray::newn(self.camera.pos, ray_dir);
            *col = self.trace_ray(ray, 1.0, 0, f64::INFINITY);
        }
        return img;
    }

    /// Traces the ray through the scene and returns its color.
    fn trace_ray(&self, ray: Ray, intensity: f64, depth: usize, t_max: f64) -> AColor {
        let mut nearest: Option<DelayedIntersection> = None;
        let mut nearest_t: f64 = t_max;
        for obj in self.objects.iter() {
            if let Some(intersection) = obj.intersect(ray, nearest_t) {
                    nearest_t = intersection.t;
                    nearest = Some(intersection);
            }
        }
        intensity * match nearest {
            Some(intersection) => self.shade(ray, &intersection.eval(), intensity, depth + 1),
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
        self.compute_illuminance(ray.dir, inter)
        + self.compute_reflection_refraction(ray.dir, inter, intensity, depth)
    }

    /// Computes the illuminance at the given intersection point.
    /// This means that ambient, diffuse, and specular reflection are taken into account,
    /// but not mirror-like reflection or refraction for transparent objects.
    fn compute_illuminance(&self, dir: UnitVec3, inter: &Intersection) -> AColor {
        let mat = inter.material;
        // Start with the ambient color of the object.
        let mut color = (mat.ambient * (self.ambient_color * mat.color)).with_alpha();
        // Add the illuminance of every light up to get the final color:
        for light in self.lights.iter() {
            // Construct shadow ray:
            let light_vec = light.pos - inter.point;
            let t_max = light_vec.norm();
            let light_dir = light_vec.normalize();
            let shadow_ray = shadow_ray(inter, light_dir);
            // Check if the point is in the shadow of the current light source.
            if self.is_hit_by(shadow_ray, t_max) {
                continue // the point is in the shadow of this light source
            }
            // Compute the diffuse reflection:
            let lambert_coefficient = mat.diffuse * f64::max(0.0, light_dir * inter.normal);
            let lambert = lambert_coefficient * (light.col * mat.color);
            // Compute the specular reflection (Blinn-Phong):
            let specular_coefficient = mat.specular * compute_specular(-dir, light_dir, inter.normal, mat.shininess);
            let specular = specular_coefficient * light.col;
            // Add these two terms to overall color:
            color = color + lambert.with_alpha() + specular.with_alpha();
        }
        return color;
    }

    /// Computes the refraction for transparent objects and reflection for reflective ones.
    fn compute_reflection_refraction(&self, dir: UnitVec3, inter: &Intersection, intensity: f64, depth: usize) -> AColor {
        let mut color = AColor::new(0., 0., 0.);
        let mat = inter.material;

        // Compute the REFLECTION:
        if mat.reflectance > 0. && mat.reflectance * intensity > INTENSITY_THRESHOLD && depth < MAX_DEPTH {
            let reflected_ray = reflect_ray(inter, dir);
            let reflected_intensity = mat.reflectance * intensity;
            color = color + self.trace_ray(reflected_ray, reflected_intensity, depth + 1, f64::INFINITY);
        }

        // Compute the REFRACTION:
        if mat.refractivity > 0. && mat.refractivity * intensity > INTENSITY_THRESHOLD && depth < MAX_DEPTH {
            color = color + self.compute_recursive_refraction(dir, inter, intensity, depth);
        }

        return color;
    }

    /// Traces the reflected and refracted (except in case of total reflection).
    fn compute_recursive_refraction(&self, dir: UnitVec3, inter: &Intersection, intensity: f64, depth: usize) -> AColor {
        let mat = inter.material;
        // TODO: We assume that the ray travels to or from vacuum (which is almost always the case).
        // But, for example, if the ray travels from glass (1.5) to water (1.33),
        // the ior used here (1.33) is incorrect, should be 1.33/1.5.
        let (ior, normal) = if dir * inter.normal < 0. { // Ray enters object:
            (mat.refraction_index, inter.normal)
        } else { // Ray exits object:
            (1. / mat.refraction_index, -inter.normal)
        };
        let ref inter = Intersection { normal: normal, .. *inter };
        let reflected_ray = reflect_ray(inter, dir);
        let refracted_ray = refract_ray(inter, dir, ior);
        // TODO: Implement beer's law for light absorption inside material.
        match refracted_ray {
            None => { // Total internal reflection:
                let reflected_intensity = intensity * mat.refractivity;
                let reflected = self.trace_ray(reflected_ray, reflected_intensity, depth + 1, f64::INFINITY);
                return reflected_intensity * reflected
            },
            Some(refracted_ray) => { // Both reflection and refraction:
                let fresnel_factor = fresnel(dir, normal, ior);
                let refracted_intensity = intensity * mat.refractivity * (1. - fresnel_factor);
                let reflected_intensity = intensity * mat.refractivity * fresnel_factor;
                let reflected = self.trace_ray(reflected_ray, reflected_intensity, depth + 1, f64::INFINITY);
                let refracted = self.trace_ray(refracted_ray, refracted_intensity, depth + 1, f64::INFINITY);
                return refracted_intensity * refracted + reflected_intensity * reflected
            }
        }
    }
}
