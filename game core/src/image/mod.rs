use std::io::Read;
use std::ops::{Add, Mul};
use std::time::Duration;
use png::{Decoder, DecodingError, OutputInfo};
use crate::image::color::{ColorFn, Overdraw, Rgb8, Rgba8};

pub mod color;
pub mod file_load;



pub struct Image<Color: ColorFn + PartialEq + Clone + Copy + From<Rgb8> + From<Rgba8>> {

    pixels: Vec<Color>,
    dimensions: Dimensions,

}

impl<Color: ColorFn + PartialEq + Clone + Copy + From<Rgb8> + From<Rgba8>> Image<Color> {

    pub fn new_raw(image: Vec<Color>, dimensions: Dimensions) -> ImageResult<Self> {
        if dimensions.len() != image.len() {
            return Err(ImageError::DifferentDimensions);
        }

        Ok(Self {
            pixels: image,
            dimensions,
        })
    }

    pub fn new_uniform(filler: Color, dimensions: Dimensions) -> Self {

        Self {
            pixels: vec![filler; dimensions.len()],
            dimensions,
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

        Self::new_raw(image, Dimensions::from_png_info(info))
    }



    pub fn overdraw_shaped_image<
        Filler:     ColorFn + PartialEq + Clone + Copy + From<Rgb8> + From<Rgba8> + Overdraw<Color>,
        ShapeColor: ColorFn + PartialEq + Clone + Copy + From<Rgb8> + From<Rgba8>,
    >(&mut self, filler: Image<Filler>, draw_offset: Dimensions, shape: Image<ShapeColor>, shape_color: ShapeColor) -> ImageResult<()> {

        if ! (filler.dimensions == shape.dimensions) {
            return Err(ImageError::DifferentDimensions);
        }

        for filler_position in filler.dimensions {
            let raw_filler_position = filler_position.len();
            let self_position = filler_position + draw_offset;
            let raw_self_position = self_position.len();

            if shape.pixels[raw_filler_position] == shape_color {
                filler.pixels[raw_filler_position].overdraw_on(&mut self.pixels[raw_self_position]);
            }
        }

        Ok(())
    }



    pub fn raw_u8_data(&self) -> Vec<u8>
        where [(); Color::BYTE_LENGTH]: Sized
    {
        let mut data = vec![];
        data.reserve(self.pixels.len() * Color::BYTE_LENGTH);

        for pixel in &self.pixels {
            for pixe in pixel.to_raw_bytes() {
                data.push(pixe);
            }
        }

        data
    }

    pub fn dimensions(&self) -> Dimensions {

        self.dimensions
    }

    pub fn u32_dimension_tuple(&self) -> (u32, u32) {

        (self.dimensions.x as u32, self.dimensions.y as u32)
    }

}



pub struct DimensionIterator {

    current: Dimensions,
    limit: Dimensions,

}

impl Iterator for DimensionIterator {
    type Item = Dimensions;

    fn next(&mut self) -> Option<Self::Item> {

        self.current.x += 1;

        if self.current.x == self.limit.x {
            self.current.y += 1;

            if self.current.y == self.limit.y {
                return None;
            }
        }

        Some(self.current)
    }
}



#[derive(Clone, Copy, PartialEq)]
pub struct Dimensions {

    pub x: usize,
    pub y: usize,

}

impl Dimensions {

    pub const ZERO: Dimensions = Dimensions { x: 0, y: 0 };

    pub fn from_png_info(info: OutputInfo) -> Self {

        Self::new(info.width as usize, info.height as usize)
    }

    pub fn new(x: usize, y: usize) -> Self {

        Self { x, y }
    }


    pub fn index_on_bigger_image(&self, width: usize) -> usize {

        self.x + self.y * width
    }

    pub fn len(&self) -> usize {

        self.x * self.y
    }

}

impl IntoIterator for Dimensions {
    type Item = Dimensions;
    type IntoIter = DimensionIterator;

    fn into_iter(self) -> Self::IntoIter {

        DimensionIterator {
            current: Dimensions::ZERO,
            limit: self,
        }
    }
}

impl Add<Dimensions> for Dimensions {
    type Output = Dimensions;

    fn add(self, rhs: Dimensions) -> Self::Output {

        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<Dimensions> for Dimensions {
    type Output = Dimensions;

    fn mul(self, rhs: Dimensions) -> Self::Output {

        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
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

#[derive(Debug)]
pub enum ImageError {

    PngLoad(DecodingError),
    DifferentDimensions,

}
