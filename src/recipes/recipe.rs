//! Structures holding recipe data
use chrono::{DateTime, Local, TimeZone, Utc};
use druid::im::{vector, Vector};
use serde::{Deserialize, Serialize};
use std::{fmt, sync::Arc, time};

use super::{
    db::RecipeId,
    measure::{Mass, Volume},
};

/// One ingredient in a recipe, with amount of the ingredient and ingredient name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, druid::Lens, druid::Data)]
pub struct Ingredient {
    pub name: Arc<str>,
    #[data(ignore)]
    pub amount: IngredientAmount,
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.amount)
    }
}

/// Enumeration for how an ingredient's amount is displayed
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
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
            Self::Mass(Mass { val: _, unit }) => unit.to_string(),
            Self::Volume(Volume { val: _, unit }) => unit.to_string(),
            Self::None => "no unit".to_owned(),
        }
    }
}

impl fmt::Display for IngredientAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Count(amt) => write!(f, "x{}", amt),
            Self::Volume(vol) => vol.fmt(f),
            Self::Mass(mass) => mass.fmt(f),
            Self::None => Ok(()),
        }
    }
}

/// Immutable struct containing all data a user can add to a recipe
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, druid::Lens)]
pub struct Recipe {
    /// The internal ID of this recipe
    pub id: RecipeId,
    /// The name of the recipe
    pub name: Arc<str>,
    /// When this recipe was created
    #[serde(default = "chrono::Utc::now")]
    pub created_on: DateTime<Utc>,
    /// How many servings a recipe makes
    pub servings: Option<f32>,
    /// A list of ingredients in the recipe
    pub ingredients: Vector<Ingredient>,
    /// The recipe's instructions
    pub body: Arc<str>,
    /// The time that the recipe takes to make
    pub time: Option<time::Duration>,
}

impl Recipe {
    /// Return a `Recipe` for top ramen
    pub fn top_ramen() -> Self {
        Self {
            id: RecipeId::new(),
            name: "Top Ramen".into(),
            created_on: DateTime::from(Local.ymd(2021, 11, 10).and_hms(16, 7, 0)),
            servings: Some(2.),
            ingredients: vector![
                Ingredient {
                    name: "Top Ramen Packet".into(),
                    amount: IngredientAmount::Count(1),
                },
                Ingredient {
                    name: "Water".into(),
                    amount: IngredientAmount::Volume(Volume::new(
                        super::measure::VolumeUnit::Cup,
                        2.,
                    )),
                },
            ],
            body: r#"- Add water to small / medium pot and bring to boil
- Remove noodle brick from packet and add to water
- Allow noodles to cook for around 3 minutes, stirring occasionally
- Remove heat and add flavor packet to noodles, ensuring that flavor spreads to noodles by stirring
- Leave for 5-10 minutes to cool and enjoy"#
                .into(),
            time: Some(time::Duration::from_secs(600)),
        }
    }
}
