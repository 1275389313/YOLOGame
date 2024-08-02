use image::{DynamicImage, GenericImage, RgbaImage};
use win_screenshot::prelude::*;

fn main() {
    // Capture entire screen
    // let buf = capture_display().unwrap();

    let hwnd = window_list();
    // hwnd.iter().for_each(|a|{
    //     println!("{:?}",a)
    // });
    // return;
    let hwnd = window_list()
        .unwrap()
        .iter()
        .find(|i| i.window_name=="机械革命电竞控制台" )
        .unwrap()
        .hwnd;
    println!("{:?}",hwnd);

   let buf =  capture_window(hwnd).unwrap();
   //  // convert to image and save
   //  let mut  img = DynamicImage::ImageRgba8(
   //      RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap());
   //  // img.to_rgb8().save("screenshot.jpg").unwrap();
    println!("{:?} {:?} , {:?} {:?}",buf.x,buf.y,buf.width,buf.height)
   // let data =  img.sub_image(0u32,0,800_u32,600_u32).to_image().into() as DynamicImage;

    // Fine tuning

    // BitBlt dramatically faster, often fails
    // (e.g. firefox, steam, 3d accelerated windows)
    // let using = Using::BitBlt;
    // // PrintWindow much slower, much more reliable
    // let using = Using::PrintWindow;
    //
    // // Capture client area of window
    // let area = Area::ClientOnly;
    // // Capture whole window (not supported with BitBlt)
    // let area = Area::Full;
    //
    // // Build-in crop, faster on large windows
    // let crop_xy = None; //Some([100, 100]);
    // let crop_wh = None; //Some([300, 300]);
    // let buf = capture_window_ex(hwnd, using, area, crop_xy, crop_wh).unwrap();
}