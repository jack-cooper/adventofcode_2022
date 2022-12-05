use std::{borrow::Cow, error::Error, fmt};

pub type AnyResult = Result<(), Box<dyn Error>>;

#[derive(Debug)]
pub struct CustomError {
    pub msg: Cow<'static, str>,
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError({})", self.msg)
    }
}
