use std::sync::Arc;
use crate::items::{ItemType, ItemTypeValues};
use crate::items::units::{AmountType, Calories, CaloriesRate, Weight};
use crate::map::{Map, MapShape};
use crate::map::terrain::sectors::TileSectorType;
use crate::map::terrain::surface::{SurfaceType, SurfaceTypes};
use crate::powers::Power;
use crate::units::Time;

pub mod image;
pub mod items;
pub mod map;
pub mod population;
pub mod powers;
pub mod units;



pub struct Game {

    definitions: Arc<Definitions>,
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
                Power::new("Poland".to_string()),
                Power::new("Ukraine".to_string()),
                Power::new("Hungary".to_string()),
            ],

            world_time,

            map: Map::new(definitions.clone(), MapShape::Rectangular { width: 10, height: 10}, world_time),
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
                    SurfaceType::new(format!("grass"), 1),
                ],
                8
            ),



        }
    }

}
