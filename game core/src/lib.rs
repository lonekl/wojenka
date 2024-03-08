#![feature(generic_const_exprs)]

use std::sync::Arc;
use crate::image::ImageDimensions;
use crate::items::{ItemType, ItemTypeValues};
use crate::items::units::{AmountType, Calories, CaloriesRate, Weight};
use crate::map::{Map, MapSettings, MapShape};
use crate::map::tile::sectors::TileSectorType;
use crate::map::tile::surface::{SurfaceTypes};
use crate::map::units::TerrainHeight;
use crate::powers::Power;
use crate::units::Time;

pub mod image;
pub mod items;
pub mod map;
pub mod population;
pub mod powers;
pub mod error;
pub mod units;



pub struct Game {

    pub definitions: Arc<Definitions>,
    pub powers: Vec<Power>,

    world_time: Time,

    pub map: Map,

}

impl Game {

    pub fn new() -> Self {
        let definitions = Arc::new(Definitions::new_default());
        let world_time = Time::from_years(1918);

        Self {
            definitions: definitions.clone(),
            powers: vec![
                Power::new("no state".to_string()),
                Power::new("Second Polish Republic".to_string()),
                Power::new("Arab Republic of Egypt".to_string()),
                Power::new("Kingdom of Hungary".to_string()),
            ],

            world_time,

            map: Map::new(
                definitions.clone(),
                MapSettings::new(
                    TerrainHeight::from_meters(1_000),
                    MapShape::Rectangular { width: 30, height: 30}),
                world_time
            ),
        }
    }

}



pub struct Definitions {

    pub item_types: Vec<ItemType>,
    pub tile_sector_types: Vec<TileSectorType>,
    pub surface_types: SurfaceTypes,

}

impl Definitions {

    pub fn new_default() -> Self {

        Self {



            item_types: vec![
                ItemType::new(
                    "maize",
                    AmountType::Weight,
                    Weight::from_tons(20).as_raw_amount(),
                    Weight::from_tons(3500).as_raw_amount(),
                    ItemTypeValues::Food {
                        calories_rate: CaloriesRate::from_x_per_kilo_grams(Calories::from_kcal(3650)),
                    },
                ),
            ],



            tile_sector_types: vec![
                TileSectorType::new(
                    "farm",
                    Time::YEAR,
                    Time::from_days(280),
                ),
            ],



            surface_types: SurfaceTypes::new(
                vec![
                    vec![
                        ("game sets/historical/surface/plains".into(), 4),
                        ("game sets/historical/surface/dessert".into(), 1),
                        ("game sets/historical/surface/mountains".into(), 3),
                    ]
                ],
                ImageDimensions::new(64, 64),
            ).unwrap(),



        }
    }

}
