use std::fs::File;
use std::path::PathBuf;
use crate::error::{CoreResult, OptionToCoreError, ResultToCoreError, CoreError};
use crate::image::color::Rgb8;
use crate::image::{ImageDimensions, Image, ImageResult};
use crate::map::Map;



#[derive(Clone)]
pub struct TileSurface {

    type_id: usize,
    variant: usize,

}

impl TileSurface {

    pub fn new(type_id: usize, variant: usize) -> Self {

        Self {
            type_id,
            variant,
        }
    }

}



pub struct SurfaceTypes {

    tile_image_dimensions: ImageDimensions,
    pub types: Vec<SurfaceType>,

}

impl SurfaceTypes {

    pub fn new(directory_paths: Vec<(PathBuf, usize)>, tile_dimensions: ImageDimensions) -> CoreResult<Self> {
        let mut types = vec![];

        for (directory_path, variant) in directory_paths {
            types.push(SurfaceType::new(directory_path, variant)?);
        }

        Ok(Self {
            tile_image_dimensions: tile_dimensions,
            types,
        })
    }

    pub fn build_surface_texture(&self, map: &Map) -> ImageResult<Image<Rgb8>> {
        let mut surface_texture = Image::new_uniform(Rgb8::new(255, 255, 255), map.image_dimensions(self.tile_image_dimensions));

        for (tile_index, tile) in (&map.tiles).iter().enumerate() {
            let tile_pos_tuple = map.index_to_xy(tile_index);
            let tile_pos = ImageDimensions::from_u32_tuple(tile_pos_tuple);
            let image_tile_pos = self.tile_image_dimensions * tile_pos;

            surface_texture.overdraw_image(&self.types[tile.surface.type_id].variants[tile.surface.variant], image_tile_pos)?;
        }

        Ok(surface_texture)
    }

}



pub struct SurfaceType {

    name_id: String,
    variants: Vec<Image<Rgb8>>,

}

impl SurfaceType {

    pub fn new(directory_path: PathBuf, variant_amount: usize) -> CoreResult<Self> {
        let name_id = directory_path.file_name().to_core_error(CoreError::InvalidPath)?.to_string_lossy().into();

        let mut variants = vec![];
        for variant_index in 0..variant_amount {
            let mut variant_image_path = directory_path.clone();
            variant_image_path.push(&format!("{variant_index}.png"));

            let image_reader = File::open(variant_image_path).to_core_error()?;
            let image = Image::load_png(image_reader).to_core_error()?;

            variants.push(image);
        }

        Ok(Self {
            name_id,
            variants,
        })
    }

}
