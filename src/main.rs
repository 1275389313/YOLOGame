use std::{thread, time};

use lib::handle;
use lib::yolo;
use lib::yolo::model::OrtEP;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("10秒后运行");
    thread::sleep(time::Duration::from_secs(10));
    let mut run = handle::dnf::Dnf::new(yolo::yolo_run("GPU")?);
    run.mov_door();
    // loop {
    //     run.pre_work();
    //     run.run();
    // }
    Ok(())
}
