
use std::io::Error;

pub trait Keys {
    fn install(&self) -> Result<i32, Error>;
}
