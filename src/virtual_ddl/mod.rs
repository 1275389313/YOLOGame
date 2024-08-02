
pub  mod windows;
pub  mod dnf;

pub  use windows::MouseAndKeyboardInstruct ;
pub  use dnf::DNFVirtual ;

use thiserror::Error;
#[derive(Error,Debug)]
pub  enum  Virtual{
    #[error("key not ddcode")]
    KeyError
}
