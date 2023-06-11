use super::ops::CH_LEN;

pub mod plot2d;
pub mod canvas;
pub mod node;

pub type Res<T> = Result<T, Box<dyn std::error::Error>>;

pub type Pixel = [u8; CH_LEN];

#[derive(Clone, Copy, Debug)]
pub struct Pos (usize, usize);

impl Pos {
    pub fn to_i32(&self) -> (i32, i32) {
        (self.0 as i32, self.1 as i32)
    }
    pub fn to_f64(&self) -> (f64, f64) {
        (self.0 as f64, self.1 as f64)
    }
    pub fn to_u32(&self) -> (u32, u32) {
        (self.0 as u32, self.1 as u32)
    }
}

fn debug_img(
    v: &Vec<Pixel>,
    Pos (w, h): Pos,
    path: &str
) {
    use std::fs::File;
    
    use image::codecs::png::PngEncoder;
    use image::ImageEncoder;
    use image::ColorType;
    
    use crate::ops::{PNG_COMPRESSION, PNG_FILTER};
    
    let write_to = File::create(format!("tmp/{}", path).as_str())
        .unwrap();

    let png = PngEncoder::new_with_quality(
        write_to,
        PNG_COMPRESSION,
        PNG_FILTER
    );

    png.write_image(
        v.flatten(),
        w as u32,
        h as u32,
        ColorType::Rgba8
    ).unwrap();
}