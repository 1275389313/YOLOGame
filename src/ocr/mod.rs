use std::error::Error;
pub mod paddle;
pub use paddle as ocr;

pub trait Ocr {
    fn ocr(&self, img: &image::DynamicImage) -> Result<String, Box<dyn Error>>;
}