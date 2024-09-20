use std::{path::Path, borrow::Cow};
use std::error::Error;
use image::{DynamicImage, GenericImageView};
use ndarray::{Array, ArrayBase, Dim, OwnedRepr, Axis, s};
use ort::{ inputs, Session};


pub struct Rec {
    model: Session,
    keys: Vec<char>,
    score: f32
}

impl Rec {

    pub fn from_file(model_path: impl AsRef<Path>, keys_path: impl AsRef<Path>,socre:f32) -> Result<Self, Box<dyn Error>> {
        let model = ort::Session::builder()?.commit_from_file(model_path)?;
        let keys = " ".chars()
            .chain(std::fs::read_to_string(keys_path)?
            .chars()
            .filter(|x| *x != '\n'))
            .chain(" ".chars())
            .collect();
        Ok(Self { model, keys, score: socre })
    }


    pub fn predict_char_score(&self, img: &DynamicImage) -> Result<Vec<(char, f32)>, Box<dyn Error>> {
        let (w, h) = img.dimensions();
        let img = if h <= 48 {
            Cow::Borrowed(img)
        } else {
            Cow::Owned(img.resize_exact(w, 48, image::imageops::FilterType::CatmullRom))
        };
        let mut input = Array::zeros((1, 3, 48, w as usize));
        for pixel in img.pixels() {
            let x = pixel.0 as _;
            let y = pixel.1 as _;
            let [r, g, b, _] = pixel.2 .0;
            input[[0, 0, y, x]] = (((r as f32) / 255.) - 0.5) / 0.5;
            input[[0, 1, y, x]] = (((g as f32) / 255.) - 0.5) / 0.5;
            input[[0, 2, y, x]] = (((b as f32) / 255.) - 0.5) / 0.5;
        }
        let output = self.run_model(&input)?;
        Ok(output)
    }

    pub fn predict_str(&self, img: &DynamicImage) -> Result<String, Box<dyn Error>> {
        let ret = self.predict_char_score(img)?;
        Ok(ret.into_iter().map(|x| x.0).collect())
    }

    fn run_model(&self, input: &ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>>) -> Result<Vec<(char, f32)>, Box<dyn Error>>{
        let outputs = self.model.run(inputs!["x" => input.view()]?)?;
        let output = outputs.iter().next().ok_or("no result")?.1;
        let output = output.try_extract_tensor::<f32>()?;
        let output = output.view();
        let output = output.slice(s![0, .., ..]);
        let output = output.axis_iter(Axis(0))
            .filter_map(|x| {
                x.iter().copied().enumerate().max_by(|(_, x), (_, y)|{
                    x.total_cmp(y)
                })
            })
            .filter(|(index, score)|{
                *index != 0 && *score > self.score
            })
            .filter_map(|(index, score)|{
                self.keys.get(index).map(|x| (*x, score))
            })
            .collect();
        Ok(output)
    }
}
