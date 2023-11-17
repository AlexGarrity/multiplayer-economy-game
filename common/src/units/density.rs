use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};

use super::{Mass, UnitT, Volume};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub struct Density {
    pub mass: Mass,
    pub volume: Volume,
}

impl Density {
    pub fn new(mass: Mass, volume: Volume) -> Self {
        Self { mass, volume }
    }
}

impl Add for Density {
    type Output = Density;

    fn add(self, rhs: Self) -> Self::Output {
        Density {
            mass: self.mass + rhs.mass,
            volume: self.volume + rhs.volume,
        }
    }
}

impl Sub for Density {
    type Output = Density;

    fn sub(self, rhs: Self) -> Self::Output {
        Density {
            mass: self.mass - rhs.mass,
            volume: self.volume - rhs.volume,
        }
    }
}

impl Div<UnitT> for Density {
    type Output = Density;

    fn div(self, rhs: UnitT) -> Self::Output {
        Self {
            mass: self.mass,
            volume: Volume::from_microlitres(self.volume.as_microlitres() * rhs),
        }
    }
}

impl Div<Density> for Mass {
    type Output = Volume;

    fn div(self, rhs: Density) -> Self::Output {
        Volume::from_cubic_millimetres(rhs.volume.as_cubic_millimetres() * (self / rhs.mass))
    }
}

impl Mul<Volume> for Density {
    type Output = Mass;

    fn mul(self, rhs: Volume) -> Self::Output {
        Mass::from_milligrams(
            (self.mass.as_milligrams() * rhs.as_cubic_millimetres())
                / self.volume.as_cubic_millimetres(),
        )
    }
}

impl PartialOrd for Density {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Density {
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = self.mass.as_milligrams() / self.volume.as_cubic_millimetres();
        let d2 = other.mass.as_milligrams() / other.volume.as_cubic_millimetres();

        d1.cmp(&d2)
    }
}

impl Display for Density {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}mg/m^3",
            self.mass.as_milligrams() / self.volume.as_cubic_metres()
        ))
    }
}

impl Sum for Density {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = Density::default();

        for density in iter {
            total = total + density;
        }

        total
    }
}

impl<'a> Sum<&'a Density> for Density {
    fn sum<I: Iterator<Item = &'a Density>>(iter: I) -> Self {
        let mut total = Density::default();

        for density in iter {
            total = total + *density;
        }

        total
    }
}
