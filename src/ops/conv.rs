use std::time::Instant;
use std::fs::File;

use image::ColorType;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;

use super::{
    OpImage,
    OpResult,
    PNG_COMPRESSION,
    PNG_FILTER,
    CH_LEN
};

pub fn test_blur(
    img: &OpImage
) {
    let mut raw = img.clone()
        .into_vec();

    let org_dim = img.dimensions();

    //######### Manual ###########
    println!("\nBlurring");
    let start = Instant::now();
    let new_dim = conv(
        &mut raw,
        org_dim,
        vec![
            4,0,2,0,4,
            0,0,0,0,0,
            2,0,1,0,2,
            0,0,0,0,0,
            4,0,2,0,4
        ],
        Some(false)
    ).unwrap();
    println!("saving: {:?}", start.elapsed());

    //######### Saving ###########
    let write_to = File::create("tmp/blurred.png")
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

    println!("blurred: {:?}", start.elapsed());
    
    //######### ImageOps ###########
    let start = Instant::now();
    image::imageops::blur(
        img,
        1.0
    ).save("tmp/opsblurred.png").unwrap();
    
    println!("against: {:?}", start.elapsed());
}

pub fn conv(
    v: &mut Vec<u8>,
    (w, h): (u32, u32),
    k: Vec<i32>,
    reduced: Option<bool>
) -> OpResult<(u32, u32)> {
    let k_len : usize = k.len();
    let k_side : usize = (k_len as f32).sqrt() as usize;
    
    //length of v equals width times height
    assert_eq!(
        v.len(), 
        CH_LEN*(w*h) as usize
    );
    
    // kernel is a square
    assert_eq!(
        k_side*k_side,
        k_len
    );
    
    let div = k
        .clone()
        .iter()
        .sum::<i32>();
    
    let (w, h) = (w as usize, h as usize);
    
    let (crit_ci, w_ex) = {
        let bal = w % k_side;
        (w-bal, bal)
    };
    let (crit_ri, h_ex) = {
        let bal = h % k_side;
        (h-bal, bal)
    };
    
    let reduced = reduced
       .unwrap_or(true);
    let mut rv : Vec<u8> = if reduced {
            Vec::with_capacity(
                (w/k_side + 1)
                *(h/k_side + 1)
                *CH_LEN
            )
        } else {
            Vec::with_capacity(0)
        };
    
    let chr = CH_LEN*w;
    
    for ri in (0..h).step_by(k_side) {
        for ci in (0..w).step_by(k_side) {
            let close = {
                let mut vec : Vec<usize> = Vec::with_capacity(k_len);
                
                for y in 0..k_side {
                    let y = if ri == crit_ri
                        && y >= h_ex {
                            h_ex-1
                        } else {
                            y
                        };
                    
                    let chr_back = (ri+y)*chr;
                    
                    for x in 0..k_side {
                        let x = if ci == crit_ci
                            && x >= w_ex {
                                w_ex-1
                            } else {
                                x
                            };
                        let chc_back = (ci+x)*CH_LEN;
                        
                        vec.push(chr_back + chc_back);
                    }
                }
                
                vec
            };
            
            let mut pixel : [
                i32; CH_LEN
            ] = [0, 0, 0, 0];
            
            for ki in 0..k_len {
                for color_i in 0..CH_LEN {
                    pixel[color_i] += k[ki] * v[
                        close[ki] + color_i
                    ] as i32;
                }
            }
            
            let mut pixel = pixel
                .iter()
                .map(|p| (p/div) as u8)
                .collect::<Vec<u8>>();
            
            if reduced {
                rv.append(&mut pixel);
            } else {
                for pi in close {
                    v.splice(
                        pi..pi+CH_LEN,
                        pixel.clone()
                    );
                }
            }
        }
    }
    
    if reduced {
        *v = rv;
        return if w_ex == 0 {
            Ok((
                (w/k_side) as u32,
                (h/k_side) as u32
            ))
        } else {
            Ok((
                (w/k_side+1) as u32,
                (h/k_side+1) as u32
            ))
        };
    } else {
        return Ok((
            w as u32,
            h as u32
        ));
    }
}