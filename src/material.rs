use crate::onb::ONB;
use crate::pdf::{CosinePdf, Pdf};
use crate::rtweekend::random_double;
use crate::texture::{CheckerTexture, SolidColor, Texture};
use crate::vec3::{Point3, Vec3};
use crate::{color::Color, hittable::HitRecord, ray::Ray, rtweekend};
use rand::random;
use std::cmp::min;
use std::f64;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct scatter_record {
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn Pdf>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl scatter_record {
    pub fn default() -> Self {
        Self {
            attenuation: Color::new(0.0, 0.0, 0.0),
            pdf_ptr: None,
            skip_pdf: true,
            skip_pdf_ray: Ray::new(Vec3::zero(), Vec3::zero()),
        }
    }
}

pub struct EmptyMaterial;

impl Material for EmptyMaterial {}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut scatter_record) -> bool {
        false
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, r: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut scatter_record) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(CosinePdf::new(rec.normal)));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = Vec3::dot(rec.normal, Vec3::unit_vector(scattered.direction()));
        if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut scatter_record) -> bool {
        let reflected = Vec3::reflect(r_in.direction(), rec.normal);
        let reflected = Vec3::unit_vector(reflected) + Vec3::random_unit_vector() * self.fuzz;
        srec.attenuation = self.albedo;
        srec.pdf_ptr = None;
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::new_with_time(rec.p, reflected, r_in.time());
        true
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut scatter_record) -> bool {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.pdf_ptr = None;
        srec.skip_pdf = true;

        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, ri) > rtweekend::random_double() {
                Vec3::reflect(unit_direction, rec.normal)
            } else {
                Vec3::refract(unit_direction, rec.normal, ri)
            };

        srec.skip_pdf_ray = Ray::new_with_time(rec.p, direction, r_in.time());
        true
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new_from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn new_from_color(c: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            Color::new(0.0, 0.0, 0.0)
        } else {
            self.tex.value(u, v, p)
        }
    }
}

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_with_color(color: Color) -> Self {
        use crate::texture::SolidColor;
        Self {
            tex: Arc::new(SolidColor::new(color)),
        }
    }
    pub fn new_with_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut scatter_record) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(CosinePdf::new(rec.normal)));
        srec.skip_pdf = false;
        true
    }
    fn scattering_pdf(&self, r: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.25 / PI
    }
}
