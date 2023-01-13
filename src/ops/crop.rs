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

pub fn test_crop(
    img: &mut OpImage
) {
    let mut raw = img.clone()
        .into_vec();

    let org_dim = img.dimensions();
    let new_dim = (50, 50);
    let offset = (75, 25);

    println!(
        "\nCropping ({}x{}) to {}x{} at ({}, {})",
        org_dim.0, org_dim.1, new_dim.0, new_dim.1, offset.0, offset.1
    );

    //######### Manual ###########
    let start = Instant::now();
    crop(
        &mut raw,
        org_dim,
        offset,
        new_dim,
    ).unwrap();
    println!("saving: {:?}", start.elapsed());

    //######### Manual ###########
    let write_to = File::create("tmp/cropped.png")
        .unwrap();

    let png = PngEncoder::new_with_quality(
        write_to,
        PNG_COMPRESSION,
        PNG_FILTER
    );
    png.write_image(
        &raw,
        new_dim.0,
        new_dim.1,
        ColorType::Rgba8
    ).unwrap();
    
    println!("cropped: {:?}", start.elapsed());
    
    //######### ImageOps ###########
    let start = Instant::now();
    image::imageops::crop(
        img,
        75, 25,
        50, 50,
    ).to_image().save("tmp/opscropped.png").unwrap();
    
    println!("against: {:?}", start.elapsed());
}

pub fn crop(
    v: &mut Vec<u8>,
    (org_w, org_h): (u32, u32),
    (x_off, y_off): (u32, u32),
    (w, h): (u32, u32)
) -> OpResult<()> {
    if x_off + w  > org_w 
    || y_off + h > org_h {
        Err("Given arguments are not valid. Either axis, offset + length exceed original length.")?;
    }

    let row_top = (y_off) as usize;
    let row_btm = (y_off + h) as usize;

    let ch_pix = 4;//rgba
    let ch_row = (org_w*ch_pix) as usize;
    let ch_lft = (x_off*ch_pix) as usize;
    let ch_rgt = ((x_off + w)*ch_pix) as usize;

    let org_h = org_h as usize;

    for y in 0..org_h {
        let y = org_h - y - 1;

        let up = y*ch_row;

        if y < row_top
        || y >= row_btm {
            v.drain(
                up
                ..
                up + ch_row
            );
        } else {
            v.drain(
                up + ch_rgt
                ..
                up + ch_row
            );
            v.drain(
                up
                ..
                up + ch_lft
            );
        }
    }

    assert_eq!((w*h*ch_pix) as usize, v.len());
    return Ok(());
}