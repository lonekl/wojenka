use std::io::Read;
use png::{BitDepth, ColorType, Decoder, DecodingError};
use crate::image::color::{Overdraw, Rgb8};

pub mod color;



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
        let image_value_buffer = match info.bit_depth {
            BitDepth::One => {
                let mut values = vec![];
                values.reserve(raw_image_buffer.len() * 8);

                for raw_image_byte in raw_image_buffer.iter_mut() {
                    for _ in 0..8 {
                        values.push((*raw_image_byte as u16 & 0b1) * u16::MAX);

                        *raw_image_byte >>= 1;
                    }
                }

                values
            },
            BitDepth::Two => {
                let mut values = vec![];
                values.reserve(raw_image_buffer.len() * 4);

                for raw_image_byte in raw_image_buffer.iter_mut() {
                    for _ in 0..4 {
                        values.push((*raw_image_byte as u16 & 0b11) * (u16::MAX / 0b11));

                        *raw_image_byte >>= 2;
                    }
                }

                values
            },
            BitDepth::Four => {
                let mut values = vec![];
                values.reserve(raw_image_buffer.len() * 2);

                for raw_image_byte in raw_image_buffer.iter_mut() {
                    for _ in 0..2 {
                        values.push((*raw_image_byte as u16 & 0b1111) * (u16::MAX / 0b1111));

                        *raw_image_byte >>= 2;
                    }
                }

                values
            },
            BitDepth::Eight => {
                let mut values = vec![];
                values.reserve(raw_image_buffer.len());

                for raw_image_byte in raw_image_buffer {
                    values.push(*raw_image_byte as u16 * u8::MAX as u16);
                }

                values
            },
            BitDepth::Sixteen => {
                let mut values = vec![0; raw_image_buffer.len()];
                values.reserve(raw_image_buffer.len() / 2);

                for (value_index, value) in values.iter_mut().enumerate() {
                    let raw_data_index = value_index * 2;
                    *value = raw_image_buffer[raw_data_index] as u16 | ((raw_image_buffer[raw_data_index + 1] as u16) << 8);
                }

                values
            },
        };

        let image = match info.color_type {
            ColorType::Rgb => {
                let pixel_amount = image_value_buffer.len() / 3;
                let mut pixels = vec![];
                pixels.reserve(pixel_amount);

                for pixel_index in 0..pixel_amount {
                    let value_index = pixel_index * 3;

                    pixels.push(
                        Rgb8::new(
                            (image_value_buffer[value_index    ] >> 8) as u8,
                            (image_value_buffer[value_index + 1] >> 8) as u8,
                            (image_value_buffer[value_index + 2] >> 8) as u8,
                        ).into()
                    );
                }

                pixels
            },
            _ => todo!("Some colors decompleted."),
        };

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
