use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(Clone, Copy, PartialEq)]
pub enum AmountType {

    Weight,
    Count,

}



#[derive(Clone, Copy)]
pub struct DeterminedAmount {

    amount_type: AmountType,
    raw_amount: RawAmount,

}

impl DeterminedAmount {

    pub fn new_weighted(weight: Weight) -> Self {

        Self {
            amount_type: AmountType::Weight,
            raw_amount: RawAmount ( weight.0),
        }
    }

    pub fn new_counted(count: u64) -> Self {

        Self {
            amount_type: AmountType::Count,
            raw_amount: RawAmount ( count),
        }
    }


    pub fn get_amount_type(&self) -> AmountType {

        self.amount_type
    }

    pub fn get_raw_amount(&self) -> RawAmount {

        self.raw_amount
    }

}


#[derive(Clone, Copy)]
pub struct RawAmount ( u64);



#[derive(Clone, Copy)]
pub struct Weight ( u64);

impl Weight {

    pub fn from_milli_grams(milli_grams: u64) -> Self {

        Self ( milli_grams)
    }

    pub fn from_grams(grams: u64) -> Self {

        Self ( grams * 1_000)
    }

    pub fn from_kilo_grams(kilo_grams: u64) -> Self {

        Self ( kilo_grams * 1_000_000)
    }

    pub fn from_tons(tons: u64) -> Self {

        Self ( tons * 1_000_000_000)
    }

    pub fn from_kilo_tons(kilo_tons: u64) -> Self {

        Self ( kilo_tons * 1_000_000_000_000)
    }

    pub fn from_mega_tons(mega_tons: u64) -> Self {

        Self ( mega_tons * 1_000_000_000_000_000)
    }

    pub fn from_giga_tons(giga_tons: u64) -> Self {

        Self ( giga_tons * 1_000_000_000_000_000_000)
    }


    pub fn as_milli_grams(self) -> u64 {

        self.0
    }

    pub fn as_grams(self) -> u64 {

        self.0 / 1_000
    }

    pub fn as_kilo_grams(self) -> u64 {

        self.0 / 1_000_000
    }

    pub fn as_tons(self) -> u64 {

        self.0 / 1_000_000_000
    }

    pub fn as_kilo_tons(self) -> u64 {

        self.0 / 1_000_000_000_000
    }

    pub fn as_mega_tons(self) -> u64 {

        self.0 / 1_000_000_000_000_000
    }

    pub fn as_giga_tons(self) -> u64 {

        self.0 / 1_000_000_000_000_000_000
    }


    pub fn as_raw_amount(self) -> RawAmount {

        RawAmount ( self.0)
    }

}

impl Mul<u64> for Weight {
    type Output = Weight;

    fn mul(self, rhs: u64) -> Self::Output {
        Self ( self.0 * rhs)
    }
}



#[derive(Clone, Copy)]
pub struct Calories ( u64);

impl Calories {

    pub fn from_cal(calories: u64) -> Self {

        Self ( calories)
    }

    pub fn from_kcal(kilo_calories: u64) -> Self {

        Self ( kilo_calories * 1_000)
    }


    pub fn as_cal(self) -> u64 {

        self.0
    }

    pub fn as_kcal(self) -> u64 {

        self.0 / 1_000
    }

}

impl Mul<u64> for Calories {
    type Output = Calories;

    fn mul(self, rhs: u64) -> Self::Output {

        Self ( self.0 * rhs)
    }
}

impl MulAssign<u64> for Calories {
    fn mul_assign(&mut self, rhs: u64) {

        self.0 *= rhs;
    }
}

impl Div<u64> for Calories {
    type Output = Calories;

    fn div(self, rhs: u64) -> Self::Output {

        Self ( self.0 / rhs)
    }
}

impl DivAssign<u64> for Calories {
    fn div_assign(&mut self, rhs: u64) {

        self.0 /= rhs;
    }
}



#[derive(Clone, Copy)]
pub struct CaloriesRate ( Calories);

impl CaloriesRate {

    pub fn from_x_per_kilo_grams(calories: Calories) -> Self {

        Self ( calories * 1_000)
    }

    pub fn from_x_per_tons(calories: Calories) -> Self {

        Self ( calories)
    }


    pub fn as_x_per_kilo_grams(self) -> Calories {

        self.0 / 1_000
    }

    pub fn as_x_per_tone(self) -> Calories {

        self.0
    }

}
