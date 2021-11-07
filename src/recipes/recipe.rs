//! Structures holding recipe data
use std::{fmt, time};
use serde::{Serialize, Deserialize};

use super::measure::{Mass, Volume};

/// One ingredient in a recipe, with amount of the ingredient and ingredient name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub amount: IngredientAmount,
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.amount)
    }
}

/// Enumeration for how an ingredient's amount is displayed
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum IngredientAmount {
    /// A raw number, displayed as x{n}
    Count(usize),
    /// A measurement of volume in cups, liters, etc.
    Volume(Volume),
    /// A measurement in mass
    Mass(Mass),
    /// No amount given
    None,
}   

impl IngredientAmount {
    pub fn unit_string(&self) -> String {
        match self {
            Self::Count(_) => "count".to_owned(),
            Self::Mass(Mass { val: _, unit} ) => unit.to_string(),
            Self::Volume(Volume { val: _, unit} ) => unit.to_string(),
            Self::None => "no unit".to_owned()
        }
    }
}

impl fmt::Display for IngredientAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Count(amt) => write!(f, "x{}", amt),
            Self::Volume(vol) => vol.fmt(f),
            Self::Mass(mass) => mass.fmt(f),
            Self::None => Ok(())
        }
    }
}

/// Struct containing all data a user can add to a recipe
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    /// The name of the recipe
    pub name: String,
    /// How many servings a recipe makes
    pub servings: f32,
    /// A list of ingredients in the recipe
    pub ingredients: Vec<Ingredient>,
    /// The recipe's instructions
    pub body: String,
    /// The time that the recipe takes to make
    pub time: Option<time::Duration>,
}

impl Recipe {
    /// Return a `Recipe` for top ramen
    pub fn top_ramen() -> Self {
        Self {
            name: "Top Ramen".to_owned(),
            servings: 2.,
            ingredients: vec![
                Ingredient {
                    name: "Top Ramen Packet".to_owned(),
                    amount: IngredientAmount::Count(1)
                },
                Ingredient {
                    name: "Water".to_owned(),
                    amount: IngredientAmount::Volume(Volume::new(super::measure::VolumeUnit::Cup, 2.))
                },
            ],
            body: 
    "- Add water to small / medium pot and bring to boil
    - Remove noodle brick from packet and add to water
    - Allow noodles to cook for around 3 minutes, stirring occasionally
    - Remove heat and add flavor packet to noodles, ensuring that flavor spreads to noodles by stirring
    - Leave for 5-10 minutes to cool and enjoy
    ".to_owned(),
                time: Some(std::time::Duration::from_secs(600))
        }
    }
}

impl Default for Recipe {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            servings: 0.,
            ingredients: vec![],
            body: "".to_owned(),
            time: None
        }
    }
}
