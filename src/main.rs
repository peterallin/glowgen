use image::{RgbaImage, ImageBuffer, Rgba, DynamicImage};
use std::error::Error;

fn dist2alpha(size: i32, dist: f32) -> u8 {
    if dist > size as f32 / 2.0 
        0
    } else {
        255 - dist as u8 * 15
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let size  = 33;
    let mut buf: RgbaImage = ImageBuffer::new(size as u32, size as u32);
    for x in 0..size  as i32 {
        for y in 0..size as i32 {
            let dist = (((x - size/2).pow(2) + (y - size/2).pow(2)) as f32).sqrt();
            let alpha = dist2alpha(size, dist);
            let p = Rgba([255, 80, 30,  alpha]);
            buf.put_pixel(x as u32, y as u32, p);
        }
    }
    let dyn_image = DynamicImage::ImageRgba8(buf);
    dyn_image.save("glowball.png")?;
    Ok(())
}
