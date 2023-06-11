#![feature(iter_array_chunks)]
#![feature(raw_slice_split)]
#![feature(slice_ptr_get)]
#![feature(slice_flatten)]
#![feature(result_option_inspect)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod ops;
mod plot;

use image::io::Reader;

fn main() {
    test_plot();
    test_img();
}

fn test_plot() {
    plot::plot2d::test();
}

fn test_img() {
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
    ops::doub::test_overlay(&mut img, &img2);
}