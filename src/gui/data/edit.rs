//! State for the currently edited recipe

use std::time;

use druid::{Data, Lens, im::Vector};
use serde::{Deserialize, Serialize};

use crate::recipes::recipe::IngredientAmount;

/// Data for the currently edited recipe
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct EditState {
    /// The name of the recipe
    pub title: String,
    /// Ingredients list
    pub ingredients: Vector<EditedIngredient>,
    /// Body of the recipe
    pub body: String,
    /// Number of servings the recipe makes
    pub servings: Option<f32>,
    /// The amount of time that the recipe is expected to take
    #[data(same_fn = "PartialEq::eq")]
    pub time: Option<time::Duration>
}

/// Ingredient data stored in a more efficiently mutable way
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct EditedIngredient {
    /// The name of the ingredient
    pub name: String,
    /// The amount of the ingredient needed
    #[data(same_fn = "PartialEq::eq")]
    pub amount: IngredientAmount
}