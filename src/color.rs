use crate::interval::Interval;
use crate::vec3::Vec3;
use std::io::{self, Write};
pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    if r != r {
        println!("uuu");
        r = 0.0;
    }
    if g != g {
        g = 0.0;
    }
    if b != b {
        b = 0.0;
    }

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);

    let r_byte = (256.0 * intensity.clamp(r)) as u8;
    let g_byte = (256.0 * intensity.clamp(g)) as u8;
    let b_byte = (256.0 * intensity.clamp(b)) as u8;

    writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)
}
