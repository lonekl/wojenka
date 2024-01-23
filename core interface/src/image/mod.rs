use crate::image::color::Overdraw;

pub mod color;



pub struct Image<Color: PartialEq + Clone> {

    pixels: Vec<Color>,
    width: usize,
    height: usize,

}

impl<Color: PartialEq + Clone> Image<Color> {

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


    pub fn overdraw_shaped_image<
        Filler: Overdraw<Color> + PartialEq + Clone,
        ShapeColor: PartialEq + Clone,
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



pub type ImageResult<T> = Result<T, ImageError>;

#[derive(Clone, Copy)]
pub enum ImageError {

    DifferentDimensions,

}
