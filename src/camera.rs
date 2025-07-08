use crate::color::{Color, write_color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, degrees_to_radians};
use crate::vec3::{Point3, Vec3};
use crate::{color, rtweekend};
use std::f64::consts::PI;
use std::io::{self, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pub samples_per_pixel: u32,
    pixel_samples_scale: f64,
    sqrt_spp: u32,
    recip_sqrt_spp: f64,
    pub max_depth: usize,
    pub background: Color,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn ray_color(&self, r: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::default();
        if !world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        let mut pdf_value = 0.0;
        if rec.mat.is_some() {
            let rec_ = rec.clone();
            let color_from_emission = rec.clone().mat.unwrap().emitted(r, &rec, rec.u, rec.v, &rec.p);
            let mut color_from_scatter = Color::new(0.0, 0.0, 0.0);
            let rec__ = rec_.clone();
            if rec_.mat.unwrap().scatter(
                r,
                &rec__,
                &mut attenuation,
                &mut scattered,
                &mut pdf_value,
            ) {
                let on_light = Point3::new(
                    rtweekend::random_double_range(213.0, 343.0),
                    554.0,
                    rtweekend::random_double_range(227.0, 332.0),
                );

                let mut to_light = on_light - rec.p;
                let distance_squared = to_light.length_squared();
                to_light = Vec3::unit_vector(to_light);

                if Vec3::dot(to_light, rec.normal) < 0.0 {
                    return color_from_emission;
                }

                let light_area = (343.0 - 213.0) * (332.0 - 227.0);
                let light_cosine = to_light.y().abs();
                if light_cosine < 0.000001 {
                    return color_from_emission;
                }

                let pdf_value = distance_squared / (light_cosine * light_area);
                let scattered = Ray::new_with_time(rec.p, to_light, r.time());

                let scattering_pdf = rec.mat.unwrap().scattering_pdf(&r, &rec__, &scattered);

                color_from_scatter =
                    attenuation * scattering_pdf * self.ray_color(&scattered, depth - 1, world)
                        / pdf_value;
            }
            return color_from_emission + color_from_scatter;
        }
        Color::new(0.0, 0.0, 0.0)
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        let mut cam = Camera {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            samples_per_pixel: 1,
            pixel_samples_scale: 1.0,
            sqrt_spp: 1,
            recip_sqrt_spp: 1.0,
            max_depth: 0,
            background: Color::new(1.0, 1.0, 1.0),
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::new(1.0, 1.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
            w: Vec3::new(0.0, 1.0, 1.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        };
        cam.initialize();
        cam
    }

    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        //self.samples_per_pixel = 100;
        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as u32;
        self.pixel_samples_scale = 1.0 / (self.sqrt_spp as f64 * self.sqrt_spp as f64);
        self.recip_sqrt_spp = 1.0 / (self.sqrt_spp as f64);
        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = Vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = Vec3::unit_vector(Vec3::cross(self.vup, self.w));
        self.v = Vec3::cross(self.w, self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - self.w * self.focus_dist - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
        //self.max_depth = 10;

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn sample_square(&self) -> Vec3 {
        Vec3::new(
            rtweekend::random_double() - 0.5,
            rtweekend::random_double() - 0.5,
            0.0,
        )
    }

    pub fn defocus_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x() + self.defocus_disk_v * p.y()
    }

    pub fn get_ray(&self, i: usize, j: usize, s_i: u32, s_j: u32) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.x())
            + self.pixel_delta_v * (j as f64 + offset.y());

        let ray_origin = if (self.defocus_angle <= 0.0) {
            self.center
        } else {
            self.defocus_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rtweekend::random_double();
        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn sample_square_stratified(&self, s_i: u32, s_j: u32) -> Vec3 {
        let px = ((s_i as f64 + rtweekend::random_double()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + rtweekend::random_double()) * self.recip_sqrt_spp) - 0.5;
        Vec3::new(px, py, 0.0)
    }

    pub fn render<W: Write>(&self, world: &dyn Hittable, writer: &mut W) -> io::Result<()> {
        writeln!(
            writer,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )?;

        for j in (0..self.image_height) {
            eprint!("\rScanlines remaining: {} ", j);
            io::stderr().flush().unwrap();

            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for s_j in 0..self.sqrt_spp {
                    for s_i in 0..self.sqrt_spp {
                        let r = self.get_ray(i, j, s_i, s_j);
                        pixel_color += self.ray_color(&r, self.max_depth, world);
                    }
                }

                pixel_color *= self.pixel_samples_scale;
                color::write_color(writer, &pixel_color)?;
            }
        }

        eprintln!("\rDone.");
        Ok(())
    }
}
