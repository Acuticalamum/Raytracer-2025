mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod onb;
mod pdf;
mod perlin;
mod quad;
mod ray;
mod rtw_stb_image;
mod rtweekend;
mod sphere;
mod texture;
mod triangle;
mod vec3;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{RotateY, Translate};
use crate::material::Dielectric;
use crate::quad::Quad;
use crate::texture::{ImageTexture, NoiseTexture, SolidColor};
use color::Color;
use hittable_list::HittableList;
use material::{DiffuseLight, Lambertian, Metal};
use sphere::Sphere;
use std::fs::File;
use std::io::{self, BufWriter};
use std::sync::Arc;
use vec3::{Point3, Vec3};
#[allow(clippy::upper_case_acronyms)]
fn final_scene(image_width: usize, samples_per_pixel: usize, max_depth: usize) -> io::Result<()> {
    let path = std::path::Path::new("output/book2/Image23.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book2/Image23.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);

    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.48, 0.83, 0.53,
    )))));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rtweekend::random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(quad::make_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();
    world.add(Arc::new(BVHNode::new_from_list(&mut boxes1)));

    let light = Arc::new(DiffuseLight::new_from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.7, 0.3, 0.1,
    )))));
    world.add(Arc::new(Sphere::_new(
        center1,
        center2,
        50.0,
        Some(sphere_material),
    )));

    world.add(Arc::new(Sphere::static_new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Some(Arc::new(Dielectric::new(1.5))),
    )));
    world.add(Arc::new(Sphere::static_new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Some(Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0))),
    )));

    let boundary = Arc::new(Sphere::static_new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Some(Arc::new(Dielectric::new(1.5))),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::_new_with_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary2 = Arc::new(Sphere::static_new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Some(Arc::new(Dielectric::new(1.5))),
    ));
    world.add(Arc::new(ConstantMedium::_new_with_color(
        boundary2,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_material = Arc::new(Lambertian::new(earth_texture));
    world.add(Arc::new(Sphere::static_new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        Some(earth_material),
    )));

    let perlin_texture = Arc::new(NoiseTexture::_new(0.2));
    world.add(Arc::new(Sphere::static_new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Some(Arc::new(Lambertian::new(perlin_texture))),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.73, 0.73, 0.73,
    )))));
    for _ in 0..1000 {
        boxes2.add(Arc::new(Sphere::static_new(
            Point3::random_range(0.0, 165.0),
            10.0,
            Some(white.clone()),
        )));
    }

    let rotated = Arc::new(RotateY::new(
        Arc::new(BVHNode::new_from_list(&mut boxes2)),
        15.0,
    ));
    let translated = Arc::new(Translate::new(rotated, Vec3::new(-100.0, 270.0, 395.0)));
    world.add(translated);

    let mut camera = Camera::new(1.0, 400);
    camera.aspect_ratio = 1.0;
    camera.image_width = image_width;
    camera.samples_per_pixel = samples_per_pixel as u32;
    camera.max_depth = max_depth;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 40.0;
    camera.lookfrom = Point3::new(478.0, 278.0, -600.0);
    camera.lookat = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    let lights = Arc::new(HittableList::new());
    camera.initialize();
    camera.render(&world, lights, &mut out)?;

    Ok(())
}

fn main() -> io::Result<()> {
    final_scene(800, 1000, 10)
}
