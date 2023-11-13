use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};

use super::{Area, Distance, UnitT};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Volume(UnitT);

impl Volume {
    pub const fn from_cubic_kilometres(value: UnitT) -> Self {
        Self(value * 1_000_000_000_000_000_000)
    }

    pub const fn from_cubic_metres(value: UnitT) -> Self {
        Self(value * 1_000_000_000)
    }

    pub const fn from_cubic_decimetres(value: UnitT) -> Self {
        Self(value * 1_000_000)
    }

    pub const fn from_cubic_centimetres(value: UnitT) -> Self {
        Self(value * 1_000)
    }

    pub const fn from_cubic_millimetres(value: UnitT) -> Self {
        Self(value)
    }

    pub const fn from_litres(value: UnitT) -> Self {
        Self(value * 1_000_000)
    }

    pub const fn from_decilitres(value: UnitT) -> Self {
        Self(value * 100_000)
    }

    pub const fn from_centilitres(value: UnitT) -> Self {
        Self(value * 10_000)
    }

    pub const fn from_millilitres(value: UnitT) -> Self {
        Self(value * 1_000)
    }

    pub const fn from_microlitres(value: UnitT) -> Self {
        Self(value)
    }


    pub const fn as_cubic_kilometres(&self) -> UnitT {
        self.0 / 1_000_000_000_000_000_000
    }

    pub const fn as_cubic_metres(&self) -> UnitT {
        self.0 / 1_000_000_000
    }

    pub const fn as_cubic_decimetres(&self) -> UnitT {
        self.0 / 1_000_000
    }

    pub const fn as_cubic_centimetres(&self) -> UnitT {
        self.0 / 1_000
    }

    pub const fn as_cubic_millimetres(&self) -> UnitT {
        self.0
    }

    pub const fn as_litres(&self) -> UnitT {
        self.0 / 1_000_000
    }

    pub const fn as_decilitres(&self) -> UnitT {
        self.0 / 100_000
    }

    pub const fn as_centilitres(&self) -> UnitT {
        self.0 / 10_000
    }

    pub const fn as_millilitres(&self) -> UnitT {
        self.0 / 1_000
    }

    pub const fn as_microlitres(&self) -> UnitT {
        self.0
    }
}

impl Add for Volume {
    type Output = Volume;

    fn add(self, rhs: Self) -> Self::Output {
        Volume(self.0 + rhs.0)
    }
}

impl AddAssign for Volume {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Volume {
    type Output = Volume;

    fn sub(self, rhs: Self) -> Self::Output {
        Volume(self.0 - rhs.0)
    }
}

impl SubAssign for Volume {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul<UnitT> for Volume {
    type Output = Volume;

    fn mul(self, rhs: UnitT) -> Self::Output {
        Volume(self.0 * rhs)
    }
}

impl Div<Distance> for Volume {
    type Output = Area;

    fn div(self, rhs: Distance) -> Self::Output {
        Area::from_square_millimetres(self.as_cubic_millimetres() / rhs.as_millimetres())
    }
}

impl Div<Area> for Volume {
    type Output = Distance;

    fn div(self, rhs: Area) -> Self::Output {
        Distance::from_millimetres(self.as_cubic_millimetres() / rhs.as_square_millimetres())
    }
}

impl Div<UnitT> for Volume {
    type Output = Volume;

    fn div(self, rhs: UnitT) -> Self::Output {
        Volume(self.0 / rhs)
    }
}