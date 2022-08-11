pub enum ImageErrors {
    FileIOError(String),
    UserInputError(String),
    ImageResizingError(String),
    FormatError(String),
}