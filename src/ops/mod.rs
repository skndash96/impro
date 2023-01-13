use image::codecs::png::{
    CompressionType,
    FilterType
};

use image::Rgba;
use image::ImageBuffer;

pub type OpImage = ImageBuffer<Rgba<u8>, Vec<u8>>;
pub type OpResult<T> = Result<T, Box<dyn std::error::Error>>;

pub const CH_LEN : usize = 4;

pub mod crop;
pub mod brit;
pub mod conv;
pub mod doub;

pub const PNG_COMPRESSION : CompressionType = CompressionType::Fast;
pub const PNG_FILTER : FilterType = FilterType::Adaptive;