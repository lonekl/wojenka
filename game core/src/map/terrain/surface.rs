use std::fs::File;
use std::path::PathBuf;
use crate::error::{CoreResult, OptionToCoreError, ResultToCoreError, CoreError};
use crate::image::color::Rgb8;
use crate::image::{ImageDimensions, Image, ImageResult};
use crate::map::Map;
use crate::map::units::TerrainPart;



#[derive(Clone)]
pub struct TileSurface {

    part: TerrainPart,
    variant: usize,

}

impl TileSurface {

    pub fn new(part: TerrainPart, variant: usize) -> Self {

        Self {
            part,
            variant,
        }
    }

}



pub struct SurfaceTypes {

    layers: Image<Rgb8>,
    pub types: Vec<SurfaceType>,
    texture_levels: u8,

}

impl SurfaceTypes {

    pub fn new(directory_paths: Vec<PathBuf>, texture_levels: u8) -> CoreResult<Self> {
        let mut types = vec![];

        for directory_path in directory_paths {
            types.push(SurfaceType::new(directory_path, 1)?);
        }

        let layer_image_file = File::open("game sets/historical/surface/layers.png").to_core_error()?;
        let layers = Image::load_png(layer_image_file).to_core_error()?;

        Ok(Self {
            layers,
            types,
            texture_levels,
        })
    }

    pub fn build_surface_texture(&self, map: &Map) -> ImageResult<Image<Rgb8>> {
        let layers = Image::new_uniform(Rgb8::new(0, 0, 0), ImageDimensions::new(24, 24));
        let tile_dimensions = self.layers.dimensions();
        let mut surface_texture = Image::new_uniform(Rgb8::new(255, 255, 255), map.image_dimensions(tile_dimensions));

        for (tile_index, tile) in (&map.tiles).iter().enumerate() {
            let tile_pos_tuple = map.index_to_xy(tile_index);
            let tile_pos = ImageDimensions::from_u32_tuple(tile_pos_tuple);
            let image_tile_pos = tile_dimensions * tile_pos;

            surface_texture.overdraw_shaped_image(
                &self.types[0].variants[0],
                image_tile_pos,
                &layers,
                Rgb8::new(0, 0, 0),
            )?;

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
