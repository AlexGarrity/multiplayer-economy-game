use std::ops::{Add, Sub};

use super::{
    factors::{CELCIUS, KELVIN},
    UnitT,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Temperature(UnitT);

impl Temperature {
    pub fn from_kelvin(value: UnitT) -> Self {
        Self(value + KELVIN)
    }

    pub fn from_celcius(value: UnitT) -> Self {
        Self(value + CELCIUS)
    }

    pub fn as_kelvin(&self) -> UnitT {
        self.0 - KELVIN
    }

    pub fn as_celcius(&self) -> UnitT {
        self.0 - CELCIUS
    }
}

impl Add for Temperature {
    type Output = Temperature;

    fn add(self, rhs: Self) -> Self::Output {
        Temperature(self.0 + rhs.0)
    }
}

impl Sub for Temperature {
    type Output = Temperature;

    fn sub(self, rhs: Self) -> Self::Output {
        Temperature(self.0 - rhs.0)
    }
}
