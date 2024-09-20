use image::{DynamicImage, RgbaImage};
use win_screenshot::prelude::*;

pub struct Windows {
    pub x: u32, //屏幕x大小
    pub y: u32, //屏幕y大小
    pub find_window_x: u32, //x应用窗口在屏幕点位
    pub find_window_y: u32, //y应用窗口在屏幕点位
}


impl super::Capture for Windows {
    fn capture_now(&self) -> DynamicImage {
        let hwnd = find_window("地下城与勇士：创新世纪").expect(" 地下城与勇士：创新世纪 窗口查询失败");

        let buf = capture_window_ex(hwnd, Using::BitBlt, Area::ClientOnly, None, None).unwrap();

        //缩放应用属性导致图片异常: 可能实际上800*600 变成 1200*900 的图片(大于部分都是黑色)
        DynamicImage::ImageRgba8(
            RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap())
    }
}

impl Windows {
    pub fn new() -> Self {
        //获取屏幕尺寸
        let buf = capture_display().expect("capture_display error");
        let hwnd = find_window("地下城与勇士：创新世纪").expect("not 地下城与勇士：创新世纪");

        let buf2 = capture_window_ex(hwnd, Using::PrintWindow, Area::Full, None, None).unwrap();
        println!("当前游戏窗口定点：({:?},{:?}),窗口大小:({:?},{:?})", buf2.x, buf2.y, buf.width, buf.height);
        Self {
            x: buf.width,
            y: buf.height,
            
            find_window_x: buf2.x,
            find_window_y: buf2.y,
        }
    }
}