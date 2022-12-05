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

pub fn flatten_result<T, E>(result: Result<Result<T, E>, E>) -> Result<T, E> {
    match result {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(err)) | Err(err) => Err(err),
    }
}
