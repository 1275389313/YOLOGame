use std::collections::HashMap;
use std::error::Error;
use std::hash::RandomState;
use std::{thread, time};
use crate::virtual_ddl;
use rand::Rng;
//
// #[link(name = "dd40605x64")]
// extern "C" {
//     /// 1 =左键按下 ，2 =左键放开
//     // // 4 =右键按下 ，8 =右键放开
//     // // 16 =中键按下 ，32 =中键放开
//     // // 64 =4键按下 ，128 =4键放开
//     // // 256 =5键按下 ，512 =5键放开
//     fn DD_btn(btn: c_int); //鼠标点击
//     fn DD_mov(x: c_int, y: c_int); //鼠标绝对移动
//     fn DD_movR(x: c_int, y: c_int); //模拟鼠标相对移动
//     fn DD_whl(whl: c_int); //模拟鼠标滚轮 1=前 , 2 = 后
//     fn DD_str(s: *mut c_char); // 直接输入键盘上可见字符和空格
//     // ddcode参考[DD虚拟键盘码表]。
//     // flag，1=按下，2=放开
//     fn DD_key(ddcode: c_int, flag: c_int); // 键盘按键
// }

pub struct MouseAndKeyboardInstruct {
    keys: HashMap<String, i32, RandomState>,
    lib: libloading::Library,
}

impl MouseAndKeyboardInstruct {
    pub fn new() -> Self {
        let data: Vec<(&str, i32)> = vec![
            ("esc", 100), ("f1", 101), ("f2", 102), ("f3", 103), ("f4", 104), ("f5", 105), ("f6", 106), ("f7", 107), ("f8", 108), ("f9", 109), ("f10", 110), ("f11", 111), ("f12", 112),
            ("~", 200), ("1", 201), ("2", 202), ("3", 203), ("4", 204), ("5", 205), ("6", 206), ("7", 207), ("8", 208), ("9", 209), ("0", 210), ("k-", 211), ("k=", 212), ("\\", 213), ("back_space", 214),
            ("tab", 300), ("q", 301), ("q", 301), ("w", 302), ("e", 303), ("r", 304), ("t", 305), ("y", 306), ("u", 307), ("i", 308), ("o", 309), ("p", 310), ("[", 311), ("]", 311), ("k_enter", 313),
            ("caps_lock", 400), ("a", 401), ("s", 402), ("d", 403), ("f", 404), ("g", 405), ("h", 406), ("j", 407), ("k", 408), ("l", 409), (";", 410), ("\"", 411),
            ("shift", 500), ("z", 501), ("x", 502), ("c", 503), ("v", 504), ("b", 505), ("n", 506), ("m", 507), (",", 508), (".", 509), ("?", 510), ("right_shift", 511),
            ("ctrl", 600), ("left_window", 603), ("alt", 602), ("space", 603),
            ("up", 709), ("left", 710), ("down", 711), ("right", 712),
            ("n_0", 800), ("n_1", 801), ("n_enter", 815), //还有
        ];
        let mut m = HashMap::new();
        for (k, b) in data {
            m.insert(k.to_string(), b);
        }
        unsafe {
            let lib = libloading::Library::new("./ddhid.40616x64.dll").unwrap();
            let func2: libloading::Symbol<unsafe extern fn(i32) -> i32> = lib.get(b"DD_btn").unwrap();
            //lib init
            if func2(0) != 1 {
                panic!("lib error");
            }
            //初始化会按 win 键
            let func: libloading::Symbol<unsafe extern fn(i32, i32) -> i32> = lib.get(b"DD_key").unwrap();
            func(100, 1);
            thread::sleep(time::Duration::from_millis(30));
            func(100, 2);
            Self {
                keys: m,
                lib,
            }
        }
    }
    pub fn on(&self, keys: &str) -> Result<(), Box<dyn Error>> {
        match self.keys.get(keys) {
            Some(code) => {
                unsafe {
                    let func: libloading::Symbol<unsafe extern fn(i32, i32) -> i32> = self.lib.get(b"DD_key").unwrap();
                    if func(*code, 1) != 1 {
                        return Err(Box::new(virtual_ddl::Virtual::KeyError));
                    }
                    thread::sleep(time::Duration::from_millis(rand::thread_rng().gen_range(30..58)));
                    if func(*code, 2) != 1 {
                        return Err(Box::new(virtual_ddl::Virtual::KeyError));
                    }
                }
                Ok(())
            }
            None => {
                Err(Box::new(virtual_ddl::Virtual::KeyError))
            }
        }
    }
    pub fn on_mov(&self, x: f32, y: f32, to_x: f32, to_y: f32) -> Result<(), Box<dyn Error>> {
        // ("up", 709), ("left", 710), ("down", 711), ("right", 712),
        let mut x_code = 712;
        let mut y_code = 711;
        let x_mov: u64 = (x - to_x).abs() as u64;
        let y_mov: u64 = (y - to_y).abs() as u64;
        if x - to_x > 0f32 {
            x_code = 710
        }
        if y - to_y > 0f32 {
            y_code = 709
        }
        println!("{:?}", (x, y, to_x, to_y));
        unsafe {
            let func: libloading::Symbol<unsafe extern fn(i32, i32) -> i32> = self.lib.get(b"DD_key").unwrap();
            //快速移动
            if func(x_code, 1) != 1 {
                return Err(Box::new(virtual_ddl::Virtual::KeyError));
            }
            thread::sleep(time::Duration::from_millis(rand::thread_rng().gen_range(30..50)));
            if func(x_code, 2) != 1 {
                return Err(Box::new(virtual_ddl::Virtual::KeyError));
            }
            thread::sleep(time::Duration::from_millis(rand::thread_rng().gen_range(20..50)));
            if func(x_code, 1) != 1 {
                return Err(Box::new(virtual_ddl::Virtual::KeyError));
            }
            thread::sleep(time::Duration::from_millis(rand::thread_rng().gen_range(10..20)));
            if func(y_code, 1) != 1 {
                return Err(Box::new(virtual_ddl::Virtual::KeyError));
            }
            thread::sleep(time::Duration::from_millis(rand::thread_rng().gen_range(20..58)));

            //需要速度
            if x_mov > y_mov {
                let s = y_mov * 3; //需要计算实际速度
                if s > 0 {
                    thread::sleep(time::Duration::from_millis(s));
                }
                if func(y_code, 2) != 1 {
                    return Err(Box::new(virtual_ddl::Virtual::KeyError));
                }
            } else {
                let s = x_mov * 2; //需要计算实际速度
                if s > 0 {
                    thread::sleep(time::Duration::from_millis(s));
                }
                if func(x_code, 2) != 1 {
                    return Err(Box::new(virtual_ddl::Virtual::KeyError));
                }
            }

            if x_mov > y_mov {
                let s = (x_mov - y_mov) * 2; //需要计算实际速度
                if s > 0 {
                    thread::sleep(time::Duration::from_millis(s));
                }
                if func(x_code, 2) != 1 {
                    return Err(Box::new(virtual_ddl::Virtual::KeyError));
                }
            } else {
                let s = (y_mov - x_mov) * 3; //需要计算实际速度
                if s > 0 {
                    thread::sleep(time::Duration::from_millis(s));
                }
                if func(y_code, 2) != 1 {
                    return Err(Box::new(virtual_ddl::Virtual::KeyError));
                }
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::virtual_ddl::windows::MouseAndKeyboardInstruct;

    #[test]
    fn key() {
        let w = MouseAndKeyboardInstruct::new();
        w.on("a").unwrap()
    }
}