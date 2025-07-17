use crate::color::Color;
use crate::perlin::Perlin;
use crate::rtw_stb_image::RtwImage;
use crate::vec3::Point3;
use std::sync::Arc;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        SolidColor { albedo }
    }

    pub fn _from_rgb(red: f64, green: f64, blue: f64) -> Self {
        SolidColor {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn _new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn _from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self::_new(
            scale,
            Arc::new(SolidColor::new(c1)),
            Arc::new(SolidColor::new(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (self.inv_scale * p.x).floor() as i32;
        let y_integer = (self.inv_scale * p.y).floor() as i32;
        let z_integer = (self.inv_scale * p.z).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        //let mut image = RtwImage::default();
        //image.load(filename);
        Self {
            image: RtwImage::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Point3) -> Color {
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        u = u.clamp(0.0, 1.0);
        v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;

        let pixel = self.image.pixel_data(i, j);
        let scale = 1.0 / 255.0;

        Color::new(
            scale * pixel[0] as f64,
            scale * pixel[1] as f64,
            scale * pixel[2] as f64,
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn _new(scale: f64) -> Self {
        Self {
            noise: Perlin::_new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let scaled_p = *p * self.scale;
        Color::new(0.5, 0.5, 0.5) * (1.0 + (scaled_p.z() + 10.0 * self.noise.turb(*p, 7)).sin())
    }
}
