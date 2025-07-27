use std::{error::Error, fmt, num::ParseIntError};

#[derive(Debug, Clone)]
pub struct ApplicationError(pub String);
impl Error for ApplicationError {}
impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error occurred: {}", self.0)
    }
}
impl From<std::io::Error> for ApplicationError {
    fn from(val: std::io::Error) -> Self {
        Self { 0: val.to_string() }
    }
}
impl From<ParseIntError> for ApplicationError {
    fn from(val: ParseIntError) -> Self {
        Self { 0: val.to_string() }
    }
}
impl From<SelectionError> for ApplicationError {
    fn from(val: SelectionError) -> Self {
        Self { 0: val.to_string() }
    }
}

#[derive(Debug, Clone)]
pub struct SelectionError(pub String);
impl Error for SelectionError {}
impl fmt::Display for SelectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid selection: {}", self.0)
    }
}