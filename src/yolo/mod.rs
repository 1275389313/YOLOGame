pub mod model;
mod ort_backend;
mod yolo_result;
mod lib;

pub  use ort_backend::OrtEP as deviceType;

pub fn yolo_run(device_type:&str) -> Result<model::YOLOv8, Box<dyn std::error::Error>> {
    let mut dt = deviceType::Cpu;
    if let Some(_) = device_type.find("GPU") {
        dt = deviceType::Cuda(1);
    }
    //部分配置写死
    let  model = model::YOLOv8::new("./best.onnx".to_string(), 0.40,dt)?;
    // model.summary(); // model info
    Ok(model)
}

#[cfg(test)]
mod test{
    use crate::yolo;

    #[test]
    fn yolo()-> Result<(), Box<dyn std::error::Error>>{
        // 1. load image
        let x = image::io::Reader::open("./1.jpg")?
            .with_guessed_format()?
            .decode()?;
        // let origin_height = x.height();
        // let origin_width = x.width();
        let a = image::imageops::resize(&x, 640, 640, image::imageops::FilterType::CatmullRom);


        // 2. model support dynamic batch inference, so input should be a Vec
        let x = image::DynamicImage::from(a);
        let xs = vec![x];

        // You can test `--batch 2` with this

        let mut model = yolo::yolo_run("cpu")?;

        // 4. run
        let ys = model.run(&xs)?;

        //结果集
        let name =model.names().clone();
        for data in ys.iter() {
            // draw bboxes & keypoints
            if let Some(bboxes) = data.bboxes() {    // let xs = vec![x.clone(), x];
                for (_idx, bbox) in bboxes.iter().enumerate() {
                    // x_original = (x_scaled / 640) * original_width
                    // x_original, y_original 是映射后的边界框坐标。
                    // x_scaled, y_scaled 是模型输出的边界框坐标。
                    // scaled_width, scaled_height 是缩放后的图像尺寸（640 或 640）。
                    // original_width, original_height 是原始图像的尺寸。
                    println!("{:?},{:?},{:?}",name,bbox,(bbox.xmin()/2.0,bbox.ymin()/2.0,(bbox.xmin()+bbox.width())/2.0,(bbox.ymin()+bbox.height())/2.0));
                }
            }
        }
        Ok(())
    }
}