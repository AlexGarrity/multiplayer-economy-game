use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use super::{
    Area, Distance, UnitT, CENTILITRE, CENTIMETRE3, DECILITRE, DECIMETRE3, KILOMETRE3, LITRE,
    METRE3, MICROLITRE, MILLILITRE, MILLIMETRE3,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
pub struct Volume(UnitT);

impl Volume {
    pub const fn from_cubic_kilometres(value: UnitT) -> Self {
        Self(value * KILOMETRE3)
    }

    pub const fn from_cubic_metres(value: UnitT) -> Self {
        Self(value * METRE3)
    }

    pub const fn from_cubic_decimetres(value: UnitT) -> Self {
        Self(value * DECIMETRE3)
    }

    pub const fn from_cubic_centimetres(value: UnitT) -> Self {
        Self(value * CENTIMETRE3)
    }

    pub const fn from_cubic_millimetres(value: UnitT) -> Self {
        Self(value * MILLIMETRE3)
    }

    pub const fn from_litres(value: UnitT) -> Self {
        Self(value * LITRE)
    }

    pub const fn from_decilitres(value: UnitT) -> Self {
        Self(value * DECILITRE)
    }

    pub const fn from_centilitres(value: UnitT) -> Self {
        Self(value * CENTILITRE)
    }

    pub const fn from_millilitres(value: UnitT) -> Self {
        Self(value * MILLILITRE)
    }

    pub const fn from_microlitres(value: UnitT) -> Self {
        Self(value * MICROLITRE)
    }

    pub const fn as_cubic_kilometres(&self) -> UnitT {
        self.0 / KILOMETRE3
    }

    pub const fn as_cubic_metres(&self) -> UnitT {
        self.0 / METRE3
    }

    pub const fn as_cubic_decimetres(&self) -> UnitT {
        self.0 / DECIMETRE3
    }

    pub const fn as_cubic_centimetres(&self) -> UnitT {
        self.0 / CENTIMETRE3
    }

    pub const fn as_cubic_millimetres(&self) -> UnitT {
        self.0 / MILLIMETRE3
    }

    pub const fn as_litres(&self) -> UnitT {
        self.0 / LITRE
    }

    pub const fn as_decilitres(&self) -> UnitT {
        self.0 / DECILITRE
    }

    pub const fn as_centilitres(&self) -> UnitT {
        self.0 / CENTILITRE
    }

    pub const fn as_millilitres(&self) -> UnitT {
        self.0 / MILLILITRE
    }

    pub const fn as_microlitres(&self) -> UnitT {
        self.0 / MICROLITRE
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

impl Div for Volume {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 as f32 / rhs.0 as f32
    }
}
