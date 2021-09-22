//! Units of measurement in recipes, that also hold the unit they were
//! originally entered in
use std::fmt;

/// A unit of mass like kg, pound, ounce, etc.
///
/// The value of the enum variant is the conversion factor to grams
#[derive(Clone, Copy, Debug, )]
pub enum MassUnit {
    Gram = 1.,
    Kilogram = 1000.,
    Milligram = 0.001,
    Ounce = 28.34952,
    Pound = 453.59237,
} 

impl fmt::Display for MassUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gram => write!(f, "gram"),
            Self::Kilogram => write!(f, "kilogram"),
            Self::Milligram => write!(f, "milligram"),
            Self::Ounce => write!(f, "ounce"),
            Self::Pound => write!(f, "pound")
        }
    }
}

impl MassUnit {
    /// Convert a measurement in a unit of `self` to a measurement in grams
    pub fn to_grams(&self, val: f32) -> f32 {
        val * ( (self as f64) as f32 )
    }
    
    /// Convert a measurement in grams to a measurement in `self` units
    pub fn from_grams(&self, val: f32) -> f32 {
        val / ( ( self as f64 ) as f32 )
    }
}

/// A measurement of mass with unit of measurement
#[derive(Clone, Debug, )]
pub struct Mass {

}


/// A volume of a substance, like cups, liters, etc.
///
/// The value of this enum as an f64 is the conversion factor from measurement to liters
#[derive(Clone, Copy, Debug, )]
pub enum VolumeUnit {
    Cup = 0.24,
    Liter = 1.,
    Milliliter = 0.001,
    Teaspoon = 0.00492892,
    Tablespoon = 0.014787,
    Pint = 0.473176,
    Quart = 0.946353,
    Gallon = 3.78541,
    FluidOz = 0.0295735,
}

impl VolumeUnit {
    /// Convert a measurement in `self` units to liters
    pub fn to_liters(&self, val: f32) -> f32 {
        val * ( (self as f64) as f32 )
    }
    
    /// Convert a measurement in liters to a measurement in units of 
    /// `self`
    pub fn from_liters(&self, val: f32) -> f32 {
        val / ( (self as f64) as f32)
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
#[derive(Clone, Debug, )]
pub struct Volume {
    /// The unit that this volume is measured in
    unit: VolumeUnit,
    /// The amount of the given unit
    val: f32,
}

impl Volume {
    #[inline(always)]
    pub const fn new(unit: VolumeUnit, val: f32) -> Self {
        Self {
            unit,
            val
        }
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
        write!("{} {}{}", self.val, self.unit, if self.val == 1.0 { "s" } else { "" })
    }
}

impl PartialOrd for Volume {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let my_liters = self.unit.to_liters(self.val);
        let their_liters = other.unit.to_liters(other.val);
        my_liters.partial_cmp(&their_liters)
    }
}
