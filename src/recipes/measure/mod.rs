//! Units of measurement in recipes, that also hold the unit they were
//! originally entered in
use serde::{Deserialize, Serialize};
use std::fmt;

use super::recipe::IngredientAmount;

/// Units of time a user can pick
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Second => write!(f, "seconds"),
            Self::Minute => write!(f, "minutes"),
            Self::Hour => write!(f, "hours"),
            Self::Day => write!(f, "days"),
        }
    }
}

/// A unit of mass like kg, pound, ounce, etc.
///
/// The value of the enum variant is the conversion factor to grams
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, druid::Data)]
pub enum MassUnit {
    Gram,
    Kilogram,
    Milligram,
    Ounce,
    Pound,
}

impl fmt::Display for MassUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gram => write!(f, "gram"),
            Self::Kilogram => write!(f, "kilogram"),
            Self::Milligram => write!(f, "milligram"),
            Self::Ounce => write!(f, "ounce"),
            Self::Pound => write!(f, "pound"),
        }
    }
}

impl MassUnit {
    /// Get the conversion factor for this unit to grams
    pub const fn conversion_factor(&self) -> f32 {
        match self {
            Self::Gram => 1.,
            Self::Kilogram => 1000.,
            Self::Milligram => 0.001,
            Self::Ounce => 28.34952,
            Self::Pound => 453.59237,
        }
    }

    /// Convert a measurement in a unit of `self` to a measurement in grams
    pub fn to_grams(&self, val: f32) -> f32 {
        val * self.conversion_factor()
    }

    /// Convert a measurement in grams to a measurement in `self` units
    pub fn from_grams(&self, val: f32) -> f32 {
        val / self.conversion_factor()
    }
}

/// A measurement of mass with unit of measurement
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq,)]
pub struct Mass {
    pub unit: MassUnit,
    pub val: f32,
}

impl Mass {
    /// Create a new mass with the given unit of measure and value
    pub fn new(unit: MassUnit, val: f32) -> Self {
        Self { unit, val }
    }

    /// Convert this measure's unit of mass to another unit
    pub fn convert(&self, unit: MassUnit) -> Self {
        Self {
            val: unit.from_grams(self.unit.to_grams(self.val)),
            unit,
        }
    }
}

impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}{}",
            self.val,
            self.unit,
            if self.val == 1.0 { "s" } else { "" }
        )
    }
}

/// A volume of a substance, like cups, liters, etc.
///
/// The value of this enum as an f64 is the conversion factor from measurement to liters
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, druid::Data)]
pub enum VolumeUnit {
    Cup,
    Liter,
    Milliliter,
    Teaspoon,
    Tablespoon,
    Pint,
    Quart,
    Gallon,
    FluidOz,
}

impl VolumeUnit {
    /// Get the conversion factor to liters for this unit
    pub const fn conversion_factor(&self) -> f32 {
        match self {
            Self::Cup => 0.24,
            Self::Liter => 1.,
            Self::Milliliter => 0.001,
            Self::Teaspoon => 0.00492892,
            Self::Tablespoon => 0.014787,
            Self::Pint => 0.473176,
            Self::Quart => 0.946353,
            Self::Gallon => 3.78541,
            Self::FluidOz => 0.0295735,
        }
    }
    /// Convert a measurement in `self` units to liters
    pub fn to_liters(&self, val: f32) -> f32 {
        val * self.conversion_factor()
    }

    /// Convert a measurement in liters to a measurement in units of
    /// `self`
    pub fn from_liters(&self, val: f32) -> f32 {
        val / self.conversion_factor()
    }
}

impl fmt::Display for VolumeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cup => write!(f, "cup"),
            Self::Liter => write!(f, "liter"),
            Self::Milliliter => write!(f, "milliliter"),
            Self::Teaspoon => write!(f, "teaspoon"),
            Self::Tablespoon => write!(f, "tablespoon"),
            Self::Pint => write!(f, "pint"),
            Self::Quart => write!(f, "quart"),
            Self::Gallon => write!(f, "gallon"),
            Self::FluidOz => write!(f, "fluid ounce"),
        }
    }
}

/// An amount of a substance with a given unit of measurement
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Volume {
    /// The unit that this volume is measured in
    pub unit: VolumeUnit,
    /// The amount of the given unit
    pub val: f32,
}

impl Volume {
    #[inline(always)]
    pub const fn new(unit: VolumeUnit, val: f32) -> Self {
        Self { unit, val }
    }

    /// Convert this volume measurement into a measurement of the given unit
    pub fn convert(&self, unit: VolumeUnit) -> Self {
        Self {
            val: unit.from_liters(self.unit.to_liters(self.val)),
            unit,
        }
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}{}",
            self.val,
            self.unit,
            if self.val == 1.0 { "" } else { "s" }
        )
    }
}

/// Enumeration for how an ingredient's amount's unit is stored
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, druid::Data)]
pub enum AmountUnit {
    /// A raw number, displayed as x{n}
    Count,
    /// A measurement of volume in cups, liters, etc.
    Volume(VolumeUnit),
    /// A measurement in mass
    Mass(MassUnit),
    /// No amount given
    None,
}

impl From<IngredientAmount> for AmountUnit {
    fn from(amt: IngredientAmount) -> Self {
        match amt {
            IngredientAmount::Count(_) => Self::Count,
            IngredientAmount::Volume(Volume { unit, val: _ }) => Self::Volume(unit),
            IngredientAmount::Mass(Mass { unit, val: _ }) => Self::Mass(unit),
            IngredientAmount::None => Self::None,
        }
    }
}

impl fmt::Display for AmountUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Count => write!(f, "count"),
            Self::Volume(vol) => vol.fmt(f),
            Self::Mass(mass) => mass.fmt(f),
            Self::None => write!(f, "no measure"),
        }
    }
}
