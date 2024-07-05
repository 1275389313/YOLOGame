use std::fmt::format;
use std::{thread, time};
use virtual_ddl::windows;
use crate::capture::Capture;
use crate::capture::windows::Windows;

mod yolo;
mod adb;
mod capture;
mod handle;
mod virtual_ddl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // handle::dnf::Dnf::new(yolo::yolo_run()?).run();
    let capture_window = Windows::new();
    let mut i=0;
    loop {
        capture_window.capture_now().save(format!("./runs/{:?}.png",i)).unwrap();
        i+=1;
        thread::sleep(time::Duration::from_secs(5));
    }
    Ok(())
}
