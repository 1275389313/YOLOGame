use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        let device_state = DeviceState::new();
        let device_state2 = device_state.clone();

        let thread_handle = thread::spawn(move || {
            loop {
                println!("{:?}", device_state.get_mouse().coords);
                println!("{:?}", device_state.get_keys());

                thread::sleep(Duration::from_secs(1));
                thread::park()
            }
        });

        let _guard = device_state2.on_key_down(move |key| {
            if *key == Keycode::F5 {
                thread_handle.thread()();
                println!("1")
            }
        });
        // // 设置超时时间
        // thread::park_timeout(Duration::from_secs(2));

        // 终止线程
        // thread_handle.join().expect("Failed to join the thread");
        loop {

        }

    }
}

