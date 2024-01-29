use std::io::{Error as IOError, Result as IOResult};
use crate::image::{ImageError, ImageResult};


pub trait ToInvalidPath<T> {
    fn to_invalid_path_error(self) -> AssetLoadResult<T>;
}

impl<T> ToInvalidPath<T> for Option<T> {
    fn to_invalid_path_error(self) -> AssetLoadResult<T> {

        match self {
            Some(t) => Ok(t),
            None => Err(AssetLoadError::InvalidPath),
        }
    }
}

pub trait ToFileOp<T> {
    fn to_file_operation_error(self) -> AssetLoadResult<T>;
}

impl<T> ToFileOp<T> for IOResult<T> {
    fn to_file_operation_error(self) -> AssetLoadResult<T> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(AssetLoadError::FileOp(error)),
        }
    }
}

pub trait ImageLoadError<T> {
    fn to_image_load_error(self) -> AssetLoadResult<T>;
}

impl<T> ImageLoadError<T> for ImageResult<T> {
    fn to_image_load_error(self) -> AssetLoadResult<T> {

        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(AssetLoadError::Image(error)),
        }
    }
}



pub type AssetLoadResult<T> = Result<T, AssetLoadError>;

pub enum AssetLoadError {

    InvalidPath,
    FileOp(IOError),
    Image(ImageError),

}
