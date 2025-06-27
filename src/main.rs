mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Dielectric;
use crate::rtweekend::INFINITY;
use color::{Color, write_color};
use console::style;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use interval::Interval;
use material::{Lambertian, Material, Metal};
use ray::Ray;
use sphere::Sphere;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::option::Option;
use std::sync::Arc;
use vec3::{Point3, Vec3};

fn main() -> io::Result<()> {
    let path = std::path::Path::new("output/book1/image19.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book1/image19.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);

    let R = (PI / 4.0).cos();

    let material_left: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0))));
    let material_right: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0))));

    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(
        Point3::new(-R, 0.0, -1.0),
        R,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(R, 0.0, -1.0),
        R,
        material_right,
    )));

    let mut cam = Camera::new(16.0 / 9.0, 400);

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 90.0;

    cam.render(&world, &mut out)?;

    Ok(())
}
