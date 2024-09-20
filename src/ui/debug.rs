use std::{sync::mpsc::{self, Sender, Receiver}, thread};
use image::RgbImage;
use minifb::{Window, WindowOptions};

enum ImageMessage {
    Data(Vec<u32>),
    Exit,
}
pub struct DebugWindow {
    tx: Sender<ImageMessage>,
}
impl DebugWindow {
    pub fn new() -> DebugWindow {
        let (tx, rx): (Sender<ImageMessage>, Receiver<ImageMessage>) = mpsc::channel();
        let _a = thread::spawn(move || {
            // 创建窗口
            let mut window = Window::new(
                "Image Viewer",
                500_usize,
                500_usize,
                WindowOptions::default(),
            ).unwrap_or_else(|e| {
                panic!("{}", e);
            });
            // 限制帧率
            window.set_target_fps(35);
            let mut buf=vec![] ;
            // 显示图像
            while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
                match rx.try_recv() {
                    Ok(msg) => {
                        match msg {
                            ImageMessage::Data(img) => {
                                window
                                    .update_with_buffer(&img, 500_usize, 500_usize)
                                    .unwrap();
                                buf = img;
                            }
                            ImageMessage::Exit => {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        match e {
                            std::sync::mpsc::TryRecvError::Empty => {
                                if buf.len()>0 {
                                    window
                                        .update_with_buffer(&buf, 500_usize, 500_usize)
                                        .unwrap();
                                }
                            }
                            std::sync::mpsc::TryRecvError::Disconnected => {
                                break;
                            }
                        }
                    }
                }
            }
        });
        DebugWindow {
            tx
        }
    }
    pub fn show_debug_window(&self, img: RgbImage) {
        let img = image::DynamicImage::from(image::imageops::resize(&img, 640, 640, image::imageops::FilterType::CatmullRom)).to_rgb8();
        let buffer: Vec<u32> = img
            .pixels()
            .flat_map(|p| {
                let (r, g, b) = (p[0], p[1], p[2]);
                vec![(r as u32) << 16 | (g as u32) << 8 | b as u32]
            })
            .collect();
        let _ = self.tx.send(ImageMessage::Data(buffer));
    }
}
