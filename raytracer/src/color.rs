use crate::rtweekend;
use crate::vec3::Color1;
use image::RgbImage;
use rtweekend::clamp;

/// the multi-sample write_color() function
pub fn write_color(color: &Color1, img: &mut RgbImage, i: usize, j: usize, samples_per_pixel: i32) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / (samples_per_pixel as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let pixel_color = [
        (clamp(r, 0.0, 0.999) * 256.0) as u8,
        (clamp(g, 0.0, 0.999) * 256.0) as u8,
        (clamp(b, 0.0, 0.999) * 256.0) as u8,
    ];
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb(pixel_color);
    // Write the translated [0,255] value of each color component.
}
