use std::{fs, io};
use std::path::PathBuf;
use crate::lib::errors::ImageErrors;

pub fn get_image(path : PathBuf) -> Result<Vec<PathBuf> , ImageErrors> {
    let inputs = fs::read_dir(path)
        .map_err(|input| ImageErrors::UserInputError("invalid input".to_string()))?
        .map(|result| result.map(|entry|entry.path()))
        .collect::<Result<Vec<_>,io::Error>>()?
        .into_iter()
        .filter(|file|{
            file.extension() == Some("PNG".as_ref())
            || file.extension() == Some("png".as_ref()) })
        .collect() ;
        Ok(inputs)
}