//! State for the currently edited recipe

use std::{ops::Deref, time};

use druid::{Data, Lens, im::Vector};
use serde::{Deserialize, Serialize};

use crate::recipes::{db::RecipeId, recipe::{Ingredient, IngredientAmount, Recipe}};

/// Data for the currently edited recipe
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize, Default)]
pub struct EditState {
    /// If we are modifying the recipe, this is the ID of the modified recipe
    pub id: Option<RecipeId>,
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

impl From<&Ingredient> for EditedIngredient {
    fn from(ingredient: &Ingredient) -> Self {
        Self {
            name: ingredient.name.deref().to_owned(),
            amount: ingredient.amount
        }
    }
}

impl From<&Recipe> for EditState {
    fn from(recipe: &Recipe) -> Self {
        Self {
            id: Some(recipe.id),
            title: recipe.name.deref().to_owned(),
            ingredients: recipe.ingredients.iter().map(EditedIngredient::from).collect(),
            body: recipe.body.deref().to_owned(),
            servings: recipe.servings,
            time: recipe.time,
        }
    }
}