use crate::map::tile::surface::TileSurface;
use crate::map::units::TerrainHeight;

pub mod sectors;
pub mod surface;



pub struct TileMap {

    surface_layers: usize,

    tile_bytes: Vec<u8>,

}

impl TileMap {



}



pub struct Tile<'a> {

    pub main: &'a mut TileSizedData,
    surface: &'a mut [TileSurface],

}

impl<'a> Tile<'a> {

}



pub struct TileSizedData {

    height: TerrainHeight,

    owner: usize,

}
