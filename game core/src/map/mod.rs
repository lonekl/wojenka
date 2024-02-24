pub mod tile;
pub mod units;

use std::sync::Arc;
use crate::map::units::{TerrainHeight};
use crate::units::Time;
use crate::Definitions;
use crate::image::ImageDimensions;
use crate::map::tile::{TileArray, TileLocal};
use crate::map::tile::surface::TileSurface;



pub struct Map {

    definitions: Arc<Definitions>,
    pub properties: MapSettings,

    tiles: TileArray,

}

impl Map {

    pub fn new(definitions: Arc<Definitions>, properties: MapSettings, _start_time: Time) -> Self {
        let tile_amount = properties.shape.tile_amount() as usize;
        let mut tile_array = TileArray::new(
            definitions.clone(),
            TileLocal::new(3, Box::new([TileSurface::new(0, 3)])),
            tile_amount as u32
        );

        for tile_y in 0..30 {
            for tile_x in 0..30 {
                let tile_index = (tile_y * 30 + tile_x) as u32;
                let mut result_tile = tile_array.index(tile_index).to_local();

                let east_dessert = (tile_y - 17).max(17 - tile_y)
                    + 10 - tile_x / 3;

                if east_dessert < 7 {
                    result_tile.surface[0] = TileSurface::new(1, 0);
                }

                let from_center_distance = (tile_x - 15).max(15 - tile_x) + (tile_y - 15).max(15 - tile_y);
                let from_first_mountain_pass_distance = (tile_x - 12).max(12 - tile_x) + (tile_y - 12).max(12 - tile_y);
                let from_second_mountain_pass_distance = (tile_x - 11).max(11 - tile_x) + (tile_y - 10).max(10 - tile_y);
                let from_second_mountain_distance = (tile_x - 8).max(8 - tile_x) + (tile_y - 6).max(6 - tile_y);

                let from_mountain_distance = from_center_distance
                    .min(from_second_mountain_distance)
                    .min(from_first_mountain_pass_distance + 2)
                    .min(from_second_mountain_pass_distance + 2);

                if from_mountain_distance < 6 {
                    let mut height = (6 - from_mountain_distance) * 10;
                    height *= height;
                    result_tile.main.height += TerrainHeight::from_meters(height);
                    result_tile.surface[0] = TileSurface::new(2, 2 - from_mountain_distance as usize / 2);
                }

                let _ = tile_array.put(tile_index, result_tile);
            }
        }

        Self {
            definitions,
            properties,
            tiles: tile_array,
            //tile_sectors,
        }
    }



    pub fn tick(&mut self, _tick_length: Time) {

        /*for (sector_type_index, sector_tiles) in self.tile_sectors.iter_mut().enumerate() {

            if sector_tiles.rewards_condition(definitions, sector_type_index, tick_length) {

                // TODO rewards.

            }

        }*/

    }



    pub fn image_dimensions(&self, tile_image_dimensions: ImageDimensions) -> ImageDimensions {

        tile_image_dimensions * ImageDimensions::from_u32_tuple(self.properties.shape.max_axis())
    }

    pub fn get_terrain(&self) -> (&MapSettings, &TileArray) {

        (&self.properties, &self.tiles)
    }

}



/*pub struct SectorTiles {

    last_rewards: Time,

    tiles: Vec<TileSector>,

}

impl SectorTiles {

    pub fn rewards_condition(&mut self, definitions: &Arc<Definitions>, sector_type_index: usize, current_time: Time) -> bool {
        let sector_type = &definitions.tile_sector_types[sector_type_index];
        let is_getting_rewards = definitions.tile_sector_types[sector_type_index].is_getting_rewards(self.last_rewards, current_time);

        if is_getting_rewards {
            self.last_rewards += sector_type.reward_frequency;
        }

        is_getting_rewards
    }

}*/



pub struct MapSettings {

    pub tile_size: TerrainHeight,

    pub shape: MapShape,

}

impl MapSettings {

    pub fn new(tile_size: TerrainHeight, shape: MapShape) -> Self {

        Self {
            tile_size,
            shape,
        }
    }

}



#[derive(Clone, Copy)]
pub enum MapShape {

    Rectangular { width: u32, height: u32 }

}

impl MapShape {

    pub fn raw_index(&self, x: u32, y: u32) -> usize {

        match self {
            MapShape::Rectangular { width, .. } => (y * width + x) as usize,
        }
    }

    pub fn coordinates(&self, index: usize) -> (u32, u32) {
        let u32_index = index as u32;

        match self {
            MapShape::Rectangular { width, .. } => (u32_index % width, u32_index / width),
        }
    }



    pub fn tile_amount(&self) -> u32 {

        match self {
            MapShape::Rectangular { width, height } => width * height,
        }
    }

    pub fn max_axis(&self) -> (u32, u32) {

        match self {
            MapShape::Rectangular { width, height } => (*width, *height),
        }
    }

}
