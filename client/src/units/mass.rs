use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use super::{UnitT, GRAM, KILOGRAM, KILOTONNE, MILLIGRAM, TONNE};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Mass(UnitT);
impl Mass {
    pub const fn from_kilotonnes(value: UnitT) -> Self {
        Self(value * KILOTONNE)
    }

    pub const fn from_tonnes(value: UnitT) -> Self {
        Self(value * TONNE)
    }

    pub const fn from_kilograms(value: UnitT) -> Self {
        Self(value * KILOGRAM)
    }

    pub const fn from_grams(value: UnitT) -> Self {
        Self(value * GRAM)
    }

    pub const fn from_milligrams(value: UnitT) -> Self {
        Self(value * MILLIGRAM)
    }

    pub const fn as_kilotonnes(&self) -> UnitT {
        self.0 / KILOTONNE
    }

    pub const fn as_tonnes(&self) -> UnitT {
        self.0 / TONNE
    }

    pub const fn as_kilograms(&self) -> UnitT {
        self.0 / KILOGRAM
    }

    pub const fn as_grams(&self) -> UnitT {
        self.0 / GRAM
    }

    pub const fn as_milligrams(&self) -> UnitT {
        self.0 / MILLIGRAM
    }
}

impl Add for Mass {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Mass(self.0 + rhs.0)
    }
}

impl AddAssign for Mass {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Mass {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Mass(self.0 - rhs.0)
    }
}

impl SubAssign for Mass {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul<UnitT> for Mass {
    type Output = Mass;

    fn mul(self, rhs: UnitT) -> Self::Output {
        Mass(self.0 * rhs)
    }
}

impl Div for Mass {
    type Output = UnitT;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Div<UnitT> for Mass {
    type Output = Mass;

    fn div(self, rhs: UnitT) -> Self::Output {
        Mass(self.0 / rhs)
    }
}
