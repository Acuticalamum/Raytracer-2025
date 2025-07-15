mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod obj;
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
use crate::hittable::{HitRecord, Hittable, RotateY, Translate};
use crate::material::Dielectric;
use crate::quad::Quad;
use crate::rtweekend::{INFINITY, random_double};
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor, Texture};
use crate::triangle::Triangle;
use color::{Color, write_color};
use console::style;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use interval::Interval;
use material::{DiffuseLight, EmptyMaterial, Lambertian, Material, Metal};
use ray::Ray;
use sphere::Sphere;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::option::Option;
use std::sync::Arc;
use vec3::{Point3, Vec3};

pub fn cornell_box() -> io::Result<()> {
    let path = std::path::Path::new("output/book3/image17.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book3/image17.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);

    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.12, 0.45, 0.15,
    )))));
    let blue = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.20, 0.40, 0.80,
    )))));
    let light = Arc::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));

    /*world.add(Arc::new(Triangle::new_with_vector(
        Point3::new(100.0, 400.0, 500.0),
        Vec3::new(0.0, 150.0, 0.0),
        Vec3::new(150.0, 0.0, 0.0),
        blue.clone(),
    )));*/

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let aluminum = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));

    let box1 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    /*let box2 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);*/
    let glass = Arc::new(Dielectric::new(1.5));
    let sphere = Arc::new(Sphere::static_new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        Some(glass),
    ));

    //world.add(sphere);

    let coffin = Arc::new(obj::load_obj_model("objects/coffin.obj", 50.0));
    let coffin = Arc::new(Translate::new(coffin, Vec3::new(100.0, 100.0, 400.0)));
    world.add(coffin);

    let empty_material: Arc<dyn Material> = Arc::new(material::EmptyMaterial);

    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        empty_material.clone(),
    )));

    lights.add(Arc::new(Sphere::static_new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        Some(empty_material),
    )));

    let lights = Arc::new(lights);

    let mut cam = Camera::new(1.0, 600);

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 20;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.initialize();
    cam.render(&world, lights, &mut out)?;
    Ok(())
}

pub fn final_scene() -> io::Result<()> {
    Ok(())
}

fn main() -> io::Result<()> {
    cornell_box()
}
