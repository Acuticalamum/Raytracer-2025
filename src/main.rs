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

pub fn final_scene() -> io::Result<()> {
    let path = std::path::Path::new("output/final.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let file = File::create("output/final.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);

    let mut world = HittableList::new();

    let gray = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.5, 0.5, 0.5,
    )))));

    let dark_gray = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.15, 0.15, 0.15,
    )))));

    let kachi = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.8, 0.52, 0.23,
    )))));

    let light = Arc::new(DiffuseLight::new_from_color(Color::new(18.0, 18.0, 18.0)));
    let background = Arc::new(DiffuseLight::new_from_color(Color::new(3.0, 3.0, 3.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(50.0, 0.0, 0.0),
        Vec3::new(290.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        //gray.clone(),
        Arc::new(Lambertian::new(Arc::new(ImageTexture::new("backwall.png")))),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, 100.0),
        gray.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 100.0),
        Vec3::new(290.0, 0.0, 0.0),
        gray.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(50.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, 100.0),
        Vec3::new(290.0, 0.0, 0.0),
        gray.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(340.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 100.0),
        Vec3::new(0.0, 100.0, 0.0),
        gray.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(170.0, 97.0, 0.0),
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 3.0),
        light.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(170.0, 97.0, 3.0),
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 3.0, 0.0),
        light.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(170.0, 100.0, 0.0),
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::new(0.0, -3.0, 0.0),
        light.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(220.0, 100.0, 0.0),
        Vec3::new(0.0, -3.0, 0.0),
        Vec3::new(0.0, 0.0, 3.0),
        light.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(50.0, 0.0, 400.0),
        Vec3::new(290.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        background.clone(),
    )));

    let box1 = quad::make_box(
        Point3::new(339.0, 0.0, 0.0),
        Point3::new(311.0, 90.0, 30.0),
        kachi.clone(),
    );

    world.add(box1);

    let box1image = Arc::new(Quad::new(
        Point3::new(311.0, 0.0, 30.0),
        Vec3::new(28.0, 0.0, 0.0),
        Vec3::new(0.0, 90.0, 0.0),
        Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
            "box1image.jpg",
        )))),
    ));
    world.add(box1image);

    let box2 = quad::make_box(
        Point3::new(281.0, 0.0, 0.0),
        Point3::new(309.0, 40.0, 27.0),
        kachi.clone(),
    );

    world.add(box2);

    let box2image = Arc::new(Quad::new(
        Point3::new(281.0, 0.0, 27.0),
        Vec3::new(28.0, 0.0, 0.0),
        Vec3::new(0.0, 40.0, 0.0),
        Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
            "box2image.jpg",
        )))),
    ));
    world.add(box2image);

    let box3 = quad::make_box(
        Point3::new(61.0, 0.0, 0.0),
        Point3::new(90.0, 30.0, 30.0),
        kachi.clone(),
    );

    world.add(box3);

    let box3image = Arc::new(Quad::new(
        Point3::new(61.0, 0.0, 30.0),
        Vec3::new(29.0, 0.0, 0.0),
        Vec3::new(0.0, 30.0, 0.0),
        Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
            "box3image.jpg",
        )))),
    ));
    world.add(box3image);

    let box4 = quad::make_box(
        Point3::new(78.0, 0.0, 40.0),
        Point3::new(102.0, 22.0, 77.0),
        kachi.clone(),
    );

    let box4 = Arc::new(RotateY::new(box4, 20.0));

    world.add(box4);

    let box5 = quad::make_box(
        Point3::new(102.0, 22.0, 22.0),
        Point3::new(117.0, 40.0, 42.0),
        kachi.clone(),
    );

    world.add(box5);

    let box5image = Arc::new(Quad::new(
        Point3::new(102.0, 22.0, 42.0),
        Vec3::new(15.0, 0.0, 0.0),
        Vec3::new(0.0, 18.0, 0.0),
        Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
            "box5image.jpg",
        )))),
    ));
    world.add(box5image);

    let board1 = quad::make_box(
        Point3::new(140.0, 0.0, 5.0),
        Point3::new(240.0, 0.5, 55.0),
        kachi.clone(),
    );
    world.add(board1);

    let board2 = quad::make_box(
        Point3::new(130.0, 0.5, 15.0),
        Point3::new(225.0, 1.2, 65.0),
        dark_gray.clone(),
    );
    world.add(board2);

    let box6 = quad::make_box(
        Point3::new(248.0, 0.0, 13.0),
        Point3::new(270.0, 15.0, 35.5),
        kachi.clone(),
    );

    world.add(box6);

    let box6image = Arc::new(Quad::new(
        Point3::new(248.0, 0.0, 35.5),
        Vec3::new(22.0, 0.0, 0.0),
        Vec3::new(0.0, 15.0, 0.0),
        Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
            "box6image.jpg",
        )))),
    ));
    world.add(box6image);

    let clue1 = Arc::new(Quad::new(
        Point3::new(172.0, 23.5, 0.01),
        Vec3::new(17.5, 0.0, 0.0),
        Vec3::new(0.0, 24.5, 0.0),
        Arc::new(Lambertian::new(Arc::new(ImageTexture::new("clue1.jpg")))),
    ));
    world.add(clue1);

    let clue3 = Arc::new(Quad::new(
        Point3::new(200.0, 26.5, 0.01),
        Vec3::new(17.5, 0.0, 0.0),
        Vec3::new(0.0, 24.5, 0.0),
        Arc::new(Lambertian::new(Arc::new(ImageTexture::new("clue3.jpg")))),
    ));
    world.add(clue3);

    let clue6 = Arc::new(Quad::new(
        Point3::new(191.0, 10.5, 0.01),
        Vec3::new(17.5, 0.0, 0.0),
        Vec3::new(0.0, 24.5, 0.0),
        Arc::new(Lambertian::new(Arc::new(ImageTexture::new("clue6.jpg")))),
    ));
    world.add(clue6);

    let whiteboard = Arc::new(obj::load_obj_model("objects/whiteboard.obj", 35.0));
    let whiteboard = Arc::new(Translate::new(whiteboard, Vec3::new(130.0, 60.0, 3.0)));

    world.add(whiteboard);

    let whiteboard_content = Arc::new(Quad::new(
        Point3::new(95.0, 42.5, 3.0),
        Vec3::new(70.0, 0.0, 0.0),
        Vec3::new(0.0, 35.0, 0.0),
        Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
            "Its_Raytracer!!!!.jpg",
        )))),
    ));
    world.add(whiteboard_content);

    let computer = Arc::new(obj::load_obj_model("objects/computer.obj", 1.0));
    let computer = Arc::new(RotateY::new(computer, 270.0));
    let computer = Arc::new(Translate::new(computer, Vec3::new(252.0, 20.0, 53.0)));

    world.add(computer);

    let grumble = Arc::new(Sphere::static_new(
        Point3::new(70.0, 15.0, 60.0),
        15.0,
        Some(Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
            "grumble.jpg",
        ))))),
    ));

    world.add(grumble);

    let bloodywolf = Arc::new(Sphere::static_new(
        Point3::new(325.0, 15.0, 50.0),
        15.0,
        Some(Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
            "bloodywolf.jpg",
        ))))),
    ));

    world.add(bloodywolf);

    let magiczc = Arc::new(Sphere::static_new(
        Point3::new(85.0, 9.0, 70.0),
        9.0,
        Some(Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
            "magiczc.png",
        ))))),
    ));

    world.add(magiczc);

    let sphere = Arc::new(Sphere::static_new(
        Point3::new(304.0, 10.0, 48.0),
        10.0,
        Some(Arc::new(Dielectric::new(1.8))),
    ));

    world.add(sphere);

    let anon = Arc::new(Sphere::static_new(
        Point3::new(153.0, 17.0, 43.0),
        17.0,
        Some(Arc::new(DiffuseLight::new_from_texture(Arc::new(
            ImageTexture::new("anon.png"),
        )))),
    ));

    world.add(anon);

    let metal = Arc::new(Sphere::static_new(
        Point3::new(59.0, 6.0, 79.0),
        6.0,
        Some(Arc::new(Metal::new(Color::new(0.6, 0.9, 0.6), 0.2))),
    ));

    world.add(metal);

    let empty_material: Arc<dyn Material> = Arc::new(material::EmptyMaterial);

    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        Point3::new(170.0, 97.0, 0.0),
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 3.0),
        empty_material.clone(),
    )));

    lights.add(Arc::new(Quad::new(
        Point3::new(170.0, 97.0, 3.0),
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(0.0, 3.0, 0.0),
        empty_material.clone(),
    )));

    lights.add(Arc::new(Quad::new(
        Point3::new(50.0, 0.0, 400.0),
        Vec3::new(290.0, 0.0, 0.0),
        Vec3::new(0.0, 100.0, 0.0),
        empty_material.clone(),
    )));

    let lights = Arc::new(lights);

    let mut cam = Camera::new(2.0, 600);

    cam.aspect_ratio = 2.5;
    cam.image_width = 2400;
    cam.samples_per_pixel = 2000;
    cam.max_depth = 50;
    cam.background = Color::new(0.18, 0.18, 0.18);

    cam.vfov = 30.0;
    cam.lookfrom = Point3::new(195.0, 50.0, 300.0);
    cam.lookat = Point3::new(195.0, 50.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.initialize();
    cam.render(&world, lights, &mut out)?;

    Ok(())
}

fn main() -> io::Result<()> {
    final_scene()
}
