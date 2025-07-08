mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod rtw_stb_image;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;
mod onb;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{HitRecord, Hittable, RotateY, Translate};
use crate::material::Dielectric;
use crate::quad::Quad;
use crate::rtweekend::{INFINITY, random_double};
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor, Texture};
use color::{Color, write_color};
use console::style;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use interval::Interval;
use material::{DiffuseLight, Lambertian, Material, Metal};
use ray::Ray;
use sphere::Sphere;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::option::Option;
use std::sync::Arc;
use vec3::{Point3, Vec3};

fn bouncing_spheres() -> io::Result<()> {
    let path = std::path::Path::new("output/book2/image2.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book2/image2.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);
    let mut world = HittableList::new();

    let ground_material: Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new(Arc::new(
        CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)),
    ))));
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
                    let albedo: Color = Color::random() * Color::random();
                    sphere_material =
                        Some(Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo)))));
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

    let material2: Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new(Arc::new(
        SolidColor::new(Color::new(0.4, 0.2, 0.1)),
    ))));
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

    let world = HittableList::from(Arc::new(BVHNode::new_from_list(&mut world)));

    let mut cam = Camera::new(16.0 / 9.0, 1200);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 10;
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

fn checkered_spheres() -> io::Result<()> {
    let path = std::path::Path::new("output/book2/image3.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book2/image3.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);
    let mut world = HittableList::new();

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::static_new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Some(Arc::new(Lambertian::new(checker.clone()))),
    )));

    world.add(Arc::new(Sphere::static_new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Some(Arc::new(Lambertian::new(checker))),
    )));

    let mut cam = Camera::new(16.0 / 9.0, 1200);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    cam.initialize();
    cam.render(&world, &mut out)?;
    Ok(())
}

pub fn earth() -> io::Result<()> {
    let path = std::path::Path::new("output/book2/image4.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book2/image4.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("earthmap.png"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::static_new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        Some(earth_surface),
    ));

    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.initialize();

    let mut world = HittableList::new();
    world.add(globe);

    cam.render(&world, &mut out)?;

    Ok(())
}

pub fn perlin_spheres() -> io::Result<()> {
    let path = std::path::Path::new("output/book2/image15.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book2/image15.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    world.add(Arc::new(Sphere::static_new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new(pertext.clone()))),
    )));
    world.add(Arc::new(Sphere::static_new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Some(Arc::new(Lambertian::new(pertext.clone()))),
    )));

    let mut cam = Camera::new(16.0 / 9.0, 400);

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.initialize();

    cam.render(&world, &mut out)?;
    Ok(())
}

pub fn quads() -> io::Result<()> {
    let path = std::path::Path::new("output/book2/image16.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book2/image16.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);

    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        1.0, 0.2, 0.2,
    )))));
    let back_green = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.2, 1.0, 0.2,
    )))));
    let right_blue = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.2, 0.2, 1.0,
    )))));
    let upper_orange = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        1.0, 0.5, 0.0,
    )))));
    let lower_teal = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.2, 0.8, 0.8,
    )))));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut cam = Camera::new(1.0, 400);

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.background = Color::new(0.7, 0.8, 1.0);
    cam.defocus_angle = 0.0;

    cam.initialize();

    cam.render(&world, &mut out)?;
    Ok(())
}

pub fn simple_light() -> io::Result<()> {
    let path = std::path::Path::new("output/book2/image18ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book2/image18.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    world.add(Arc::new(Sphere::static_new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new(pertext.clone()))),
    )));

    world.add(Arc::new(Sphere::static_new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Some(Arc::new(Lambertian::new(pertext.clone()))),
    )));

    let difflight = Arc::new(DiffuseLight::new_from_color(Color::new(4.0, 4.0, 4.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));

    world.add(Arc::new(Sphere::static_new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        Some(difflight),
    )));

    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
    cam.lookat = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.initialize();
    cam.render(&world, &mut out)?;
    Ok(())
}

pub fn cornell_box() -> io::Result<()> {
    let path = std::path::Path::new("output/book3/image6.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book3/image6.ppm").expect("Failed to create file");
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
    let light = Arc::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));

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

    let box1 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut cam = Camera::new(1.0, 600);

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 1000;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.initialize();
    cam.render(&world, &mut out)?;
    Ok(())
}

pub fn cornell_smoke() -> io::Result<()> {
    let path = std::path::Path::new("output/book2/image22.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/book2/image22.ppm").expect("Failed to create file");
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
    let light = Arc::new(DiffuseLight::new_from_color(Color::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(Translate::new(
        Arc::new(RotateY::new(box1, 15.0)),
        Vec3::new(265.0, 0.0, 295.0),
    ));
    let smoke1 = Arc::new(ConstantMedium::new_with_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    ));
    world.add(smoke1);

    let box2 = quad::make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Arc::new(Translate::new(
        Arc::new(RotateY::new(box2, -18.0)),
        Vec3::new(130.0, 0.0, 65.0),
    ));
    let smoke2 = Arc::new(ConstantMedium::new_with_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    ));
    world.add(smoke2);

    let mut cam = Camera::new(1.0, 600);
    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.initialize();
    cam.render(&world, &mut out)?;
    Ok(())
}

fn main() -> io::Result<()> {
    cornell_box()
}
