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

use color::{Color, write_color};
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use ray::Ray;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use vec3::{Point3, Vec3};

pub fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64) as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let file = File::create("output/book1/image2.ppm").expect("Failed to create file");
    let mut out = BufWriter::new(file);

    writeln!(out, "P3")?;
    writeln!(out, "{} {}", image_width, image_height)?;
    writeln!(out, "255")?;

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);

        for i in 0..image_width {
            // 假设 pixel_center、ray_direction、camera_center、pixel_delta_u、pixel_delta_v 是已定义的
            let pixel_center =
                pixel00_loc + pixel_delta_u * (i as f64) + pixel_delta_v * (j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center.clone(), ray_direction);

            let pixel_color = ray_color(&ray);
            write_color(&mut out, &pixel_color)?;
        }
    }
    Ok(())
}
