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
    CH_LEN,
    Kernel
};

pub enum KType {
    Reduced,
    Overlap
}

const SHARP_KN : Kernel = Kernel::K3([
    0,-1,0,
    -1,5,-1,
    0,-1,0
], 1);

const RED_FILTER_KN : Kernel = Kernel::Kc3([
    1,0,0,1, 1,0,0,1, 1,0,0,1,
    1,0,0,1, 1,0,0,1, 1,0,0,1,
    1,0,0,1, 1,0,0,1, 1,0,0,1,
], 9);

const PIXEL_KN : Kernel = Kernel::K5([
    4,0,2,0,4,
    0,0,0,0,0,
    2,0,1,0,2,
    0,0,0,0,0,
    4,0,2,0,4
], 25);

const BLUR_KN : Kernel = Kernel::K5([
    0,1,0,1,0,
    1,0,3,0,1,
    0,3,5,3,0,
    1,0,3,0,1,
    0,1,0,1,0
], 25);

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
        //TODO: Can't blur high quality images, Increase Intensity
        &BLUR_KN,
        Some(KType::Overlap)
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
        3.0
    ).save("tmp/opsblurred.png").unwrap();
    
    println!("against: {:?}", start.elapsed());
}

pub fn conv(
    v: &mut Vec<u8>,
    (w, h): (u32, u32),
    kernel: &Kernel,
    k_type: Option<KType>
) -> OpResult<(u32, u32)> {
    let (k, k_div, ch_n) = match kernel {
        Kernel::Kc3(k,d) => (k.to_vec(), d, CH_LEN),
        Kernel::Kc5(k,d) => (k.to_vec(), d, CH_LEN),
        Kernel::K3(k,d) => (k.to_vec(), d, 1),
        Kernel::K5(k,d) => (k.to_vec(), d, 1),
    };
    
    let k_len : usize = k.len();
    let k_side : usize = ((k_len/ch_n) as f32).sqrt() as usize;
    
    assert_eq!(
        v.len(), 
        CH_LEN*(w*h) as usize
    );
    assert_eq!(
        k_side*k_side*ch_n,
        k_len
    );
    
    let (w, h) = (w as usize, h as usize);
    
    let (reduced, overlap) = if let Some(val) = k_type {
        match val {
            KType::Overlap => (true, true),
            KType::Reduced => (true, false)
        }
    } else {
        (false, false)
    };
    
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
    
    let step = if overlap {1} else {k_side};
    
    let ek = k.clone();
    let ek = ek.iter()
        .enumerate()
        .filter(|v| *v.1 != 0)
        .collect::<Vec<(usize, &i32)>>();
    let ek_len = ek.len();
    
    for ri in (0..h).step_by(step) {
        for ci in (0..w).step_by(step) {
            let close = {
                let mut vec : Vec<usize> = Vec::with_capacity(k_len);
                
                for y in 0..k_side {
                    let y = (ri+y).min(h-1);
                    let chr_back = y*chr;
                    
                    for x in 0..k_side {
                        let x = (ci+x).min(w-1);
                        let chc_back = x*CH_LEN;
                        
                        vec.push(chr_back + chc_back);
                    }
                }
                
                vec
            };
            
            let mut pixel : [
                i32; CH_LEN
            ] = [0, 0, 0, 0];
            
            if ch_n == 1 {
                for i in 0..ek_len {
                    let (ki, kv) = ek[i];
                    for ch_i in 0..CH_LEN {
                        pixel[ch_i] += kv * v[close[ki] + ch_i] as i32;
                    }
                }
            } else {
                for i in 0..ek_len {
                    let (ki, kv) = ek[i];
                
                    let ch_i = ki%ch_n;
                    pixel[ch_i] += kv * v[close[ki/ch_n] + ch_i] as i32;
                }
            }
            
            let mut pixel = pixel
                .iter()
                .map(|p| (p/k_div) as u8)
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
        
        let n = if w%k_side == 0 || overlap {0} else {1};
        let d = if overlap {1} else {k_side};
        
        return Ok((
            (w/d +n) as u32,
            (h/d +n) as u32
        ));
    } else {
        return Ok((
            w as u32,
            h as u32
        ));
    }
}