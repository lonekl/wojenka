pub mod terrain;
pub mod units;

use std::sync::Arc;
use crate::map::terrain::sectors::{TileSector};
use crate::map::units::{HeightVariation, TerrainHeight, TerrainPart};
use crate::population::Population;
use crate::units::Time;
use crate::Definitions;
use crate::map::terrain::surface::TileSurface;

pub struct Map {

    shape: MapShape,
    tiles: Vec<Tile>,
    tile_sectors: Vec<SectorTiles>,
    tile_surfaces: Vec<TileSurface>,

}

impl Map {

    pub fn new(definitions: Arc<Definitions>, shape: MapShape, start_time: Time) -> Self {
        let tile_amount = shape.tile_amount() as usize;
        let mut tile_sectors = vec![];
        let mut tile_surfaces = vec![];

        for tile_sector_type in &definitions.tile_sector_types {
            let last_rewards = tile_sector_type.last_rewards(start_time);

            let sector_tile = TileSector::new(Population::new(1000));
            let sector_tiles = vec![sector_tile; tile_amount];

            tile_sectors.push(SectorTiles {
                last_rewards,
                tiles: sector_tiles,
            });

        }

        for _surface_type in definitions.surface_types {

            tile_surfaces.push(SurfaceTiles {
                tiles: vec![TileSurface::new(TerrainPart::from_8bit_scale(255), 0); tile_amount],
            })

        }

        Self {
            shape,
            tiles: {
                let mut result = vec![Tile::new(3); tile_amount];

                result[25].height = TerrainHeight::from_meters(3000);
                result[35].height = TerrainHeight::from_meters(7500);

                result[11].height = TerrainHeight::from_meters(-10000);

                result
            },
            tile_sectors,
        }
    }


    fn x_y_index(&self, x: u32, y: u32) -> usize {

        match self.shape {
            MapShape::Rectangular { width, .. } => (x + y * width) as usize,
        }
    }


    pub fn tick(&mut self, definitions: &Arc<Definitions>, tick_length: Time) {

        for (sector_type_index, sector_tiles) in self.tile_sectors.iter_mut().enumerate() {

            if sector_tiles.rewards_condition(definitions, sector_type_index, tick_length) {

                // TODO rewards.

            }

        }

    }


    pub fn get_terrain(&self) -> (MapShape, &Vec<Tile>) {

        (self.shape, &self.tiles)
    }

}



/// One tile has area of: 10km * 10km, or 100km².
#[derive(Clone)]
pub struct Tile {

    pub height: TerrainHeight,
    pub height_variation: HeightVariation,

    /// What power is owner of that terrain.
    owner: usize,

}

impl Tile {

    pub fn new(owner: usize) -> Self {

        Self {
            height: TerrainHeight::from_meters(10),
            height_variation: HeightVariation::from_meters(3),

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



pub struct SurfaceTiles {

    tiles: Vec<TileSurface>,

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
