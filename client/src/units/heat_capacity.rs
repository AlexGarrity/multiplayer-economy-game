use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt::Display;
use std::ops::{Add, Sub};

use super::{Energy, Mass};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct HeatCapacity {
    pub energy: Energy,
    pub mass: Mass,
}

impl Add for HeatCapacity {
    type Output = HeatCapacity;

    fn add(self, rhs: Self) -> Self::Output {
        HeatCapacity {
            energy: self.energy + rhs.energy,
            mass: self.mass + rhs.mass,
        }
    }
}

impl Sub for HeatCapacity {
    type Output = HeatCapacity;

    fn sub(self, rhs: Self) -> Self::Output {
        HeatCapacity {
            energy: self.energy - rhs.energy,
            mass: self.mass - rhs.mass,
        }
    }
}

impl PartialOrd for HeatCapacity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeatCapacity {
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = self.energy.as_microjoules() / self.mass.as_milligrams();
        let d2 = other.energy.as_microjoules() / other.mass.as_milligrams();

        d1.cmp(&d2)
    }
}

impl Display for HeatCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}J/kgK",
            self.energy.as_joules() / self.mass.as_kilograms()
        ))
    }
}
