use std::io::Read;
use png::{BitDepth, ColorType, Decoder, DecodingError};
use crate::image::color::{Overdraw, Rgb8};

pub mod color;
pub mod file_load;



pub struct Image<Color: PartialEq + Clone + From<Rgb8>> {

    pixels: Vec<Color>,
    width: usize,
    height: usize,

}

impl<Color: PartialEq + Clone + From<Rgb8>> Image<Color> {

    pub fn new_raw(image: Vec<Color>, width: usize, height: usize) -> ImageResult<Self> {
        if width * height != image.len() {
            return Err(ImageError::DifferentDimensions);
        }

        Ok(Self {
            pixels: image,
            width,
            height,
        })
    }

    pub fn new_uniform(filler: Color, width: usize, height: usize) -> Self {

        Self {
            pixels: vec![filler; width * height],
            width,
            height,
        }
    }

    pub fn load_png<R: Read>(reader: R) -> ImageResult<Self> {
        let decoder = Decoder::new(reader);
        let mut png_reader = decoder.read_info().to_image_result()?;

        let mut pixel_buffer = vec![0; png_reader.output_buffer_size()];
        let info = png_reader.next_frame(&mut pixel_buffer).to_image_result()?;

        let raw_image_buffer = &mut pixel_buffer[..info.buffer_size()];
        let image_value_buffer = file_load::pass_bit_depth(raw_image_buffer, info.bit_depth);

        let image = file_load::pass_color_type(image_value_buffer, info.color_type);

        Self::new_raw(image, info.width as usize, info.height as usize)
    }


    pub fn overdraw_shaped_image<
        Filler: Overdraw<Color> + PartialEq + Clone + From<Rgb8>,
        ShapeColor: PartialEq + Clone + From<Rgb8>,
    >(&mut self, filler: Image<Filler>, shape: Image<ShapeColor>, shape_color: ShapeColor) -> ImageResult<()> {

        if ! (
            self.width == filler.width
                && self.width == shape.width
                && self.height == filler.height
                && self.height == shape.height
        ) {
            return Err(ImageError::DifferentDimensions);
        }

        for (pixel_index, pixel) in self.pixels.iter_mut().enumerate() {

            if shape.pixels[pixel_index] == shape_color {
                filler.pixels[pixel_index].overdraw_on(pixel);
            }

        }

        Ok(())
    }

}



pub trait ToImageResult<T> {
    fn to_image_result(self) -> ImageResult<T>;
}

impl<T> ToImageResult<T> for Result<T, DecodingError> {
    fn to_image_result(self) -> ImageResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(ImageError::PngLoad(error)),
        }
    }
}



pub type ImageResult<T> = Result<T, ImageError>;

pub enum ImageError {

    PngLoad(DecodingError),
    DifferentDimensions,

}
