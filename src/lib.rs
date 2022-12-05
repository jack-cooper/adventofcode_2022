use std::{error::Error, fmt};

pub type AnyResult = Result<(), Box<dyn Error>>;

#[derive(Debug)]
pub struct CustomError(String);

impl CustomError {
    pub fn new(msg: impl ToString) -> Self {
        Self(msg.to_string())
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError({})", self.0)
    }
}
