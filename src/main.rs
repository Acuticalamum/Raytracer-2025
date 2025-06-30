mod aabb;
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
use crate::rtweekend::{INFINITY, random_double};
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
    let path = std::path::Path::new("output/book2/image1.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book2/image1.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);

    let mut world = HittableList::new();

    let ground_material: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))));
    world.add(Arc::new(Sphere::static_new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * rtweekend::random_double(),
                0.2,
                b as f64 + 0.9 * rtweekend::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Option<Arc<dyn Material>>;

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Some(Arc::new(Lambertian::new(albedo)));
                    let center2 = center + Vec3::new(0.0, random_double() * 0.5, 0.0);
                    world.add(Arc::new(Sphere::new(center, center2, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    sphere_material = Some(Arc::new(Metal::new(albedo, fuzz)));
                    world.add(Arc::new(Sphere::static_new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Some(Arc::new(Dielectric::new(1.5)));
                    world.add(Arc::new(Sphere::static_new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1: Option<Arc<dyn Material>> = Some(Arc::new(Dielectric::new(1.5)));
    world.add(Arc::new(Sphere::static_new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))));
    world.add(Arc::new(Sphere::static_new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3: Option<Arc<dyn Material>> =
        Some(Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Arc::new(Sphere::static_new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::new(16.0 / 9.0, 1200);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.initialize();
    cam.render(&world, &mut out)?;

    Ok(())
}
