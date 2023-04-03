use crate::error::Error;
use image::{self, GenericImageView, ImageOutputFormat, Rgb};
use regex;
use std::io::Cursor;

pub fn generate_wallpaper(
    img: Vec<u8>,
    foreground: image::Rgb<u8>,
    background: image::Rgb<u8>,
    size: (usize, usize),
    padding: (usize, usize, usize, usize),
) -> Vec<u8> {
    let img = image::load_from_memory(&img).unwrap();

    let (padding_top, padding_right, padding_bottom, padding_left) = padding;

    let source_width = usize::min(size.0 - padding_left - padding_right, img.width() as usize);
    let source_height = usize::min(size.1 - padding_top - padding_bottom, img.height() as usize);

    let img = img.resize(
        source_width as u32,
        source_height as u32,
        image::imageops::FilterType::Triangle,
    );

    let source_width = img.width() as usize;
    let source_height = img.height() as usize;

    let target = image::ImageBuffer::from_fn(size.0 as u32, size.1 as u32, |x, y| {
        // check if x,y is in the image when centered in the target
        let source_x = x as i32 - (size.0 as i32 - source_width as i32) / 2;
        let source_y = y as i32 - (size.1 as i32 - source_height as i32) / 2;

        let pixel = if source_x >= 0
            && source_x < source_width as i32
            && source_y >= 0
            && source_y < source_height as i32
        {
            img.get_pixel(source_x as u32, source_y as u32)
        } else {
            return background;
        };

        // Get the value and the saturation of the pixel
        let v = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
        let s = (pixel[0] as f32 - pixel[1] as f32).abs()
            + (pixel[1] as f32 - pixel[2] as f32).abs()
            + (pixel[2] as f32 - pixel[0] as f32).abs();

        // If the pixel is saturated, use the original color, otherwise use the foreground or background color
        let v = v / 255.0;
        let s = s / 255.0;
        Rgb([
            ((foreground[0] as f32 * (1.0 - v) + background[0] as f32 * v) * (1.0 - s)
                + pixel[0] as f32 * s) as u8,
            ((foreground[1] as f32 * (1.0 - v) + background[1] as f32 * v) * (1.0 - s)
                + pixel[1] as f32 * s) as u8,
            ((foreground[2] as f32 * (1.0 - v) + background[2] as f32 * v) * (1.0 - s)
                + pixel[2] as f32 * s) as u8,
        ])
    });

    let mut bytes: Vec<u8> = vec![];
    let mut writer = Cursor::new(&mut bytes);
    target
        .write_to(&mut writer, ImageOutputFormat::Png)
        .unwrap();

    bytes
}

fn parse_rgb(color: &str) -> Result<image::Rgb<u8>, Error> {
    let re = regex::Regex::new(r"^#?([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})$|^#?([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])$").unwrap();
    let caps = re
        .captures(color)
        .ok_or_else(|| Error::ColorParseError(color.to_string()))?;

    let r = caps
        .get(1)
        .or(caps.get(4))
        .ok_or_else(|| Error::ColorParseError(color.to_string()))?;
    let g = caps
        .get(2)
        .or(caps.get(5))
        .ok_or_else(|| Error::ColorParseError(color.to_string()))?;
    let b = caps
        .get(3)
        .or(caps.get(6))
        .ok_or_else(|| Error::ColorParseError(color.to_string()))?;

    let mut r = u8::from_str_radix(r.as_str(), 16)
        .map_err(|_err| Error::ColorParseError(color.to_string()))?;
    let mut g = u8::from_str_radix(g.as_str(), 16)
        .map_err(|_err| Error::ColorParseError(color.to_string()))?;
    let mut b = u8::from_str_radix(b.as_str(), 16)
        .map_err(|_err| Error::ColorParseError(color.to_string()))?;

    if caps.get(4).is_some() {
        r *= 17;
        g *= 17;
        b *= 17;
    }

    Ok(image::Rgb([r, g, b]))
}

pub fn generate_wallpaper_hex(
    img: Vec<u8>,
    foreground: &str,
    background: &str,
    size: (usize, usize),
    padding: (usize, usize, usize, usize),
) -> Result<Vec<u8>, Error> {
    // Convert the hex strings to rgb values
    let foreground = parse_rgb(foreground)?;
    let background = parse_rgb(background)?;

    Ok(generate_wallpaper(
        img, foreground, background, size, padding,
    ))
}
