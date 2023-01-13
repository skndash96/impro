use std::time::Instant;
use std::fs::File;

use image::ColorType;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;

use super::{
    OpResult,
    OpImage,
    CH_LEN,
    PNG_FILTER,
    PNG_COMPRESSION
};

pub fn test_overlay(
    img: &mut OpImage,
    img2: &OpImage
) {
    let dim = img.dimensions();
    let mut raw = img.clone()
        .into_vec();
    
    let dim2 = img2.dimensions();
    let raw2 = img2.clone()
        .into_vec();
    
    //######### Manual ###########
    println!("\nOverlaying");
    let start = Instant::now();
    overlay(
        &mut raw,
        dim,
        &raw2,
        dim2,
        (-50,-50),
        200
    ).unwrap();
    
    println!("saving: {:?}", start.elapsed());

    //######### Saving ###########
    let write_to = File::create("tmp/overlaid.png")
        .unwrap();

    let png = PngEncoder::new_with_quality(
        write_to,
        PNG_COMPRESSION,
        PNG_FILTER
    );

    png.write_image(
        &raw,
        dim.0,
        dim.1,
        ColorType::Rgba8
    ).unwrap();

    println!("overlaid: {:?}", start.elapsed());
    
    //######### ImageOps ###########
    let start = Instant::now();
    image::imageops::overlay(
        img,
        img2,
        25, 25
    );
    img.save("tmp/opsoverlaid.png").unwrap();
    
    println!("against: {:?}", start.elapsed());
}

fn overlay(
    v: &mut Vec<u8>,
    (w, h): (u32, u32),
    v2: &Vec<u8>,
    (w2, h2): (u32, u32),
    (off_x, off_y): (i32, i32),
    a: u8
) -> OpResult<()> {
    assert_eq!(v.len(), (w*h) as usize*CH_LEN);
    assert_eq!(v2.len(), (w2*h2) as usize*CH_LEN);
    
    let (w, h) = (w as i32, h as i32);
    let (w2, h2) = (w2 as i32, h2 as i32);
    
    let chr = w as usize*CH_LEN;
    let chr2 = w2 as usize*CH_LEN;
    
    let r_rng = (if off_y < 0 {
        0..h
    } else {
        off_y..h
    }).into_iter();
    
    let c_rng = (if off_x < 0 {
        0..w
    } else {
        off_x..w
    }).into_iter();
    
    for ri in r_rng {
        if ri >= off_y + h2 -1 {
            break;
        }
        
        for ci in c_rng.clone() {
            if ci >= off_x + w2 -1 {
                break;
            }
            
            let ch_back = chr*ri as usize + CH_LEN*ci as usize;
            let ch2_back = chr2*(ri-off_y) as usize + CH_LEN*(ci-off_x) as usize;
            
            for color_i in 0..CH_LEN {
                let i = ch_back + color_i;
                let i2 = ch2_back + color_i;
                
                v[i] = (//FORMULA at EOF
                    v[i] as f32
                    + (a as f32/255.0) * (v2[i2] as f32 - v[i] as f32)
                ) as u8;
            }
        }
    }
    
    Ok(())
}

/*
# formula
# v[i] = v[i] * (100-a)/100
  + v[f] * a/100
# v[i] = v[i] - v[i]*a/100 + v[f]*a/100
# v[i] = v[i] + a/100 * (v[f] - v[i])
*/