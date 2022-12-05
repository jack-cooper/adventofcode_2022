use std::{borrow::Cow, error::Error, fmt};

pub type AnyResult = Result<(), Box<dyn Error>>;

#[derive(Debug)]
pub struct CustomError<'a> {
    pub msg: Cow<'a, str>,
}

impl Error for CustomError<'_> {}

impl fmt::Display for CustomError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError({})", self.msg)
    }
}
