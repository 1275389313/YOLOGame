use std::error::Error;

pub use paddleocr as ocr;

pub trait Ocr {
    fn ocr(&self, img: &image::DynamicImage) -> Result<String, Box<dyn Error>>;
}