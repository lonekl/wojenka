pub mod terrain;
pub mod units;

use std::sync::Arc;
use crate::map::terrain::sectors::{TileSector};
use crate::map::units::{HeightVariation, TerrainHeight};
use crate::population::Population;
use crate::units::Time;
use crate::Definitions;
use crate::image::ImageDimensions;
use crate::map::terrain::surface::TileSurface;



pub struct Map {

    shape: MapShape,
    tile_size: TerrainHeight,

    tiles: Vec<Tile>,
    tile_sectors: Vec<SectorTiles>,

}

impl Map {

    pub fn new(definitions: Arc<Definitions>, shape: MapShape, tile_size: TerrainHeight, start_time: Time) -> Self {
        let tile_amount = shape.tile_amount() as usize;
        let mut tile_sectors = vec![];

        for tile_sector_type in &definitions.tile_sector_types {
            let last_rewards = tile_sector_type.last_rewards(start_time);

            let sector_tile = TileSector::new(Population::new(1000));
            let sector_tiles = vec![sector_tile; tile_amount];

            tile_sectors.push(SectorTiles {
                last_rewards,
                tiles: sector_tiles,
            });

        }

        Self {
            shape,
            tile_size,
            tiles: {
                let mut tiles = vec![
                    Tile::new(
                        3,
                        TileSurface::new(0, 1),
                    );
                    tile_amount
                ];

                for tile_y in 0..30 {
                    for tile_x in 0..30 {
                        let tile_index = tile_y as usize * 30 + tile_x as usize;

                        let east_dessert = (tile_y - 17).max(17 - tile_y)
                            + 10 - tile_x / 3;

                        if east_dessert < 7 {
                            tiles[tile_index].surface = TileSurface::new(1, 0);
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
                            tiles[tile_index].height += TerrainHeight::from_meters(height);
                            tiles[tile_index].surface = TileSurface::new(2, 2 - from_mountain_distance as usize / 2);
                        }

                    }
                }

                tiles
            },
            tile_sectors,
        }
    }



    pub fn tick(&mut self, definitions: &Arc<Definitions>, tick_length: Time) {

        for (sector_type_index, sector_tiles) in self.tile_sectors.iter_mut().enumerate() {

            if sector_tiles.rewards_condition(definitions, sector_type_index, tick_length) {

                // TODO rewards.

            }

        }

    }



    fn xy_to_index(&self, x: u32, y: u32) -> usize {

        match self.shape {
            MapShape::Rectangular { width, .. } => (x + y * width) as usize,
        }
    }

    pub fn index_to_xy(&self, index: usize) -> (u32, u32) {

        match self.shape {
            MapShape::Rectangular { width, .. } => (index as u32 % width, index as u32 / width),
        }
    }

    pub fn image_dimensions(&self, tile_dimensions: ImageDimensions) -> ImageDimensions {

        match self.shape {
            MapShape::Rectangular { width, height } => tile_dimensions * ImageDimensions::new(width as usize, height as usize),
        }
    }

    pub fn get_terrain(&self) -> (MapShape, TerrainHeight, &Vec<Tile>) {

        (self.shape, self.tile_size, &self.tiles)
    }



    /*pub fn iter_tiles(&self) -> MapTileIterator {

        MapTileIterator {
            tiles: &self.tiles,
            shape: self.shape,

            current: 0,
        }
    }*/

}



/*pub struct MapTileIterator<'a> {

    tiles: &'a [Tile],
    shape: MapShape,

    current: usize,

}

impl<'a> Iterator for MapTileIterator<'a> {
    type Item = (&'a Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.tiles.len() {

            return None;
        }

        let tile = &self.tiles[self.current];
        self.current += 1;

        Some(tile)
    }
}*/



#[derive(Clone)]
pub struct Tile {

    pub height: TerrainHeight,
    pub height_variation: HeightVariation,

    pub surface: TileSurface,

    /// What power is owner of that terrain.
    owner: usize,

}

impl Tile {

    pub fn new(owner: usize, surface: TileSurface) -> Self {

        Self {
            height: TerrainHeight::from_meters(10),
            height_variation: HeightVariation::from_meters(3),

            surface,

            owner,
        }
    }

}



pub struct SectorTiles {

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

}



#[derive(Clone, Copy)]
pub enum MapShape {

    Rectangular { width: u32, height: u32 }

}

impl MapShape {

    pub fn tile_amount(&self) -> u32 {

        match self {
            MapShape::Rectangular { width, height } => width * height,
        }
    }

}
