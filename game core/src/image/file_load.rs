use png::{BitDepth, ColorType};
use crate::image::color::{Rgb8, Rgba8};


pub fn pass_bit_depth(raw_image_buffer: &mut [u8], bit_depth: BitDepth) -> Vec<u16> {

    match bit_depth {
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
    }
}



pub fn pass_color_type<Color: From<Rgb8> + From<Rgba8>>(image_value_buffer: Vec<u16>, color_type: ColorType) -> Vec<Color> {

    match color_type {
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
        ColorType::Rgba => {
            let pixel_amount = image_value_buffer.len() / 4;
            let mut pixels = vec![];
            pixels.reserve(pixel_amount);

            for pixel_index in 0..pixel_amount {
                let value_index = pixel_index * 4;

                pixels.push(
                    Rgba8::new(
                        (image_value_buffer[value_index    ] >> 8) as u8,
                        (image_value_buffer[value_index + 1] >> 8) as u8,
                        (image_value_buffer[value_index + 2] >> 8) as u8,
                        (image_value_buffer[value_index + 3] >> 8) as u8,
                    ).into()
                );
            }

            pixels
        },
        _ => todo!("Some colors decompleted."),
    }
}
