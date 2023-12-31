use super::UnitT;

pub const MICROJOULES: UnitT = 1;
pub const MILLIJOULES: UnitT = 1_000 * MICROJOULES;
pub const JOULES: UnitT = 1_000 * MILLIJOULES;
pub const KILOJOULES: UnitT = 1_000 * JOULES;
pub const MEGAJOULES: UnitT = 1_000 * KILOJOULES;

pub const MICROGRAM: UnitT = 1;
pub const MILLIGRAM: UnitT = 1_000 * MICROGRAM;
pub const GRAM: UnitT = 1_000 * MILLIGRAM;
pub const KILOGRAM: UnitT = 1_000 * GRAM;
pub const TONNE: UnitT = 1_000 * KILOGRAM;
pub const KILOTONNE: UnitT = 1_000 * TONNE;

pub const MILLIMETRE: UnitT = 1;
pub const CENTIMETRE: UnitT = 10 * MILLIMETRE;
pub const DECIMETRE: UnitT = 10 * CENTIMETRE;
pub const METRE: UnitT = 10 * DECIMETRE;
pub const KILOMETRE: UnitT = 1_000 * METRE;

pub const MILLIMETRE2: UnitT = MILLIMETRE * MILLIMETRE;
pub const CENTIMETRE2: UnitT = CENTIMETRE * CENTIMETRE;
pub const DECIMETRE2: UnitT = DECIMETRE * DECIMETRE;
pub const METRE2: UnitT = METRE * METRE;
pub const KILOMETRE2: UnitT = KILOMETRE * KILOMETRE;

pub const MILLIMETRE3: UnitT = MILLIMETRE * MILLIMETRE * MILLIMETRE;
pub const CENTIMETRE3: UnitT = CENTIMETRE * CENTIMETRE * CENTIMETRE;
pub const DECIMETRE3: UnitT = DECIMETRE * DECIMETRE * DECIMETRE;
pub const METRE3: UnitT = METRE * METRE * METRE;
pub const KILOMETRE3: UnitT = KILOMETRE * KILOMETRE * KILOMETRE;

pub const MICROLITRE: UnitT = MILLIMETRE3;
pub const MILLILITRE: UnitT = CENTIMETRE3;
pub const CENTILITRE: UnitT = 10 * CENTIMETRE3;
pub const DECILITRE: UnitT = 100 * CENTIMETRE3;
pub const LITRE: UnitT = DECIMETRE3;

pub const KELVIN: UnitT = 0;
pub const CELCIUS: UnitT = KELVIN + 273;
