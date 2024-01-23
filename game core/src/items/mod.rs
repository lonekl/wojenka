pub mod units;

use std::sync::Arc;
use crate::Definitions;
use crate::items::units::{DeterminedAmount, AmountType, CaloriesRate, RawAmount};



pub struct ItemType {

    pub name: &'static str,
    pub amount_type: AmountType,
    pub production_per_person: RawAmount,
    pub max_production_per_tile: RawAmount,
    pub type_values: ItemTypeValues,

}

impl ItemType {

    pub fn new(
        name: &'static str,
        amount_type: AmountType,
        production_per_person: RawAmount,
        max_production_per_tile: RawAmount,
        type_values: ItemTypeValues,
    ) -> Self {

        Self {
            name,
            amount_type,
            production_per_person,
            max_production_per_tile,
            type_values,
        }
    }

}



pub enum ItemTypeValues {

    None,
    Food {
        calories_rate: CaloriesRate,
    },

}



pub struct Item {

    type_id: usize,
    amount: RawAmount,

}

impl Item {

    pub fn new(definitions: Arc<Definitions>, type_id: usize, amount: DeterminedAmount) -> Self {

        Self::new_checked(definitions, type_id, amount).expect("Item creation error. Wrong amount type.")
    }

    pub fn new_checked(definitions: Arc<Definitions>, type_id: usize, amount: DeterminedAmount) -> Option<Self> {
        let item_type = &definitions.item_types[type_id];

        if item_type.amount_type != amount.get_amount_type() {

            return None;
        }

        Some ( Self {
            type_id,
            amount: amount.get_raw_amount(),
        })
    }

}
