use std::error::Error;
use std::path::Path;
use image::{DynamicImage, GenericImageView, Luma, GrayImage};
use imageproc::{rect::Rect};
use ndarray::{Array, ArrayBase, Dim, OwnedRepr};
use ort::{ExecutionProviderDispatch, inputs, Session};


pub struct Det {
    model: Session,
}

impl Det {
    pub fn from_file(model_path: impl AsRef<Path>,ort_path:&str) -> Result<Self, Box<dyn Error>> {
        ort::init_from(ort_path).commit()?;

        let model = ort::Session::builder()?.commit_from_file(model_path)?;
        Ok(Self { model })
    }

    pub fn from_memory(model_content: &[u8]) -> Result<Self, Box<dyn Error>> {
        let model = ort::Session::builder()?.commit_from_memory(model_content)?;
        Ok(Self { model })
    }

    pub fn from_memory_with_providers(model_content: &[u8], execution_providers: impl IntoIterator<Item=ExecutionProviderDispatch>) -> Result<Self, Box<dyn Error>> {
        let model = ort::Session::builder()?.with_execution_providers(execution_providers)?.commit_from_memory(model_content)?;
        Ok(Self { model })
    }


    pub fn find_text_rect(&self, img: &DynamicImage) -> Result<Vec<Rect>, Box<dyn Error>> {
        let (w, h) = img.dimensions();
        let pad_w = {
            if w % 32 == 0 {
                w
            } else {
                w + 32 - (w % 32)
            }
        };
        let pad_h = {
            if h % 32 == 0 {
                h
            } else {
                h + 32 - (h % 32)
            }
        };

        let mut input = Array::zeros((1, 3, pad_h as usize, pad_w as usize));
        for pixel in img.pixels() {
            let x = pixel.0 as _;
            let y = pixel.1 as _;
            let [r, g, b, _] = pixel.2.0;
            input[[0, 0, y, x]] = (((r as f32) / 255.) - 0.485) / 0.229;
            input[[0, 1, y, x]] = (((g as f32) / 255.) - 0.456) / 0.224;
            input[[0, 2, y, x]] = (((b as f32) / 255.) - 0.406) / 0.225;
        }
        let output = self.run_model(&input, img.width(), img.height())?;
        Ok(self.find_box(&output))
    }

    pub fn find_text_img(&self, img: &DynamicImage) -> Result<Vec<DynamicImage>, Box<dyn Error>> {
        Ok(self.find_text_rect(&img)?
            .iter()
            .map(|r| img.crop_imm(r.left() as u32, r.top() as u32, r.width(), r.height()))
            .collect()
        )
    }


    fn run_model(&self, input: &ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>>, width: u32, height: u32) -> Result<GrayImage, Box<dyn Error>> {
        let pad_h = {
            if height % 32 == 0 {
                height
            } else {
                height + 32 - (height % 32)
            }
        };
        let outputs = self.model.run(inputs! {"x" => input.view()}?)?;
        let output = outputs.iter().next().ok_or("no result")?.1;
        let output = output
            .try_extract_tensor::<f32>()?
            .view()
            .t()
            .to_owned();
        let output: Vec<_> = output.iter().collect();
        let img = image::ImageBuffer::from_fn(width, height, |x, y| {
            Luma([(*output[(x * pad_h + y) as usize] * 255.0).min(255.0) as u8])
        });
        Ok(img)
    }

    fn find_box(&self, img: &GrayImage) -> Vec<Rect> {
        let (w, h) = img.dimensions();
        imageproc::contours::find_contours_with_threshold::<u32>(img, 200)
            .into_iter()
            .filter(|x| x.parent.is_none())
            .map(|x| x.points)
            .map(|x| {
                let (x_min, x_max, y_min, y_max) = x.into_iter()
                    .fold(None, |ret, p| {
                        match ret {
                            None => Some((p.x, p.x, p.y, p.y)),
                            Some((x_min, x_max, y_min, y_max)) => {
                                Some((x_min.min(p.x), x_max.max(p.x), y_min.min(p.y), y_max.max(p.y)))
                            }
                        }
                    })?;
                let width = (x_max - x_min) as u32;
                let height = (y_max - y_min) as u32;
                if width <= 5 || height <= 5 {
                    return None;
                }
                Some(Rect::at(x_min as i32, y_min as i32).of_size(width, height))
            })
            .filter_map(|x| x)
            .map(|x| {
                Rect::at((x.left() - 8_i32).max(0), (x.top() - 8_i32).max(0))
                    .of_size((x.width() + 8 * 2).min(w), (x.height() + 8 * 2).min(h))
            })
            .collect()
    }
}
