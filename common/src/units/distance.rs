use super::{Area, UnitT, Volume, CENTIMETRE, DECIMETRE, KILOMETRE, METRE, MILLIMETRE};
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Distance(UnitT);

impl Distance {
    pub fn from_kilometres(value: UnitT) -> Self {
        Self(value * KILOMETRE)
    }

    pub fn from_metres(value: UnitT) -> Self {
        Self(value * METRE)
    }

    pub fn from_decimetres(value: UnitT) -> Self {
        Self(value * DECIMETRE)
    }

    pub fn from_centimetres(value: UnitT) -> Self {
        Self(value * CENTIMETRE)
    }

    pub fn from_millimetres(value: UnitT) -> Self {
        Self(value * MILLIMETRE)
    }

    pub fn as_kilometres(&self) -> UnitT {
        self.0 / KILOMETRE
    }

    pub fn as_metres(&self) -> UnitT {
        self.0 / METRE
    }

    pub fn as_decimetres(&self) -> UnitT {
        self.0 / DECIMETRE
    }

    pub fn as_centimetres(&self) -> UnitT {
        self.0 / CENTIMETRE
    }

    pub fn as_millimetres(&self) -> UnitT {
        self.0 / MILLIMETRE
    }
}

impl Add for Distance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Distance {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<Distance> for Distance {
    type Output = Area;

    fn mul(self, rhs: Self) -> Self::Output {
        Area::from_square_millimetres(self.as_millimetres() * rhs.as_millimetres())
    }
}

impl Mul<Area> for Distance {
    type Output = Volume;

    fn mul(self, rhs: Area) -> Self::Output {
        Volume::from_cubic_millimetres(self.as_millimetres() * rhs.as_square_millimetres())
    }
}

impl Mul<UnitT> for Distance {
    type Output = Distance;

    fn mul(self, rhs: UnitT) -> Self::Output {
        Distance(self.0 * rhs)
    }
}

impl Div for Distance {
    type Output = UnitT;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Div<UnitT> for Distance {
    type Output = Distance;

    fn div(self, rhs: UnitT) -> Self::Output {
        Distance(self.0 / rhs)
    }
}
