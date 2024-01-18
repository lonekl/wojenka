use std::ops::{Add, AddAssign, Rem, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Time ( i64);

impl Time {

    pub const YEAR: Time = Time::from_years(1);
    pub const HOUR: Time = Time::from_hours(1);

    pub const fn from_seconds(seconds: i64) -> Self {

        Self ( seconds)
    }

    pub const fn from_minutes(minutes: i64) -> Self {

        Self ( minutes * 60)
    }

    pub const fn from_hours(hours: i64) -> Self {

        Self ( hours * 60 * 60)
    }

    pub const fn from_days(days: i64) -> Self {

        Self ( days * 60 * 60 * 24)
    }

    pub const fn from_weeks(weeks: i64) -> Self {

        Self ( weeks * 60 * 60 * 24 * 7)
    }

    pub const fn from_months(months: i64) -> Self {

        Self ( months * 60 * 60 * 6 * (365 * 4 + 1) / 12)
    }

    pub const fn from_years(years: i64) -> Self {

        Self ( years * 60 * 60 * 6 * (365 * 4 + 1))
    }

}

impl Add<Time> for Time {
    type Output = Time;

    fn add(self, rhs: Time) -> Self::Output {

        Self ( self.0 + rhs.0)
    }
}

impl AddAssign<Time> for Time {
    fn add_assign(&mut self, rhs: Time) {
        self.0 += rhs.0;
    }
}

impl Sub<Time> for Time {
    type Output = Time;

    fn sub(self, rhs: Time) -> Self::Output {

        Self ( self.0 - rhs.0)
    }
}

impl SubAssign<Time> for Time {
    fn sub_assign(&mut self, rhs: Time) {
        self.0 -= rhs.0;
    }
}

impl Rem<Time> for Time {
    type Output = Time;

    fn rem(self, rhs: Time) -> Self::Output {

        Self ( self.0 % rhs.0)
    }
}