use lib::handle;
use lib::yolo;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let run  = handle::dnf::Dnf::new(yolo::yolo_run()?);

    run.change_hero(1,1);
    // let capture_window = Windows::new();
    // let mut i=0;l
    // loop {
    //     capture_window.capture_now().save(format!("./runs/{:?}.png",i)).unwrap();
    //     i+=1;
    //     thread::sleep(time::Duration::from_secs(5));
    // }


    Ok(())
}
