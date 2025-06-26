use crate::vec3::Vec3;
use crate::interval::Interval;
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
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();
    
    let intensity = Interval::new(0.000, 0.999);
    
    let r_byte = (256.0 * intensity.clamp(r)) as u8;
    let g_byte = (256.0 * intensity.clamp(g)) as u8;
    let b_byte = (256.0 * intensity.clamp(b)) as u8;
    
    writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)
}
