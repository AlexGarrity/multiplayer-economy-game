use std::ops::{Add, Sub};

use super::UnitT;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Temperature(UnitT);

impl Temperature {
    pub fn from_kelvin(value: UnitT) -> Self
    {
        Self(value)
    }

    pub fn from_celcius(value: UnitT) -> Self {
        Self(value + 273)
    }

    pub fn as_kelvin(&self) -> UnitT {
        self.0
    }

    pub fn as_celcius(&self) -> UnitT {
        self.0 + 273
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