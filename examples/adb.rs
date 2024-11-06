use std::process::{Command};
use image;
use std::io::Cursor;
fn main() {
    let bit = Command::new("cmd")
        .args(&["/C","adb.exe", "exec-out", "screencap", "-p"])
        .output()
        .expect("Failed to execute command");
    let png =image::load(Cursor::new(bit.stdout),image::ImageFormat::Png).unwrap();
    png.save("output.png").unwrap();
    println!("{:?}", bit.stderr)
}