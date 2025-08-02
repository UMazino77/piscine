use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed,
}

impl Display for ParseErr {
}

impl Error for ParseErr {
}
#[derive(Debug)]
pub struct ReadErr {
    child_err : Box<dyn Error>,
}

impl Display for ReadErr {
}

impl Error for ReadErr {
}