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
use crate::rtweekend::INFINITY;
use color::{Color, write_color};
use console::style;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use interval::Interval;
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::sync::Arc;
use vec3::{Point3, Vec3};

fn main() -> io::Result<()> {
    let path = std::path::Path::new("output/book1/image10.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book1/image10.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new(16.0 / 9.0, 400);
    cam.render(&world, &mut out)?;

    Ok(())
}
