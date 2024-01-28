use std::path::PathBuf;
use crate::AssetLoadError;
use crate::image::color::Rgb8;
use crate::image::Image;
use crate::map::units::TerrainPart;



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

    types: Vec<SurfaceType>,
    texture_levels: u8,

}

impl SurfaceTypes {

    pub fn new(directory_paths: Vec<PathBuf>, texture_levels: u8) -> Self {

        Self {
            types: vec![],
            texture_levels,
        }
    }

}



pub struct SurfaceType {

    name_id: String,
    variants: Vec<Image<Rgb8>>,

}

impl SurfaceType {

    pub fn new(directory_path: PathBuf, variant_amount: usize) -> Result<Self, AssetLoadError> {
        let a = directory_path.file_name();

        Ok(Self {
            name_id: directory_path.file_name().unwrap().to_string_lossy().into(),
            variants: vec![],
        })
    }

}
