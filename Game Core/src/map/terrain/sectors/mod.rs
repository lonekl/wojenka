use crate::population::Population;
use crate::units::Time;



pub struct TileSectorType {

    pub name: &'static str,
    pub reward_frequency: Time,
    pub reward_offset: Time,

}

impl TileSectorType {

    pub fn new(name: &'static str, reward_frequency: Time, reward_offset: Time) -> Self {
        Self {
            name,
            reward_frequency,
            reward_offset,
        }
    }


    pub fn last_rewards(&self, current_time: Time) -> Time {
        let next_rewards = current_time + self.reward_offset;

        next_rewards - next_rewards % self.reward_frequency
    }

    pub fn is_getting_rewards(&self, last_rewards: Time, current_time: Time) -> bool {
        last_rewards + self.reward_frequency >= current_time
    }

}



#[derive(Clone)]
pub struct TileSector {

    pub population: Population,

}

impl TileSector {

    pub fn new(population: Population) -> Self {

        Self {
            population,
        }
    }

}
