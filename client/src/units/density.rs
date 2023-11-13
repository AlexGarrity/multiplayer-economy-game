use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};

use super::{ Mass, Volume };

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Density {
    pub mass: Mass,
    pub volume: Volume
}

impl Density {
    pub fn new(mass: Mass, volume: Volume) -> Self {
        Self {
            mass,
            volume
        }
    }
}

impl Add for Density {
    type Output = Density;

    fn add(self, rhs: Self) -> Self::Output {
        Density {
            mass: self.mass + rhs.mass,
            volume: self.volume + rhs.volume
        }
    }
}

impl Sub for Density {
    type Output = Density;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Density {
            mass: self.mass - rhs.mass,
            volume: self.volume - rhs.volume
        }
    }
}

impl Div<Density> for Mass {
    type Output = Volume;

    fn div(self, rhs: Density) -> Self::Output {
        Volume::from_cubic_millimetres(
            rhs.volume.as_cubic_millimetres() * (self / rhs.mass)
        )
    }
}

impl Mul<Volume> for Density {
    type Output = Mass;

    fn mul(self, rhs: Volume) -> Self::Output {
        Mass::from_milligrams(
            (self.mass.as_milligrams() * rhs.as_cubic_millimetres()) / self.volume.as_cubic_millimetres()
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