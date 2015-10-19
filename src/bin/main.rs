extern crate raydiancy;

use raydiancy::raytrace::*;
use std::path::Path;

fn main() {
    let width = 1600;
    let height = 900;
    let material = Material {
        color: Color::new(0.0,0.0,0.0),
        ambient: 0.18,
        specular: 0.5,
        shininess: 500.0,
        diffuse: 0.32
    };
    let camera = Camera {
        pos: Vec3::new(20.0, 10.0, 20.0),
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
        material: Material { color: Color::new(1.0, 1.0, 1.0), .. material }
    };
    let sphere = Sphere {
        center: Vec3::new(5.0, 1.0, 0.0),
        radius: 5.0,
        material: Material { color: Color::new(0.0, 0.0, 1.0), .. material }
    };
    let plane = Plane {
        normal: Vec3::e2(),
        offset: 0.0,
        material: Material { color: Color::new(0.5, 0.5, 0.5), .. material }
    };
    let triangle = Triangle {
        a: Vec3::zero(),
        b: 8.0 * Vec3::e2(),
        c: 8.0 * Vec3::e3(),
        material: Material { color: Color::new(0.0, 1.0, 0.0), .. material }
    };
    let light = LightSource {
        pos: Vec3::new(10.0, 10.0, 0.0),
        col: Color::new(1.0, 1.0, 1.0)
    };
    let scene = Scene {
        camera: camera,
        objects: vec![
            Box::new(background),
            Box::new(sphere),
            Box::new(plane),
            Box::new(triangle)
            ],
        ambient_color: Color::new(1.0, 1.0, 1.0),
        lights: vec![light]
    };
    let img = scene.render();
    write_pixels_to_file(img, Path::new("output.png"));
}
