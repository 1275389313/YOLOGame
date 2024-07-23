use std::{thread, time};
use image::DynamicImage;
use image::io::Reader as ImageReader;
use lib::ui;
use lib::ui::debug;

fn main() {
    {    // 加载图像
        let img = ImageReader::open("extra/33.png")
            .unwrap()
            .decode()
            .unwrap()
            ;
        let ui = debug::DebugWindow::new();
        ui.show_debug_window(img.to_rgb8());
        thread::sleep(time::Duration::from_secs(1));
        let img = ImageReader::open("extra/34.png")
            .unwrap()
            .decode()
            .unwrap()
            ;
        ui.show_debug_window(img.to_rgb8());
        thread::sleep(time::Duration::from_secs(1));
        let img = ImageReader::open("extra/33.png")
            .unwrap()
            .decode()
            .unwrap()
            ;
        ui.show_debug_window(img.to_rgb8());
        thread::sleep(time::Duration::from_secs(1));
        let img = ImageReader::open("extra/34.png")
            .unwrap()
            .decode()
            .unwrap()
            ;
        ui.show_debug_window(img.to_rgb8());}
    thread::sleep(time::Duration::from_secs(10));
}