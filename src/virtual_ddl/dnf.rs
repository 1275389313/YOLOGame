use std::error::Error;
use std::{thread, time};
use rand::Rng;
use crate::virtual_ddl;
use crate::virtual_ddl::windows::{MouseAndKeyboardInstruct, UpOrDown};


pub struct DNFVirtual {
    key_mouse: MouseAndKeyboardInstruct,
}

impl DNFVirtual {
    pub fn new() -> Self {
        DNFVirtual {
            key_mouse: MouseAndKeyboardInstruct::new()
        }
    }
    pub fn on_mov(&self, x: f32, y: f32, to_x: f32, to_y: f32) -> Result<(), Box<dyn Error>> {
        // ("up", 709), ("left", 710), ("down", 711), ("right", 712),
        let mut x_code = "right";
        let mut y_code = "down";
        let x_mov: u64 = (x - to_x).abs() as u64;
        let y_mov: u64 = (y - to_y).abs() as u64;
        if x - to_x > 0f32 {
            x_code = "left"
        }
        if y - to_y > 0f32 {
            y_code = "up"
        }
        println!("{:?}", (x, y, to_x, to_y));
        unsafe {
            //快速移动
            self.key_mouse.on(x_code, UpOrDown::DownAndUp)?;
            self.key_mouse.on(x_code, UpOrDown::Up)?;
            thread::sleep(time::Duration::from_millis(rand::thread_rng().gen_range(30..50)));
            self.key_mouse.on(y_code, UpOrDown::Down)?;
            thread::sleep(time::Duration::from_millis(rand::thread_rng().gen_range(20..58)));

            //需要速度
            if x_mov > y_mov {
                let s = y_mov * 3; //需要计算实际速度
                if s > 0 {
                    thread::sleep(time::Duration::from_millis(s));
                }
                self.key_mouse.on(y_code, UpOrDown::Up)?;
            } else {
                let s = x_mov * 2; //需要计算实际速度
                if s > 0 {
                    thread::sleep(time::Duration::from_millis(s));
                }
                self.key_mouse.on(x_code, UpOrDown::Up)?;
            }

            if x_mov > y_mov {
                let s = (x_mov - y_mov) * 2; //需要计算实际速度
                if s > 0 {
                    thread::sleep(time::Duration::from_millis(s));
                }

                self.key_mouse.on(x_code, UpOrDown::Up)?;
            } else {
                let s = (y_mov - x_mov) * 3; //需要计算实际速度
                if s > 0 {
                    thread::sleep(time::Duration::from_millis(s));
                }
                self.key_mouse.on(y_code, UpOrDown::Up)?;
            }
        }
        Ok(())
    }
    pub fn on(&self, keys: &str) -> Result<(), Box<dyn Error>> {
        self.key_mouse.on(keys, UpOrDown::DownAndUp)?;
        Ok(())
    }

    pub fn mouse_mov_click(&self, x: u32, y: u32) {
        self.key_mouse.mouse_mov_click(x, y);
    }

}