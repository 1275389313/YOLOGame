// use windows::core::{Error, Result, HRESULT, Param};
// use std::{ptr, slice};
// use image::DynamicImage;
// use windows::{
//     Globalization::Language,
//     Graphics::Imaging::{BitmapBufferAccessMode, SoftwareBitmap, BitmapPixelFormat},
//     Media::Ocr::*,
//     Win32::System::WinRT::*,
//     core::Interface,
//
// };
//
//
// pub fn dynamic_image_ocr(img: DynamicImage) -> Result<String> {
//     //
//     let width = img.width() as i32;
//     let height = img.height() as i32;
//     let rgb = img.to_rgb8().to_vec();
//
//     let bmp = SoftwareBitmap::Create(BitmapPixelFormat::Rgba8, width, height).unwrap();
//     {
//         let bmp_buf = bmp.LockBuffer(BitmapBufferAccessMode::Write).unwrap();
//         let array: IMemoryBufferByteAccess = bmp_buf.CreateReference().unwrap().cast().unwrap();
//
//         let mut data = ptr::null_mut();
//         let mut capacity = 0;
//         unsafe {
//             array.GetBuffer(&mut data, &mut capacity)?;
//         }
//         assert_eq!((width * height * 4).abs(), capacity as i32);
//
//         let slice = unsafe { slice::from_raw_parts_mut(data, capacity as usize) };
//         slice.chunks_mut(4).enumerate().for_each(|(i, c)| {
//             c[0] = rgb[3 * i];
//             c[1] = rgb[3 * i + 1];
//             c[2] = rgb[3 * i + 2];
//             c[3] = 255;
//         });
//     }
//
//     let lang = &OcrEngine::AvailableRecognizerLanguages()?
//         .First()?
//         .Current()?
//         .LanguageTag()?;
//
//     let lang = Language::CreateLanguage(lang)?;
//     let engine = OcrEngine::TryCreateFromLanguage(&lang)?;
//
//     let result = engine
//         .RecognizeAsync(&bmp)?
//         .get()?
//         .Text()?
//         .to_string_lossy();
//     Ok(result)
// }
