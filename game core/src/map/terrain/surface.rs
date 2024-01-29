use std::fs::File;
use std::path::PathBuf;
use crate::error::{AssetLoadResult, ToInvalidPath, ToFileOp, ImageLoadError};
use crate::image::color::Rgb8;
use crate::image::Image;
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

    pub types: Vec<SurfaceType>,
    texture_levels: u8,

}

impl SurfaceTypes {

    pub fn new(directory_paths: Vec<PathBuf>, texture_levels: u8) -> AssetLoadResult<Self> {
        let mut types = vec![];

        for directory_path in directory_paths {
            types.push(SurfaceType::new(directory_path, 1)?);
        }

        Ok(Self {
            types,
            texture_levels,
        })
    }

}



pub struct SurfaceType {

    name_id: String,
    variants: Vec<Image<Rgb8>>,

}

impl SurfaceType {

    pub fn new(directory_path: PathBuf, variant_amount: usize) -> AssetLoadResult<Self> {
        let name_id = directory_path.file_name().to_invalid_path_error()?.to_string_lossy().into();

        let mut variants = vec![];
        for variant_index in 0..variant_amount {
            let mut variant_image_path = directory_path.clone();
            variant_image_path.push(&format!("{variant_index}.png"));

            let image_reader = File::open(variant_image_path).to_file_operation_error()?;
            let image = Image::load_png(image_reader).to_image_load_error()?;

            variants.push(image);
        }

        Ok(Self {
            name_id,
            variants,
        })
    }

}
