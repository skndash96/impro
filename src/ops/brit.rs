use std::time::Instant;
use std::fs::File;

use image::ColorType;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;

use super::{
    OpImage,
    OpResult,
    PNG_COMPRESSION,
    PNG_FILTER
};

pub fn test_brighten(
    img: &OpImage
) {
    let mut raw = img.clone()
        .into_vec();

    let org_dim = img.dimensions();

    println!("\nBrightening");

    //######### Manual ###########
    let start = Instant::now();
    let _ = brighten(
        &mut raw,
        125i8 //Range -128..127
    );
    println!("saving: {:?}", start.elapsed());

    //######### Saving ###########
    let write_to = File::create("tmp/brightened.png")
        .unwrap();

    let png = PngEncoder::new_with_quality(
        write_to,
        PNG_COMPRESSION,
        PNG_FILTER
    );

    png.write_image(
        &raw,
        org_dim.0,
        org_dim.1,
        ColorType::Rgba8
    ).unwrap();

    println!("brightened: {:?}", start.elapsed());
    
    //######### ImageOps ###########
    let start = Instant::now();
    image::imageops::brighten(
        img,
        100
    ).save("tmp/opsbrightened.png").unwrap();
    
    println!("against: {:?}", start.elapsed());
}

pub fn brighten(
    v: &mut Vec<u8>,
    pw: i8
) -> OpResult<()> {
    let factor = 1.0 + (pw as f32) / 128.0;

    println!("factor: {}", factor);

    for c in v.iter_mut() {
        *c = (*c as f32 * factor) as u8;
    }
    
    Ok(())
}