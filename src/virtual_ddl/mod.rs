use std::fmt;

pub  mod windows;
pub  use windows::MouseAndKeyboardInstruct ;

use thiserror::Error;
#[derive(Error,Debug)]
pub  enum  Virtual{
    #[error("key not ddcode")]
    KeyError
}
