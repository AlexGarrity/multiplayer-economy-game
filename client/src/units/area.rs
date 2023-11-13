use std::ops::{Add, Div, Mul, Sub};

use super::{Distance, UnitT, Volume, CENTIMETRE2, DECIMETRE2, KILOMETRE2, METRE2, MILLIMETRE2};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Area(UnitT);

impl Area {
    pub const fn from_square_kilometres(value: UnitT) -> Self {
        Self(value * KILOMETRE2)
    }

    pub const fn from_square_metres(value: UnitT) -> Self {
        Self(value * METRE2)
    }

    pub const fn from_square_decimetres(value: UnitT) -> Self {
        Self(value * DECIMETRE2)
    }

    pub const fn from_square_centimetres(value: UnitT) -> Self {
        Self(value * CENTIMETRE2)
    }

    pub const fn from_square_millimetres(value: UnitT) -> Self {
        Self(value * MILLIMETRE2)
    }

    pub const fn as_square_kilometres(&self) -> UnitT {
        self.0 / KILOMETRE2
    }

    pub const fn as_square_metres(&self) -> UnitT {
        self.0 / METRE2
    }

    pub const fn as_square_decimetres(&self) -> UnitT {
        self.0 / DECIMETRE2
    }

    pub const fn as_square_centimetres(&self) -> UnitT {
        self.0 / CENTIMETRE2
    }

    pub const fn as_square_millimetres(&self) -> UnitT {
        self.0 / MILLIMETRE2
    }
}

impl Add for Area {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        Area(self.0 + rhs.0)
    }
}

impl Sub for Area {
    type Output = Area;

    fn sub(self, rhs: Self) -> Self::Output {
        Area(self.0 - rhs.0)
    }
}

impl Mul<Distance> for Area {
    type Output = Volume;

    fn mul(self, rhs: Distance) -> Self::Output {
        Volume::from_cubic_millimetres(self.as_square_millimetres() * rhs.as_millimetres())
    }
}

impl Mul<UnitT> for Area {
    type Output = Area;

    fn mul(self, rhs: UnitT) -> Self::Output {
        Area(self.0 * rhs)
    }
}

impl Div<Distance> for Area {
    type Output = Distance;

    fn div(self, rhs: Distance) -> Self::Output {
        Distance::from_millimetres(self.as_square_millimetres() / rhs.as_millimetres())
    }
}

impl Div<UnitT> for Area {
    type Output = Area;

    fn div(self, rhs: UnitT) -> Self::Output {
        Area(self.0 / rhs)
    }
}
