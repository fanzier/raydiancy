extern crate raydiancy;

use raydiancy::raytrace::*;
use std::path::Path;

fn main() {
    let width = 1600;
    let height = 900;
    let camera = Camera {
        pos: Vec3::new(20.0, 0.0, 0.0),
        look_at: Vec3::zero(),
        up: Vec3::e2(),
        horizontal_fov: 120_f64.to_radians(),
        aspect_ratio: width as f64 / height as f64,
        width: width,
        height: height
    };
    let background = Plane {
        normal: Vec3::e1(),
        offset: -100.0,
        material: Material { color: Color::new(1.0, 1.0, 1.0, 1.0)}
    };
    let sphere = Sphere {
        center: Vec3::new(0.0,0.0,0.0),
        radius: 5.0,
        material: Material { color: Color::new(0.0, 0.0, 1.0, 1.0) }
    };
    let plane = Plane {
        normal: Vec3::e2(),
        offset: -1.0,
        material: Material { color: Color::new(0.5, 0.5, 0.5, 1.0) }
    };
    let triangle = Triangle {
        a: Vec3::zero(),
        b: 8.0 * Vec3::e2(),
        c: 8.0 * Vec3::e3(),
        material: Material { color: Color::new(0.0, 1.0, 0.0, 1.0) }
    };
    let scene = Scene {
        camera: camera,
        objects: vec![
            Box::new(background),
            Box::new(sphere),
            Box::new(plane),
            Box::new(triangle)
            ]
    };
    let img = scene.render();
    write_pixels_to_file(img, Path::new("output.png"));
}
