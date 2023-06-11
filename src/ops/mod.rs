use image::codecs::png::{
    CompressionType,
    FilterType
};

use image::Rgba;
use image::ImageBuffer;

pub type OpImage = ImageBuffer<Rgba<u8>, Vec<u8>>;
pub type OpResult<T> = Result<T, Box<dyn std::error::Error>>;

pub enum Kernel {
    K3 ([i32; 9], i32),
    K5 ([i32; 25], i32),
    Kc3 ([i32; 9*CH_LEN], i32),
    Kc5 ([i32; 25*CH_LEN], i32)
}

//CHANGES HERE REFLECT IN PLOT
pub const CH_LEN : usize = 4;
pub const PNG_COMPRESSION : CompressionType = CompressionType::Fast;
pub const PNG_FILTER : FilterType = FilterType::Adaptive;

pub mod crop;
pub mod brit;
pub mod conv;
pub mod doub;
