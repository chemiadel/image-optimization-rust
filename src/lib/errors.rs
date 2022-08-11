use std::{fmt};
use std::fmt::Formatter;
use std::io::{Error, ErrorKind};
#[derive(Debug)]
pub enum ImageErrors {
    FileIOError(String),
    UserInputError(String),
    ImageResizingError(String),
    FormatError(String),
}

impl From<Error> for ImageErrors {
    fn from(_err: Error) -> Self {
        ImageErrors::FileIOError("error in file reading".to_string())
    }
}
impl From<ErrorKind> for ImageErrors {
    fn from(_err: ErrorKind) -> Self {
        ImageErrors::UserInputError("error caused by user input".to_string())
    }
}

impl  fmt::Display for ImageErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f , "{}" ,ImageErrors::FormatError("processing crash".to_string())
        )
    }
}