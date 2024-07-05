use image::{DynamicImage, RgbaImage};
use win_screenshot::prelude::*;

pub struct Windows {
    pub x: u32,
    pub y: u32,
}


impl super::Capture for Windows {
    fn capture_now(&self) -> DynamicImage {
        let hwnd = find_window("地下城与勇士：创新世纪").unwrap();

        let buf = capture_window_ex(hwnd, Using::PrintWindow, Area::ClientOnly, None, None).unwrap();
        //缩放应用属性导致图片异常: 可能实际上800*600 变成 1200*900 的图片(大于部分都是黑色)
        DynamicImage::ImageRgba8(
            RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap())
    }
}

impl Windows {
    pub fn new() -> Self {
        //获取屏幕尺寸
        let buf = capture_display().unwrap();

        Self {
            x: buf.width,
            y: buf.height,
        }
    }
}