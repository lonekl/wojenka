use std::cmp::Ordering;
use std::io::Read;
use std::ops::{Add, Div, Mul, Sub};
use png::{Decoder, DecodingError, OutputInfo};
use crate::image::color::{ColorFn, Overdraw, Rgb8, Rgba8};

pub mod color;
pub mod file_load;



pub struct Image<Color: ColorFn + PartialEq + Clone + Copy + From<Rgb8> + From<Rgba8>> {

    pixels: Vec<Color>,
    dimensions: ImageDimensions,

}

impl<Color: ColorFn + PartialEq + Clone + Copy + From<Rgb8> + From<Rgba8>> Image<Color> {

    pub fn new_raw(image: Vec<Color>, dimensions: ImageDimensions) -> ImageResult<Self> {
        if dimensions.len() != image.len() {
            return Err(ImageError::DimensionsDontMatch);
        }

        Ok(Self {
            pixels: image,
            dimensions,
        })
    }

    pub fn new_uniform(filler: Color, dimensions: ImageDimensions) -> Self {

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

        Self::new_raw(image, ImageDimensions::from_png_info(info))
    }



    pub fn overdraw_image<
        Filler: ColorFn + PartialEq + Clone + Copy + From<Rgb8> + From<Rgba8> + Overdraw<Color>,
    >(&mut self, filler: &Image<Filler>, draw_offset: ImageDimensions) -> ImageResult<()> {
        let self_filler_limit = filler.dimensions + draw_offset;
        
        if self_filler_limit > self.dimensions {
            
            return Err(ImageError::DimensionsDontMatch);
        }

        for filler_position in filler.dimensions {
            let raw_filler_position = filler_position.index_on_bigger_image(filler.dimensions.x);
            let self_position = filler_position + draw_offset;
            let raw_self_position = self_position.index_on_bigger_image(self.dimensions.x);

            filler.pixels[raw_filler_position].overdraw_on(&mut self.pixels[raw_self_position]);
        }
        
        Ok(())
    }



    pub fn overdraw_with_shaped_image<
        Filler:     ColorFn + PartialEq + Clone + Copy + From<Rgb8> + From<Rgba8> + Overdraw<Color>,
        ShapeColor: ColorFn + PartialEq + Clone + Copy + From<Rgb8> + From<Rgba8>,
    >(&mut self, filler: &Image<Filler>, draw_offset: ImageDimensions, shape: &Image<ShapeColor>, shape_color: ShapeColor) -> ImageResult<()> {

        if ! (filler.dimensions == shape.dimensions) {
            return Err(ImageError::DimensionsDontMatch);
        }

        for filler_position in filler.dimensions {
            let raw_filler_position = filler_position.index_on_bigger_image(filler.dimensions.x);
            let self_position = filler_position + draw_offset;
            let raw_self_position = self_position.index_on_bigger_image(self.dimensions.x);

            if shape.pixels[raw_filler_position] == shape_color {
                filler.pixels[raw_filler_position].overdraw_on(&mut self.pixels[raw_self_position]);
            }
        }

        Ok(())
    }

    /// Horribly ineffective operation.
    pub fn overdraw_image_rescaled<
        Filler: ColorFn + PartialEq + Clone + Copy + From<Rgb8> + From<Rgba8> + Overdraw<Color>,
    >(&mut self, filler: &Image<Filler>, draw_offset: ImageDimensions, max_position: ImageDimensions) -> ImageResult<()> {

        for filler_unscaled_position in max_position - draw_offset {
            let self_position = filler_unscaled_position + draw_offset;
            let filler_position = filler_unscaled_position * self.dimensions / filler.dimensions;

            filler.pixels[filler_position.index_on_bigger_image(filler.dimensions.x)]
                .overdraw_on(&mut self.pixels[self_position.index_on_bigger_image(self.dimensions.x)]);
        }

        Ok(())
    }



    pub fn invert_on_x(&mut self) {

        for y in 0..self.dimensions.y {
            let index_by_y = y * self.dimensions.x;
            let row_copy = self.pixels[index_by_y..index_by_y + self.dimensions.x].to_vec();

            for x in 0..self.dimensions.x {
                self.pixels[index_by_y + x] = row_copy[self.dimensions.x - x - 1];
            }

        }

    }

    pub fn invert_on_y(&mut self) {
        let image_copy = self.pixels.clone();

        for y in 0..self.dimensions.y {
            let index_by_y = y * self.dimensions.x;
            let alt_index_by_y = (self.dimensions.y - y - 1) * self.dimensions.x;

            for x in 0..self.dimensions.x {
                self.pixels[index_by_y + x] = image_copy[alt_index_by_y + x];
            }

        }

    }



    pub fn raw_u8_bytes(&self) -> Vec<u8>
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

    pub fn dimensions(&self) -> ImageDimensions {

        self.dimensions
    }

}



pub struct ImageDimensionIterator {

    current: ImageDimensions,
    limit: ImageDimensions,

}

impl Iterator for ImageDimensionIterator {
    type Item = ImageDimensions;

    fn next(&mut self) -> Option<Self::Item> {
        let original_current = self.current;

        if self.current.y == self.limit.y {
            return None;
        }

        self.current.x += 1;

        if self.current.x == self.limit.x {
            self.current.x = 0;
            self.current.y += 1;
        }

        Some(original_current)
    }
}



#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ImageDimensions {

    pub x: usize,
    pub y: usize,

}

impl ImageDimensions {

    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn from_png_info(info: OutputInfo) -> Self {

        Self::new(info.width as usize, info.height as usize)
    }

    pub fn from_u32_tuple(tuple: (u32, u32)) -> Self {

        Self::new(tuple.0 as usize, tuple.1 as usize)
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



    pub fn to_u32_tuple(&self) -> (u32, u32) {

        (self.x as u32, self.y as u32)
    }

}

impl IntoIterator for ImageDimensions {
    type Item = Self;
    type IntoIter = ImageDimensionIterator;

    fn into_iter(self) -> Self::IntoIter {

        ImageDimensionIterator {
            current: ImageDimensions::ZERO,
            limit: self,
        }
    }
}

impl Add<ImageDimensions> for ImageDimensions {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {

        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<ImageDimensions> for ImageDimensions {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {

        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<ImageDimensions> for ImageDimensions {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {

        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div<ImageDimensions> for ImageDimensions {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {

        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

/// Works in slightly other way, than you would expect.
impl PartialOrd for ImageDimensions {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }

    fn lt(&self, other: &Self) -> bool {

        self.x < other.x || self.y < other.y
    }

    fn le(&self, other: &Self) -> bool {

        self.x <= other.x || self.y <= other.y
    }

    fn gt(&self, other: &Self) -> bool {

        self.x > other.x || self.y > other.y
    }

    fn ge(&self, other: &Self) -> bool {

        self.x >= other.x || self.y >= other.y
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
    DimensionsDontMatch,

}
