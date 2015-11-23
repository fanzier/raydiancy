extern crate raydiancy;

use raydiancy::raytrace::*;
use std::path::Path;
use std::fs;

macro_rules! render {
    ($scene:ident) => { {
        let _ = fs::create_dir("output/");
        let name = stringify!($scene);
        println!("Scene: {}", name);
        println!("  Constructing ...");
        let scene = $scene();
        println!("  Rendering ...");
        let rendered = scene.render();
        let file = format!("output/{}.png", name);
        println!("  Writing to file {}...", file);
        write_pixels_to_file(
            rendered,
            Path::new(&file),
        );
    } }
}

fn main() {
    render!(bunny);
    render!(dragon);
    render!(spheres);
}

fn bunny() -> Scene {
    let width = 640;
    let height = 360;
    let material = color_material(white());
    let camera = Camera {
        pos: Vec3::new(0.0, 4.0, 12.0),
        look_at: 4.0 * Vec3::e2(),
        up: Vec3::e2().to(),
        horizontal_fov: 120_f64.to_radians(),
        aspect_ratio: width as f64 / height as f64,
        width: width,
        height: height
    };
    let mesh = Mesh::from_obj_file("scenes/bunny.obj", material);
    let light = LightSource {
        pos: Vec3::new(0.0, 10.0, 10.0),
        col: white()
    };
    return Scene {
        camera: camera,
        objects: vec![
            Box::new(mesh.unwrap()),
            ],
        ambient_color: white(),
        lights: vec![light]
    };
}

fn dragon() -> Scene {
    let width = 640;
    let height = 360;
    let material = color_material(white());
    let camera = Camera {
        pos: Vec3::new(0.0, 4.0, 12.0),
        look_at: 4.0 * Vec3::e2(),
        up: Vec3::e2().to(),
        horizontal_fov: 120_f64.to_radians(),
        aspect_ratio: width as f64 / height as f64,
        width: width,
        height: height
    };
    let mesh = Mesh::from_obj_file("scenes/dragon.obj", material);
    let light = LightSource {
        pos: Vec3::new(0.0, 10.0, 10.0),
        col: white()
    };
    return Scene {
        camera: camera,
        objects: vec![
            Box::new(mesh.unwrap()),
            ],
        ambient_color: white(),
        lights: vec![light]
    };
}

fn spheres() -> Scene {
    let width = 1280;
    let height = 720;
    // Camera:
    let camera = Camera {
        pos: Vec3::new(9.0, 4.0, 1.0),
        look_at: Vec3::new(0.0, 0.0, -3.0),
        up: Vec3::e2().to(),
        horizontal_fov: 120_f64.to_radians(),
        aspect_ratio: width as f64 / height as f64,
        width: width,
        height: height
    };

    // Objects:
    let wall1 = Plane {
        normal: Vec3::e3(),
        offset: -5.0,
        material: reflective_material(0.9, black()),
    };
    let wall2 = Plane {
        normal: Vec3::e1(),
        offset: -10.0,
        material: reflective_material(0.9, black()),
    };
    let floor = Plane {
        normal: Vec3::e2(),
        offset: 0.0,
        material: color_material(white()),
    };
    let big_radius = 3.;
    let small_radius = 1.;
    let num_spheres = 8;
    let mut objects: Vec<Box<Surface>> = (0..num_spheres).map(|i| {
        let angle = (2 * i) as f64 * PI  / (num_spheres as f64);
        Box::new(Sphere {
            center: big_radius *
                Vec3::new(angle.sin(), 0.0, angle.cos()) + small_radius * Vec3::e2(),
            radius: small_radius,
            material: color_material(Color::new(
                (angle / 2.).sin(),
                (angle / 2. + PI / 3.).sin().abs(),
                (angle / 2. + PI / 1.5).sin().abs())),
        }) as Box<Surface>
    }).collect();
    objects.push(Box::new(Sphere{
        center: Vec3::new(0.0, big_radius / 2., 0.0),
        radius: big_radius / 2.,
        material: reflective_material(0.9, white()),
    }));
    objects.push(Box::new(floor));
    objects.push(Box::new(wall1));
    objects.push(Box::new(wall2));

    // Lights:
    let light = LightSource {
        pos: Vec3::new(0.0, 10.0, 0.0),
        col: 0.5 * white(),
    };
    let light2 = LightSource {
        pos: Vec3::new(10.0, 10.0, 10.0),
        col: 0.5 * white(),
    };

    // Scene:
    return Scene {
        camera: camera,
        objects: objects,
        ambient_color: Color::new(1.0, 1.0, 1.0),
        lights: vec![
            light,
            light2,
            ]
    }
}
