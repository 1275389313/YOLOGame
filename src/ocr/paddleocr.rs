use std::error::Error;
use image::DynamicImage;
use paddleocr::*;

pub struct PaddleOcr {
    d: Det,
    r: Rec,
}
impl PaddleOcr {
    pub fn new() -> Self {
        let det = Det::from_file("./extra/ch_PP-OCRv4_det_infer.onnx", "./extra/onnxruntime192.dll").expect("./extra/ch_PP-OCRv4_det_infer.onnx");
        let rec = Rec::from_file(
            "./extra/ch_PP-OCRv4_rec_infer.onnx",
            "./extra/ppocr_keys_v1.txt",
            0.8,
        ).expect("not ./extra/ch_PP-OCRv4_rec_infer.onnx or /extra/ppocr_keys_v1.txt");
        PaddleOcr {
            d: det,
            r: rec,
        }
    }
}

impl super::Ocr for PaddleOcr {
    fn ocr(&self, img: &DynamicImage) -> Result<String, Box<dyn Error>> {
        let mut result = String::new();
        for sub in self.d.find_text_img(&img)? {
            result.push_str(&self.r.predict_str(&sub)?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::ocr::Ocr;
    use crate::ocr::paddleocr::PaddleOcr;

    #[test]
    fn orc() {
        let ocr = PaddleOcr::new();
        let img = image::open("E:/project/rust_project/YOLOv8-ONNXRuntime-Rust/extra/22.jpg").unwrap();
        println!("{:?}", ocr.ocr(&img))
    }
}
