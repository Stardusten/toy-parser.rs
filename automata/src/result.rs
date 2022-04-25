use std::fmt::{Display, Formatter};

pub type IResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IllegalArgument(&'static str),
    UnsupportedOperation(&'static str),
    Uninitialized(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Error::IllegalArgument(e) => {
                write!(f,"Illegal Argument: {}",e)
            },
            Error::UnsupportedOperation(e) => {
                write!(f, "Unsupported Operation: {}", e)
            },
            Error::Uninitialized(e) => {
                write!(f, "Uninitialized: {}", e)
            }
        }
    }
}