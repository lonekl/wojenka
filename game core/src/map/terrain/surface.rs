use crate::map::units::TerrainPart;



pub struct TileSurface {

    part: TerrainPart,

}



pub struct SurfaceTypes {

    types: Vec<SurfaceType>,
    texture_levels: u8,

}

impl SurfaceTypes {

    pub fn new(types: Vec<SurfaceType>, texture_levels: u8) -> Self {

        Self {
            types,
            texture_levels,
        }
    }

}



pub struct SurfaceType {

    name: String,

}

impl SurfaceType {

    pub fn new(name: String) -> Self {

        Self {
            name,
        }
    }

}
