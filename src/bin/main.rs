extern crate raydiancy;

use raydiancy::raytrace::*;
use std::path::Path;

fn main() {
    let width = 640;
    let height = 360;
    let camera = Camera {
        pos: Vec3::new(10.0, 0.0, 0.0),
        look_at: Vec3::zero(),
        up: Vec3::e2(),
        horizontal_fov: 120_f64.to_radians(),
        aspect_ratio: width as f64 / height as f64,
        width: width,
        height: height
    };
    let sphere = Sphere {
        center: Vec3::new(0.0,0.0,0.0),
        radius: 5.0,
        material: Material { color: Color::new(0.0, 0.0, 1.0, 1.0) }
    };
    let scene = Scene {
        camera: camera,
        objects: vec![Box::new(sphere)]
    };
    let img = scene.render();
    write_pixels_to_file(img, Path::new("output.png"));
}
