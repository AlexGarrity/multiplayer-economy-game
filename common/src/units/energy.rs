use std::ops::{Add, Div, Sub};

use super::{
    factors::{JOULES, KILOJOULES, MEGAJOULES, MICROJOULES, MILLIJOULES},
    UnitT,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Energy(UnitT);

impl Energy {
    pub fn from_microjoules(value: UnitT) -> Self {
        Self(value * MICROJOULES)
    }

    pub fn from_millijoules(value: UnitT) -> Self {
        Self(value * MILLIJOULES)
    }

    pub fn from_joules(value: UnitT) -> Self {
        Self(value * JOULES)
    }

    pub fn from_kilojoules(value: UnitT) -> Self {
        Self(value * KILOJOULES)
    }

    pub fn from_megajoules(value: UnitT) -> Self {
        Self(value * MEGAJOULES)
    }

    pub fn as_microjoules(&self) -> UnitT {
        self.0 / MICROJOULES
    }

    pub fn as_millijoules(&self) -> UnitT {
        self.0 / MILLIJOULES
    }

    pub fn as_joules(&self) -> UnitT {
        self.0 / JOULES
    }

    pub fn as_kilojoules(&self) -> UnitT {
        self.0 / KILOJOULES
    }

    pub fn as_megajoules(&self) -> UnitT {
        self.0 / MEGAJOULES
    }
}

impl Add for Energy {
    type Output = Energy;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Energy {
    type Output = Energy;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Div for Energy {
    type Output = UnitT;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}
