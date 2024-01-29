use std::io::{Error as IOError, Result as IOResult};
use crate::image::{ImageError, ImageResult};



pub trait OptionToCoreError<T> {
    fn to_core_error(self, error: CoreError) -> CoreResult<T>;
}

impl<T> OptionToCoreError<T> for Option<T> {
    fn to_core_error(self, error: CoreError) -> CoreResult<T> {

        match self {
            Some(t) => Ok(t),
            None => Err(error),
        }
    }
}

pub trait ResultToCoreError<T> {
    fn to_core_error(self) -> CoreResult<T>;
}

impl<T> ResultToCoreError<T> for IOResult<T> {
    fn to_core_error(self) -> CoreResult<T> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(CoreError::FileError(error)),
        }
    }
}

impl<T> ResultToCoreError<T> for ImageResult<T> {
    fn to_core_error(self) -> CoreResult<T> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(CoreError::ImageError(error)),
        }
    }
}



pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug)]
pub enum CoreError {

    InvalidPath,
    FileError(IOError),

    ImageError(ImageError),

}
