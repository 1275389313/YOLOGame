use image::DynamicImage;
use yolo::model::YOLOv8;
use crate::{virtual_ddl, yolo};
use crate::capture::windows::Windows;
use crate::capture::Capture;
use crate::yolo::model::SKELETON;
use rand::Rng;
use std::{thread, time};
// use std::ops::Sub;
// use dirs::data_dir;

pub struct Dnf {
    pub model: YOLOv8,
    pub names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DNFYOLOResult {
    pub hero: Option<(f32, f32, f32, f32)>,
    pub monster: Vec<(f32, f32, f32, f32)>,//怪物
    pub equip: Vec<(f32, f32, f32, f32)>,//装备
    pub species: Vec<(f32, f32, f32, f32)>,//金币
    pub material: Vec<(f32, f32, f32, f32)>,//材料
    pub boss: Vec<(f32, f32, f32, f32)>,//boss
    pub door: Vec<(f32, f32, f32, f32)>,//门
    pub other: Vec<(usize, f32, f32, f32, f32)>,//其他
}

const CHARSET: &[u8] = b"asdfgqwert";

impl Dnf {
    pub fn new(m: YOLOv8) -> Self {
        Self {
            names: m.names().clone(),
            model: m,
        }
    }
    pub fn run(&mut self) {
        let capture_window = Windows::new();
        let operate_instruction = virtual_ddl::MouseAndKeyboardInstruct::new();
        let mut door_num = 0;//卡门处理
        let mut rng = rand::thread_rng();
        loop {
            //截图
            // let now = time::Instant::now();           // , now.sub(time::Instant::now());
            //处理时间 截图和识别
            let data = self.prediction(capture_window.capture_now());

            if let Ok(result) = data {
                //无法识别人物
                if result.hero.is_none() {
                    continue;
                }
                //人物位置
                let hero = result.hero.unwrap();
                let (hero_x, hero_y) = (hero.0, hero.1 + hero.3);

                //遇到怪物先处理怪物
                if let Some((x, y)) = self.last_point(hero_x, hero_y, &result.monster) {
                    println!("最近怪物距离{:?},{:?}", x, y);
                    let _ = operate_instruction.on_mov(hero_x, hero_y, x, y);
                    let random_char = CHARSET[rng.gen_range(0..CHARSET.len())] as char;
                    let _ = operate_instruction.on(&random_char.to_string());
                    continue;
                }

                //boss
                if let Some((x, y)) = self.last_point(hero_x, hero_y, &result.monster) {
                    let _ = operate_instruction.on("y");//觉醒
                    let _ = operate_instruction.on_mov(hero_x, hero_y, x, y);
                    let random_char = CHARSET[rng.gen_range(0..CHARSET.len())] as char;
                    let _ = operate_instruction.on(&random_char.to_string());
                    continue;
                }

                //物品
                if result.equip.len() > 0 || result.species.len() > 0 || result.material.len() > 0 {
                    let mut data = result.equip;
                    data.extend(result.species).extend(result.material);
                    let _ = operate_instruction.on_mov(hero_x, hero_y, data[0].0, data[0].1 + data[0].3);
                    let _ = operate_instruction.on("x");
                    continue;
                }

                //门 下一个房间

                //卡住过不了图
                if door_num > 30 {
                    door_num = 0;
                }
                door_num += 1;
            }
            thread::sleep(time::Duration::from_millis(250));
        }
    }

    ///最近距离一个怪物
    pub fn last_point(&self, hero_x: f32, hero_y: f32, material_vec: &Vec<(f32, f32, f32, f32)>) -> Option<(f32, f32)> {
        let mut last_x = std::f32::MAX;
        let mut last_y = std::f32::MAX;
        let mut point = (0f32, 0f32);
        let mut i = 0_usize;
        let mut i2 = 0_usize;
        for (x, y, _with, height) in material_vec.iter() {
            let temp_x = hero_x - x;
            let temp_y = hero_y - y + height;
            if (temp_x.abs() + temp_y.abs()) < last_x.abs() + last_y.abs() {
                last_x = temp_x;
                last_y = last_y;
                point = (*x, y + height);
                i2 = i;
            }
            i += 1;
        }
        if last_x == std::f32::MAX {
            return None;
        }
        println!("{:?}", material_vec[i2]);
        println!("{hero_x},{hero_y}");
        Some(point)
    }
    ///yolo识别和预处理并对结果整理
    pub fn prediction(&mut self, img: DynamicImage) -> Result<DNFYOLOResult, Box<dyn std::error::Error>> {
        let (original_width, original_height) = (img.width(), img.height());
        let xs = &vec![image::DynamicImage::from(image::imageops::resize(&img, 640, 640, image::imageops::FilterType::CatmullRom))];
        let mut ys = self.model.run(xs)?;
        let mut result = DNFYOLOResult {
            hero: None,
            monster: Vec::with_capacity(8),
            equip: Vec::with_capacity(4),
            species: Vec::with_capacity(4),
            material: Vec::with_capacity(4),
            boss: Vec::with_capacity(4),
            door: Vec::with_capacity(4),
            other: Vec::with_capacity(4),
        };

        //结果集
        for data in ys.iter_mut() {
            // draw bboxes & keypoints
            if let Some(bboxes) = data.bboxes_mut() {
                for (_idx, bbox) in bboxes.iter_mut().enumerate() {
                    //640 * 640 映射会原始尺寸
                    bbox.xmin = (bbox.xmin() / 640.0) * original_width as f32;
                    bbox.ymin = (bbox.ymin() / 640.0) * original_height as f32;
                    bbox.width = (bbox.width() / 640.0) * original_width as f32;
                    bbox.height = (bbox.height() / 640.0) * original_height as f32;
                    // bbox.xmin() = (x_scaled / 640) * original_width
                    let name = (&*self.names[bbox.id()]).into();
                    match name {
                        "hero" => {
                            bbox.height = bbox.height + 135f32;//人物偏差 以最小为标准
                            result.hero = Some((bbox.xmin(), bbox.ymin(), bbox.width(), bbox.height()));
                        }
                        "monster" => {
                            result.monster.push((bbox.xmin(), bbox.ymin(), bbox.width(), bbox.height()));
                        }
                        "equip" => {
                            result.equip.push((bbox.xmin(), bbox.ymin(), bbox.width(), bbox.height()));
                        }
                        "species" => {
                            result.species.push((bbox.xmin(), bbox.ymin(), bbox.width(), bbox.height()));
                        }
                        "material" => {
                            result.material.push((bbox.xmin(), bbox.ymin(), bbox.width(), bbox.height()));
                        }
                        "boss" => {
                            result.boss.push((bbox.xmin(), bbox.ymin(), bbox.width(), bbox.height()));
                        }
                        "door" => {
                            result.door.push((bbox.xmin(), bbox.ymin(), bbox.width(), bbox.height()));
                        }
                        _ => {
                            result.other.push((bbox.id, bbox.xmin(), bbox.ymin(), bbox.width(), bbox.height()));
                        }
                    }
                }
            }
        }
        self.model.plot_and_save(&ys, &vec![img], Some(&SKELETON));

        Ok(result)
    }
}