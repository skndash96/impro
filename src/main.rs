#![feature(iter_array_chunks)]
#![feature(raw_slice_split)]
#![feature(slice_ptr_get)]
#![allow(dead_code)]

mod ops;

use image::io::Reader;

fn main() {
    let img_path = "pfp.jpg";
    
    let reader = Reader::open(img_path)
        .unwrap();

    let mut img = reader
        .decode()
        .unwrap()
        .to_rgba8();
    let img2 = img.clone();
    
    ops::brit::test_brighten(&img);
    ops::crop::test_crop(&mut img);
    ops::conv::test_blur(&img);
    ops::doub::test_overlay(
        &mut img,
        &img2
    );
}