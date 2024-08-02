pub  mod windows;
pub trait Capture {
    fn capture_now(&mut self) -> image::DynamicImage;
    // fn capture_find() -> image::DynamicImage;
    // fn capture_list() -> image::DynamicImage;
    // fn save() -> Result<(), Box<std::error>>;
}