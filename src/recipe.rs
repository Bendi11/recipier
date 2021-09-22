//! Structures holding recipe data
use std::fmt;
use crate::measure::{Volume};

/// One ingredient in a recipe, with amount of the ingredient and ingredient name
#[derive(Clone, Debug, )]
pub struct Ingredient {
    pub name: String,
    pub amount: IngredientAmount,
}

/// Enumeration for how an ingredient's amount is displayed
pub enum IngredientAmount {
    /// A raw number, displayed as x{n}
    Count(usize),
    /// A measurement of volume in cups, liters, etc.
    Volume(Volume),
    /// A measurement in mass
    Mass(),
    /// No amount given
    None,
}   

impl fmt::Display for IngredientAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result<()> {
        match self {
            Self::Count(amt) => write!(f, "x{}", amt),
            Self::Volume(vol) => write!(f, '')
        }
    }
}

/// Struct containing all data a user can add to a recipe
pub struct Recipe {
    pub name: String,
    pub ingredients: Option<Vec<String>>,
}
